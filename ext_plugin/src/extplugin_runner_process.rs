// Sapphillon-Core
// SPDX-FileCopyrightText: 2025 Yuta Takahashi
// SPDX-License-Identifier: MPL-2.0 OR GPL-3.0-or-later

use anyhow::Result;

use crate::{SapphillonPackage, RsJsBridgeArgs, RsJsBridgeReturns};
use ipc_channel::ipc::{self, IpcSender, IpcOneShotServer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalPluginRunRequest {
    pub package: SapphillonPackage,
    pub func_name: String,
    pub args: RsJsBridgeArgs
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalPluginRunResponse {
    pub returns: RsJsBridgeReturns,
    pub error_message: Option<String>,
}


pub fn extplugin_client(sapphillon_package: &SapphillonPackage, func_name: &str, args: &RsJsBridgeArgs, server_path: &str, server_args: Vec<&str>) -> Result<RsJsBridgeReturns> {
    let (server, server_name) = IpcOneShotServer::<IpcSender<(ExternalPluginRunRequest, IpcSender<ExternalPluginRunResponse>)>>::new()?;

    let mut command = Command::new(server_path);
    command.args(server_args);
    command.arg(server_name);

    let mut child = command.spawn()?;

    let (_, tx_req) = server.accept()?;

    let (tx_res, rx_res) = ipc::channel()?;

    let request = ExternalPluginRunRequest {
        package: sapphillon_package.clone(),
        func_name: func_name.to_string(),
        args: args.clone(),
    };

    tx_req.send((request, tx_res))?;

    let response = rx_res.recv()?;

    let _ = child.kill();
    let _ = child.wait();

    if let Some(err) = response.error_message {
        anyhow::bail!(err);
    }

    Ok(response.returns)
}

pub fn extplugin_server(server_name: &str) -> Result<()> {
    let tx_bootstrap: IpcSender<IpcSender<(ExternalPluginRunRequest, IpcSender<ExternalPluginRunResponse>)>> = IpcSender::connect(server_name.to_string())?;
    let (tx_req, rx_req) = ipc::channel()?;
    tx_bootstrap.send(tx_req)?;

    let rt = tokio::runtime::Runtime::new()?;

    loop {
        match rx_req.recv() {
            Ok((request, tx_res)) => {
                let result = rt.block_on(async {
                    request.package.execute(request.args, &None).await
                });

                let response = match result {
                    Ok(returns) => ExternalPluginRunResponse {
                        returns,
                        error_message: None,
                    },
                    Err(e) => ExternalPluginRunResponse {
                        returns: RsJsBridgeReturns { args: HashMap::new() },
                        error_message: Some(e.to_string()),
                    },
                };

                if tx_res.send(response).is_err() {
                    break;
                }
            }
            Err(_) => break,
        }
    }

    Ok(())
}