// Sapphillon-Core
// SPDX-FileCopyrightText: 2025 Yuta Takahashi
// SPDX-License-Identifier: MPL-2.0 OR GPL-3.0-or-later

use anyhow::Result;

use crate::{RsJsBridgeArgs, RsJsBridgeReturns, SapphillonPackage};
use ipc_channel::ipc::{self, IpcOneShotServer, IpcSender};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalPluginRunRequest {
    pub package_js: String,
    pub func_name: String,
    pub args_json: String,
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
) -> Result<RsJsBridgeReturns> {
    let (server, server_name) = IpcOneShotServer::<
        IpcSender<(
            IpcSender<ExternalPluginRunResponse>,
            ExternalPluginRunRequest,
        )>,
    >::new()?;

    let mut command = Command::new(server_path);
    command.args(server_args);
    command.arg(server_name);

    let mut child = command.stderr(std::process::Stdio::inherit()).spawn()?;

    let (_rx_bootstrap, tx_req) = server.accept()?;

    let (tx_res, rx_res) = ipc::channel()?;

    let request = ExternalPluginRunRequest {
        package_js: sapphillon_package.package_script.clone(),
        func_name: func_name.to_string(),
        args_json: args.to_string()?,
    };

    tx_req.send((tx_res.clone(), request))?;

    let response = rx_res.recv()?;

    let _ = child.kill();
    let _ = child.wait();

    if let Some(err) = response.error_message {
        anyhow::bail!(err);
    }

    RsJsBridgeReturns::new_from_str(&response.returns_json)
}

pub fn extplugin_server(server_name: &str) -> Result<()> {
    let (tx_req, rx_req) = ipc::channel()?;
    {
        eprintln!("DEBUG: Connecting to bootstrap");
        let tx_bootstrap: IpcSender<
            IpcSender<(
                IpcSender<ExternalPluginRunResponse>,
                ExternalPluginRunRequest,
            )>,
        > = IpcSender::connect(server_name.to_string())?;
        eprintln!("DEBUG: Sending tx_req");
        tx_bootstrap.send(tx_req.clone())?;
        eprintln!("DEBUG: Sent tx_req");
        std::mem::forget(tx_bootstrap); // Hack to avoid Drop panic?
    }
    std::mem::forget(tx_req);

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    if let Ok((tx_res, request)) = rx_req.recv() {
        let result = rt.block_on(async {
            let package = SapphillonPackage::new_async(&request.package_js).await?;
            let args = RsJsBridgeArgs::new_from_str(&request.args_json)?;
            package.execute(args, &None).await
        });

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

    #[test]
    fn test_extplugin_runner_process() -> Result<()> {
        let package_script = r#"
            globalThis.Sapphillon = {
                Package: {
                    meta: {
                        name: "test-plugin",
                        version: "1.0.0",
                        description: "Test plugin",
                        package_id: "com.example.test"
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

        let returns = extplugin_client(&package, "echo", &args, server_path, server_args)?;

        assert_eq!(returns.args.get("result"), Some(&json!("Hello, World!")));

        Ok(())
    }
}
