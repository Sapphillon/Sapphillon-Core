// Sapphillon-Core
// SPDX-FileCopyrightText: 2025 Yuta Takahashi
// SPDX-License-Identifier: MPL-2.0 OR GPL-3.0-or-later

use anyhow::Result;

use crate::{RsJsBridgeArgs, RsJsBridgeReturns, SapphillonPackage};
use ipc_channel::ipc::{self, IpcOneShotServer, IpcSender};
use proto::sapphillon::v1::Permission;
use serde::{Deserialize, Serialize};
use std::process::Command;

/// Serializable permission for IPC transfer.
/// This is a simple struct that mirrors proto::Permission but is guaranteed to
/// be serde-compatible for ipc-channel transfer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcPermission {
    pub permission_type: i32,
    pub resource: Vec<String>,
}

impl From<&proto::sapphillon::v1::Permission> for IpcPermission {
    fn from(p: &proto::sapphillon::v1::Permission) -> Self {
        Self {
            permission_type: p.permission_type,
            resource: p.resource.clone(),
        }
    }
}

impl From<IpcPermission> for proto::sapphillon::v1::Permission {
    fn from(p: IpcPermission) -> Self {
        Self {
            permission_type: p.permission_type,
            resource: p.resource,
            display_name: String::new(),
            description: String::new(),
            permission_level: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalPluginRunRequest {
    pub package_js: String,
    pub func_name: String,
    pub args_json: String,
    pub sapphillon_permissions: Vec<IpcPermission>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalPluginRunResponse {
    pub returns_json: String,
    pub error_message: Option<String>,
}

pub fn extplugin_client(
    sapphillon_package: &SapphillonPackage,
    func_name: &str,
    args: &RsJsBridgeArgs,
    server_path: &str,
    server_args: Vec<&str>,
    sapphillon_permissions: Vec<Permission>,
) -> Result<RsJsBridgeReturns> {
    let (server, server_name) = IpcOneShotServer::<
        IpcSender<(
            IpcSender<ExternalPluginRunResponse>,
            ExternalPluginRunRequest,
        )>,
    >::new()?;

    let mut command = Command::new(server_path);
    command.args(server_args);
    command.arg(&server_name);

    let mut child = command.stderr(std::process::Stdio::inherit()).spawn()?;

    let (_rx_bootstrap, tx_req) = server.accept()?;

    let (tx_res, rx_res) = ipc::channel()?;

    // Convert Permission to IpcPermission for IPC serialization
    let ipc_permissions: Vec<IpcPermission> = sapphillon_permissions
        .iter()
        .map(IpcPermission::from)
        .collect();

    let request = ExternalPluginRunRequest {
        package_js: sapphillon_package.package_script.clone(),
        func_name: func_name.to_string(),
        args_json: args.to_string()?,
        sapphillon_permissions: ipc_permissions,
    };

    tx_req.send((tx_res.clone(), request))?;

    // Wait for either the response or the server process termination
    let response = loop {
        if let Some(status) = child.try_wait()?
            && !status.success()
        {
            panic!("Server process terminated abnormally");
        }

        match rx_res.try_recv_timeout(std::time::Duration::from_millis(100)) {
            Ok(resp) => break resp,
            Err(ipc::TryRecvError::Empty) => continue,
            Err(ipc::TryRecvError::IpcError(err)) => anyhow::bail!(err),
        }
    };

    let mut killed = false;
    if child.try_wait()?.is_none() {
        let _ = child.kill();
        killed = true;
    }
    // TODO: Improve error handling for server process termination
    let exit_status = child.wait()?;
    if !exit_status.success() && !killed {
        panic!("Server process terminated abnormally: {exit_status:?}");
    }

    if let Some(err) = response.error_message {
        anyhow::bail!(err);
    }

    RsJsBridgeReturns::new_from_str(&response.returns_json)
}

pub async fn extplugin_server(server_name: &str) -> Result<()> {
    use crate::permissions::permissions_options_from_sapphillon_permissions;

    let (tx_req, rx_req) = ipc::channel()?;
    {
        let tx_bootstrap: IpcSender<
            IpcSender<(
                IpcSender<ExternalPluginRunResponse>,
                ExternalPluginRunRequest,
            )>,
        > = IpcSender::connect(server_name.to_string())?;
        tx_bootstrap.send(tx_req.clone())?;
        std::mem::forget(tx_bootstrap); // Hack to avoid Drop panic?
    }
    std::mem::forget(tx_req);

    if let Ok((tx_res, request)) = rx_req.recv() {
        // Convert IpcPermission back to proto::Permission
        let sapphillon_permissions: Vec<proto::sapphillon::v1::Permission> = request
            .sapphillon_permissions
            .into_iter()
            .map(proto::sapphillon::v1::Permission::from)
            .collect();
        // Convert Sapphillon permissions to Deno PermissionsOptions
        let permissions_options =
            permissions_options_from_sapphillon_permissions(&sapphillon_permissions);
        let permissions_options = if permissions_options == Default::default() {
            None
        } else {
            Some(permissions_options)
        };

        let excuter = async {
            let package = SapphillonPackage::new_async(&request.package_js).await?;
            let args = RsJsBridgeArgs::new_from_str(&request.args_json)?;
            package.execute(args, &permissions_options).await
        };

        let result = excuter.await;

        let response = match result {
            Ok(returns) => ExternalPluginRunResponse {
                returns_json: returns.to_string().unwrap_or_default(),
                error_message: None,
            },
            Err(e) => ExternalPluginRunResponse {
                returns_json: "{}".to_string(),
                error_message: Some(e.to_string()),
            },
        };

        let _ = tx_res.send(response);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::sync::{Mutex, OnceLock};

    static TEST_SERVER_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

    fn test_server_lock() -> std::sync::MutexGuard<'static, ()> {
        TEST_SERVER_MUTEX
            .get_or_init(|| Mutex::new(()))
            .lock()
            .unwrap_or_else(|err| err.into_inner())
    }

    #[test]
    fn test_extplugin_runner_process() -> Result<()> {
        let _guard = test_server_lock();
        let package_script = r#"
            globalThis.Sapphillon = {
                Package: {
                    meta: {
                        name: "test-plugin",
                        version: "1.0.0",
                        description: "Test plugin",
                        author_id: "com.example",
                        package_id: "com.example.test-plugin"
                    },
                    functions: {
                        echo: {
                            description: "Echoes the input",
                            permissions: [],
                            parameters: [{
                                idx: 0,
                                name: "message",
                                type: "string",
                                description: "Message to echo"
                            }],
                            returns: [{
                                idx: 0,
                                type: "string",
                                description: "Echoed message"
                            }],
                            handler: (message) => message
                        }
                    }
                }
            };
        "#;
        if std::env::var("SAPPHILLON_TEST_SERVER_ABORT").is_ok() {
            // Skip this test if the abort environment variable is set
            return Ok(());
        }

        let package = SapphillonPackage::new(package_script)?;

        let args = RsJsBridgeArgs {
            func_name: "echo".to_string(),
            args: vec![("message".to_string(), json!("Hello, World!"))]
                .into_iter()
                .collect(),
        };

        // Locate the test server binary.
        // We assume the binary is built and located in the target directory.
        // std::env::current_exe() returns path like .../target/debug/deps/test_name-hash
        let mut server_path_buf = std::env::current_exe()?;
        server_path_buf.pop(); // Remove test binary name
        if server_path_buf.file_name().and_then(|s| s.to_str()) == Some("deps") {
            server_path_buf.pop(); // Remove "deps"
        }
        server_path_buf.push("extplugin_test_server");

        // Build the binary if it doesn't exist
        if !server_path_buf.exists() {
            let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let status = std::process::Command::new("cargo")
                .args(["build", "--bin", "extplugin_test_server"])
                .current_dir(&manifest_dir)
                .status()
                .expect("Failed to execute cargo build");
            if !status.success() {
                anyhow::bail!("Failed to build extplugin_test_server");
            }
        }

        let server_path = server_path_buf
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid path"))?;
        let server_args = vec![];
        let sapphillon_permissions = vec![];

        let returns = extplugin_client(
            &package,
            "echo",
            &args,
            server_path,
            server_args,
            sapphillon_permissions,
        )?;

        assert_eq!(returns.args.get("result"), Some(&json!("Hello, World!")));

        Ok(())
    }

    #[test]
    fn test_extplugin_runner_process_abnormal_exit_panics() -> Result<()> {
        let _guard = test_server_lock();
        let package_script = r#"
            globalThis.Sapphillon = {
                Package: {
                    meta: {
                        name: "test-plugin",
                        version: "1.0.0",
                        description: "Test plugin",
                        author_id: "com.example",
                        package_id: "com.example.test-plugin"
                    },
                    functions: {
                        echo: {
                            description: "Echoes the input",
                            permissions: [],
                            parameters: [{
                                idx: 0,
                                name: "message",
                                type: "string",
                                description: "Message to echo"
                            }],
                            returns: [{
                                idx: 0,
                                type: "string",
                                description: "Echoed message"
                            }],
                            handler: (message) => message
                        }
                    }
                }
            };
        "#;

        let package = SapphillonPackage::new(package_script)?;

        let args = RsJsBridgeArgs {
            func_name: "echo".to_string(),
            args: vec![("message".to_string(), json!("Hello, World!"))]
                .into_iter()
                .collect(),
        };

        let mut server_path_buf = std::env::current_exe()?;
        server_path_buf.pop();
        if server_path_buf.file_name().and_then(|s| s.to_str()) == Some("deps") {
            server_path_buf.pop();
        }
        server_path_buf.push("extplugin_test_server");

        if !server_path_buf.exists() {
            let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let status = std::process::Command::new("cargo")
                .args(["build", "--bin", "extplugin_test_server"])
                .current_dir(&manifest_dir)
                .status()
                .expect("Failed to execute cargo build");
            if !status.success() {
                anyhow::bail!("Failed to build extplugin_test_server");
            }
        }

        let server_path = server_path_buf
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid path"))?;
        let server_args = vec![];
        let sapphillon_permissions = vec![];

        unsafe {
            std::env::set_var("SAPPHILLON_TEST_SERVER_ABORT", "1");
        }
        let result = std::panic::catch_unwind(|| {
            let _ = extplugin_client(
                &package,
                "echo",
                &args,
                server_path,
                server_args,
                sapphillon_permissions,
            );
        });
        unsafe {
            std::env::remove_var("SAPPHILLON_TEST_SERVER_ABORT");
        }

        assert!(result.is_err(), "Expected panic on abnormal server exit");

        Ok(())
    }
}
