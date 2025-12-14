// Sapphillon-Core
// Copyright 2025 Yuta Takahashi
//
// This file is part of Sapphillon-Core
//
// Sapphillon-Core is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//

//! JavaScript execution with Deno's MainWorker

use anyhow::{anyhow, bail, Result};
use deno_runtime::deno_core::PollEventLoopOptions;
use deno_runtime::deno_core::v8;

use crate::worker::create_main_worker;

/// Executes JavaScript code using Deno's MainWorker.
///
/// This provides access to Deno's built-in APIs like `console`, `fetch`,
/// filesystem operations, etc. Handles the full worker lifecycle including
/// dispatching load/beforeunload/unload events.
///
/// # Arguments
/// * `script` - The JavaScript code to execute
///
/// # Returns
/// * `Ok(exit_code)` on successful execution
/// * `Err(...)` if the script fails to execute
///
/// # Example
/// ```rust,ignore
/// let exit_code = run_js("console.log('Hello from Deno!')").await?;
/// ```
pub async fn run_js(script: &str) -> Result<i32> {
    let mut worker = create_main_worker()?;

    // Execute the script
    worker.execute_script("[ext_plugin]", script.to_string().into())?;

    // Dispatch load event
    worker.dispatch_load_event()?;

    // Run event loop
    loop {
        worker.run_event_loop(false).await?;

        let web_continue = worker.dispatch_beforeunload_event()?;
        if !web_continue {
            break;
        }
    }

    // Dispatch unload event
    worker.dispatch_unload_event()?;

    Ok(worker.exit_code())
}

/// Executes a JavaScript function `(string) -> string` with the provided string argument.
///
/// `function_source` must be a JavaScript expression that evaluates to a function.
/// For example: `(s) => s.toUpperCase()`
pub async fn run_js_with_string_arg(function_source: &str, arg: &str) -> Result<String> {
    let mut worker = create_main_worker()?;

    // Evaluate the function expression and get the resulting function value.
    let function_value = worker.execute_script(
        "[ext_plugin]",
        format!("({function_source})").into(),
    )?;

    // Dispatch load event (Deno lifecycle)
    worker.dispatch_load_event()?;

    // Convert evaluated value to a Function handle.
    let function_global = {
        deno_runtime::deno_core::scope!(scope, &mut worker.js_runtime);
        v8::tc_scope!(tc_scope, scope);
        let local_value = v8::Local::new(tc_scope, function_value);
        if !local_value.is_function() {
            bail!("function_source did not evaluate to a function");
        }
        let local_fn: v8::Local<v8::Function> = local_value
            .try_into()
            .map_err(|_| anyhow!("failed to convert JS value to Function"))?;
        v8::Global::new(tc_scope, local_fn)
    };

    // Create the argument value.
    let arg_global = {
        deno_runtime::deno_core::scope!(scope, &mut worker.js_runtime);
        v8::tc_scope!(tc_scope, scope);
        let v8_str = v8::String::new(tc_scope, arg)
            .ok_or_else(|| anyhow!("failed to allocate V8 string"))?;
        let arg_value: v8::Local<v8::Value> = v8_str.into();
        v8::Global::<v8::Value>::new(tc_scope, arg_value)
    };

    // Call the function and drive the event loop until it resolves (supports Promise return).
    let call_fut = worker
        .js_runtime
        .call_with_args(&function_global, &[arg_global]);
    let result_value = worker
        .js_runtime
        .with_event_loop_promise(Box::pin(call_fut), PollEventLoopOptions::default())
        .await?;

    // Convert result to Rust String.
    let result_string = {
        deno_runtime::deno_core::scope!(scope, &mut worker.js_runtime);
        v8::tc_scope!(tc_scope, scope);
        let local_value = v8::Local::new(tc_scope, result_value);
        if local_value.is_null_or_undefined() {
            bail!("JS function returned null/undefined");
        }
        // Accept both primitive string and String object.
        if !(local_value.is_string() || local_value.is_string_object()) {
            bail!("JS function returned a non-string value");
        }
        let v8_str = local_value
            .to_string(tc_scope)
            .ok_or_else(|| anyhow!("failed to convert JS value to string"))?;
        v8_str.to_rust_string_lossy(tc_scope)
    };

    // Drain and unload like `run_js`.
    loop {
        worker.run_event_loop(false).await?;
        let web_continue = worker.dispatch_beforeunload_event()?;
        if !web_continue {
            break;
        }
    }
    worker.dispatch_unload_event()?;

    Ok(result_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_run_js_console_log() {
        let result = run_js("console.log('Hello from Deno MainWorker!')").await;
        assert!(result.is_ok(), "Should be able to run console.log");
        assert_eq!(result.unwrap(), 0, "Exit code should be 0");
    }

    #[tokio::test]
    async fn test_run_js_simple_calculation() {
        let result = run_js("const x = 1 + 1; console.log('1 + 1 =', x);").await;
        assert!(result.is_ok(), "Should be able to run simple calculations");
        assert_eq!(result.unwrap(), 0, "Exit code should be 0");
    }

    #[tokio::test]
    async fn test_run_js_fetch() {
        let result = run_js(
            r#"
            (async () => {
                const response = await fetch('https://httpbin.org/get');
                console.log('Fetch status:', response.status);
                const data = await response.json();
                console.log('Fetch origin:', data.origin);
            })();
            "#,
        )
        .await;
        assert!(
            result.is_ok(),
            "Should be able to run fetch: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    async fn test_run_js_with_string_arg_sync() {
        let result = run_js_with_string_arg("(s) => s.toUpperCase()", "hello").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "HELLO");
    }

    #[tokio::test]
    async fn test_run_js_with_string_arg_async() {
        let result = run_js_with_string_arg(
            r#"async (s) => {
                await new Promise((r) => setTimeout(r, 10));
                return s + "!";
            }"#,
            "ok",
        )
        .await;
        assert!(result.is_ok(), "Should resolve async function: {:?}", result.err());
        assert_eq!(result.unwrap(), "ok!");
    }
}
