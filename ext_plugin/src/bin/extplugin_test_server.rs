use ext_plugin::extplugin_server;
use std::env;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        anyhow::bail!("Usage: extplugin_test_server <server_name>");
    }
    let server_name = &args[1];
    extplugin_server(server_name)?;
    Ok(())
}
