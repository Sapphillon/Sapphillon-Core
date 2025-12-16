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
use deno_permissions::PermissionsOptions;
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

/// Executes JavaScript code that defines `entrypoint` and then calls `entrypoint(string) -> string`.
///
/// `script` must define `globalThis.entrypoint` (or a global `function entrypoint(...) {}`)
/// that returns a string (or `String` object). If it returns a Promise, the worker event loop
/// is driven until the Promise resolves.
pub async fn run_js_with_string_arg(
    script: &str,
    arg: &str,
    permissions_options: &Option<PermissionsOptions>,
) -> Result<String> {
    let mut worker = create_main_worker(permissions_options)?;

    // 1) Execute the script so it can define `globalThis.entrypoint`.
    worker.execute_script("[ext_plugin]", script.to_string().into())?;

    // 2) Dispatch the Deno "load" event. This mirrors `run_js` and lets runtime init hooks run.
    worker.dispatch_load_event()?;

    // 3) Resolve `globalThis.entrypoint` and convert it into a `v8::Function` handle.
    //    We promote locals to `v8::Global` so they remain valid outside this V8 scope.
    let function_global = {
        deno_runtime::deno_core::scope!(scope, &mut worker.js_runtime);
        v8::tc_scope!(tc_scope, scope);

        let context = tc_scope.get_current_context();
        let global_obj = context.global(tc_scope);
        let key = v8::String::new(tc_scope, "entrypoint")
            .ok_or_else(|| anyhow!("failed to allocate V8 string"))?;
        let local_value = global_obj
            .get(tc_scope, key.into())
            .ok_or_else(|| anyhow!("failed to read globalThis.entrypoint"))?;

        if !local_value.is_function() {
            bail!("globalThis.entrypoint is not a function");
        }

        let local_fn: v8::Local<v8::Function> = local_value
            .try_into()
            .map_err(|_| anyhow!("failed to convert entrypoint to Function"))?;
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
    use tempfile::tempdir;

    fn assert_permission_denied(err: &anyhow::Error, expected: &[&str]) {
        let msg = err.to_string().to_lowercase();
        // Deno commonly uses "PermissionDenied" or "NotCapable" errors.
        assert!(
            msg.contains("permission") || msg.contains("notcapable"),
            "expected permission-related error, got: {msg}"
        );
        for needle in expected {
            assert!(
                msg.contains(&needle.to_lowercase()),
                "expected error to contain '{needle}', got: {msg}"
            );
        }
    }

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

        let result = run_js(
            &format!(
                r#"
            (async () => {{
                const response = await fetch('{url}');
                console.log('Fetch status:', response.status);
                const data = await response.json();
                console.log('Fetch origin:', data.origin);
            }})();
            "#
            ),
            &Some(permissions),
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
        let script = r#"
            function entrypoint(s) {
                return s.toUpperCase();
            }
        "#;
        let result = run_js_with_string_arg(script, "hello", &None).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "HELLO");
    }

    #[tokio::test]
    async fn test_run_js_with_string_arg_async() {
        let script = r#"
            globalThis.entrypoint = async (s) => {
                await new Promise((r) => setTimeout(r, 10));
                return s + "!";
            };
        "#;
        let result = run_js_with_string_arg(script, "ok", &None).await;
        assert!(
            result.is_ok(),
            "Should resolve async function: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap(), "ok!");
    }

    #[tokio::test]
    async fn test_permissions_fs_read_denied_by_default() {
        let dir = tempdir().expect("create temp dir");
        let file_path = dir.path().join("readme.txt");
        std::fs::write(&file_path, "secret").expect("write temp file");
        let file_path = file_path.to_string_lossy();

        let result = run_js(
            &format!(r#"Deno.readTextFileSync({:?});"#, file_path.as_ref()),
            &None,
        )
        .await;

        let err = result.expect_err("fs read should be denied without allow_read");
        assert_permission_denied(&err, &["read"]);
    }

    #[tokio::test]
    async fn test_permissions_fs_read_allowed_for_specific_path() {
        let dir = tempdir().expect("create temp dir");
        let file_path = dir.path().join("allowed.txt");
        std::fs::write(&file_path, "ok").expect("write temp file");
        let file_path_str = file_path.to_string_lossy().to_string();

        let permissions = PermissionsOptions {
            allow_read: Some(vec![file_path_str.clone()]),
            ..Default::default()
        };

        let result = run_js(
            &format!(
                r#"const s = Deno.readTextFileSync({file_path_str:?}); if (s.trim() !== 'ok') throw new Error('unexpected');"#,
            ),
            &Some(permissions),
        )
        .await;

        assert!(
            result.is_ok(),
            "fs read should be allowed: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap(), 0);
    }

    #[tokio::test]
    async fn test_permissions_fs_write_denied_by_default() {
        let dir = tempdir().expect("create temp dir");
        let file_path = dir.path().join("out.txt");
        let file_path = file_path.to_string_lossy();

        let result = run_js(
            &format!(
                r#"Deno.writeTextFileSync({:?}, 'nope');"#,
                file_path.as_ref()
            ),
            &None,
        )
        .await;

        let err = result.expect_err("fs write should be denied without allow_write");
        assert_permission_denied(&err, &["write"]);
    }

    #[tokio::test]
    async fn test_permissions_fs_write_allowed_for_directory() {
        let dir = tempdir().expect("create temp dir");
        let dir_str = dir.path().to_string_lossy().to_string();
        let file_path = dir.path().join("out.txt");
        let file_path_str = file_path.to_string_lossy().to_string();

        let permissions = PermissionsOptions {
            allow_write: Some(vec![dir_str]),
            ..Default::default()
        };

        let result = run_js(
            &format!(r#"Deno.writeTextFileSync({file_path_str:?}, 'ok');"#),
            &Some(permissions),
        )
        .await;

        assert!(
            result.is_ok(),
            "fs write should be allowed: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap(), 0);
        assert_eq!(std::fs::read_to_string(&file_path).unwrap(), "ok");
    }

    #[tokio::test]
    async fn test_permissions_net_denied_by_default() {
        use httpmock::{Method::GET, MockServer};

        let server = MockServer::start_async().await;
        let _mock = server
            .mock_async(|when, then| {
                when.method(GET).path("/get");
                then.status(200)
                    .header("content-type", "application/json")
                    .body(r#"{"ok":true}"#);
            })
            .await;

        let url = format!("{}/get", server.base_url());
        let result = run_js(
            &format!(r#"(async () => {{ await fetch({url:?}); }})();"#),
            &None,
        )
        .await;

        let err = result.expect_err("fetch should be denied without allow_net");
        assert_permission_denied(&err, &["net"]);
    }

    #[tokio::test]
    async fn test_permissions_env_denied_by_default() {
        // Use a commonly present variable to avoid relying on test-time env mutation.
        let result = run_js("Deno.env.get('PATH');", &None).await;
        let err = result.expect_err("env access should be denied without allow_env");
        assert_permission_denied(&err, &["env"]);
    }

    #[tokio::test]
    async fn test_permissions_env_allowed_for_specific_var() {
        let permissions = PermissionsOptions {
            allow_env: Some(vec!["PATH".to_string()]),
            ..Default::default()
        };

        let result = run_js(
            "const v = Deno.env.get('PATH'); if (typeof v !== 'string' || v.length === 0) throw new Error('unexpected');",
            &Some(permissions),
        )
        .await;

        assert!(
            result.is_ok(),
            "env access should be allowed: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap(), 0);
    }

    #[tokio::test]
    async fn test_permissions_run_denied_by_default() {
        let result = run_js(
            r#"
                const cmd = new Deno.Command('/bin/sh', { args: ['-c', 'echo ok'], clearEnv: true });
                cmd.outputSync();
            "#,
            &None,
        )
        .await;

        let err = result.expect_err("run should be denied without allow_run");
        assert_permission_denied(&err, &["run"]);
    }

    #[tokio::test]
    async fn test_permissions_run_allowed_for_command() {
        let permissions = PermissionsOptions {
            allow_run: Some(vec!["/bin/sh".to_string()]),
            ..Default::default()
        };

        let result = run_js(
            r#"
                const cmd = new Deno.Command('/bin/sh', { args: ['-c', 'printf ok'], clearEnv: true });
                const out = cmd.outputSync();
                const text = new TextDecoder().decode(out.stdout);
                if (text !== 'ok') throw new Error('unexpected:' + text);
            "#,
            &Some(permissions),
        )
        .await;

        assert!(result.is_ok(), "run should be allowed: {:?}", result.err());
        assert_eq!(result.unwrap(), 0);
    }
}
