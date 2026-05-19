use ext_plugin::{ExternalPluginRunRequest, ExternalPluginRunResponse, extplugin_server};
use ipc_channel::ipc::{self, IpcSender};
use std::env;

async fn run(server_name: &str) -> anyhow::Result<()> {

    if env::var("SAPPHILLON_TEST_SERVER_ABORT").is_ok() {
        let (tx_req, rx_req) = ipc::channel()?;
        let tx_bootstrap: IpcSender<
            IpcSender<(
                IpcSender<ExternalPluginRunResponse>,
                ExternalPluginRunRequest,
            )>,
        > = IpcSender::connect(server_name.to_string())?;
        tx_bootstrap.send(tx_req.clone())?;
        std::mem::forget(tx_bootstrap);
        std::mem::forget(tx_req);

        if let Ok((_tx_res, _request)) = rx_req.recv() {
            std::process::exit(1);
        }
        std::process::exit(1);
    }

    extplugin_server(server_name).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_target(false)
        .init();
    let args: Vec<String> = env::args().collect();
    tracing::info!("Starting Test Extplugin Execution Server");
    if args.len() < 2 {
        anyhow::bail!("Usage: extplugin_test_server <server_name>");
    }
    let server_name = &args[1];
    tracing::info!("Server name: {server_name}");
    
    match run(server_name).await {
        Ok(()) => {}
        Err(e) => {
            tracing::error!("error occured: {e:#?}")
        }
    }

    Ok(())
}
