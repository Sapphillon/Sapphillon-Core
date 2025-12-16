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

use anyhow::{Result, anyhow, bail};
use deno_runtime::deno_core::PollEventLoopOptions;
use deno_runtime::deno_core::v8;
use deno_permissions::PermissionsOptions;

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
pub async fn run_js(script: &str, permissions_options: &Option<PermissionsOptions>) -> Result<i32> {
    let mut worker = create_main_worker(permissions_options)?;

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
/// **What this does (high level)**
/// 1. Boot a Deno `MainWorker` (with snapshot)
/// 2. Evaluate `function_source` to obtain a JS function
/// 3. Convert the function and argument into V8 handles (`v8::Global`)
/// 4. Call the function from Rust; if it returns a Promise, drive the event loop until resolved
/// 5. Convert the return value into a Rust `String`
/// 6. Run Deno unload lifecycle hooks and return
///
/// **Input contract**
/// - `function_source` must be a JavaScript *expression* that evaluates to a function.
///   Examples:
///   - `(s) => s.toUpperCase()`
///   - `async (s) => { await new Promise(r => setTimeout(r, 10)); return s + "!"; }`
/// - The function must return a string (or `String` object). Returning `null/undefined` is an error.
pub async fn run_js_with_string_arg(function_source: &str, arg: &str, permissions_options: &Option<PermissionsOptions>) -> Result<String> {
    let mut worker = create_main_worker(permissions_options)?;

    // 1) Evaluate the function expression. We wrap in parentheses so that arrow functions / function
    // expressions parse as an expression (not a statement) and the evaluated result becomes the
    // return value of `execute_script`.
    let function_value =
        worker.execute_script("[ext_plugin]", format!("({function_source})").into())?;

    // 2) Dispatch the Deno "load" event. This mirrors `run_js` and lets runtime init hooks run.
    worker.dispatch_load_event()?;

    // 3) Convert the evaluated value into a `v8::Function` handle.
    //    We promote locals to `v8::Global` so they remain valid outside this V8 scope.
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

    // 4) Create the string argument as a V8 value and promote it to `v8::Global<v8::Value>`.
    let arg_global = {
        deno_runtime::deno_core::scope!(scope, &mut worker.js_runtime);
        v8::tc_scope!(tc_scope, scope);
        let v8_str = v8::String::new(tc_scope, arg)
            .ok_or_else(|| anyhow!("failed to allocate V8 string"))?;
        let arg_value: v8::Local<v8::Value> = v8_str.into();
        v8::Global::<v8::Value>::new(tc_scope, arg_value)
    };

    // 5) Call the function.
    //    - If it returns a normal value, this completes quickly.
    //    - If it returns a Promise (e.g. async function), we must drive Deno's event loop until
    //      the Promise settles.
    let call_fut = worker
        .js_runtime
        .call_with_args(&function_global, &[arg_global]);
    let result_value = worker
        .js_runtime
        .with_event_loop_promise(Box::pin(call_fut), PollEventLoopOptions::default())
        .await?;

    // 6) Convert the returned JS value to a Rust `String`.
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

    // 7) Drain pending tasks and run unload lifecycle hooks, like `run_js`.
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
        let result = run_js("console.log('Hello from Deno MainWorker!')", &None).await;
        assert!(result.is_ok(), "Should be able to run console.log");
        assert_eq!(result.unwrap(), 0, "Exit code should be 0");
    }

    #[tokio::test]
    async fn test_run_js_simple_calculation() {
        let result = run_js("const x = 1 + 1; console.log('1 + 1 =', x);", &None).await;
        assert!(result.is_ok(), "Should be able to run simple calculations");
        assert_eq!(result.unwrap(), 0, "Exit code should be 0");
    }

    #[tokio::test]
    async fn test_run_js_fetch() {
        use httpmock::{Method::GET, MockServer};

        let server = MockServer::start_async().await;
        let _mock = server
            .mock_async(|when, then| {
                when.method(GET).path("/get");
                then.status(200)
                    .header("content-type", "application/json")
                    .body(r#"{"origin":"127.0.0.1"}"#);
            })
            .await;

        let url = format!("{}/get", server.base_url());
        // Allow network access to the mock server for this test
        let mut permissions = PermissionsOptions::default();
        // Grant network access to the mock server (host:port). Strip scheme.
        let host = server
            .base_url()
            .trim_start_matches("http://")
            .trim_start_matches("https://")
            .to_string();
        permissions.allow_net = Some(vec![host]);

        let result = run_js(&format!(
            r#"
            (async () => {{
                const response = await fetch('{url}');
                console.log('Fetch status:', response.status);
                const data = await response.json();
                console.log('Fetch origin:', data.origin);
            }})();
            "#
        ), &Some(permissions))
        .await;
        assert!(
            result.is_ok(),
            "Should be able to run fetch: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    async fn test_run_js_with_string_arg_sync() {
        let result = run_js_with_string_arg("(s) => s.toUpperCase()", "hello", &None).await;
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
            &None,
        )
        .await;
        assert!(
            result.is_ok(),
            "Should resolve async function: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap(), "ok!");
    }
}
