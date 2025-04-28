use minecraft_client_rs;
use std::{net::Ipv4Addr, process::Command};

pub fn run_game(path: String) -> bool {
    let mut is_running = check_if_running();

    if is_running {
        is_running = Command::new("sh")
            .current_dir(path)
            .args(["-C", "./run.sh", "-nogui"])
            .status()
            .expect("failed to start server")
            .success();
    }

    is_running
}

pub fn check_if_running() -> bool {
    let status = Command::new("pgrep")
        .args(["-f", "minecraft"])
        .status()
        .expect("Failed to run command");

    status.success()
}

pub struct rcon_config {
    ip: Ipv4Addr,
    port: String,
    password: String,
}

pub fn create_rcon_client(config: rcon_config) -> minecraft_client_rs::Client {
    let mut client = minecraft_client_rs::Client::new(format!(
        "{ip}:{port}",
        ip = config.ip,
        port = config.port
    ))
    .unwrap();

    match client.authenticate(config.password) {
        Ok(_) => {}
        Err(e) => { /* handle authentication error */ }
    }
    match client.send_command("seed".to_string()) {
        Ok(resp) => {
            println!("{}", resp.body);
        } // "Seed: [1871644822592853811]"
        Err(e) => { /* handle error */ }
    }

    // Disconnect cleanly when finished.
    // client.close().unwrap();
    client
}
