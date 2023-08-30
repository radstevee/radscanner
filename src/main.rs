use colored::Colorize;
use config::Config;
use elytra_ping::parse::{ServerDescription, ServerPlayers};
use elytra_ping::{ping_or_timeout, PingError, ServerPingInfo};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use tokio::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
pub struct MinecraftServer {
    pub ip: String,
    pub version: String,
    pub playerdata: Option<ServerPlayers>,
    pub motd: ServerDescription,
}

#[derive(Debug)]
enum ServerResult {
    Success(MinecraftServer),
    Failure(PingError),
}

#[tokio::main]
async fn main() {
    let cfg = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .unwrap();

    let config = cfg.try_deserialize::<HashMap<String, String>>().unwrap();

    clear_file(config.get("output_file").unwrap().to_string());

    let iplist = fs::read_to_string(config.get("iplist_file").unwrap());

    let iplist_str = iplist.unwrap().to_string();

    let ips: Vec<String> = iplist_str.lines().map(|s| s.to_string()).collect();

    let mut data: Vec<MinecraftServer> = vec![];

    for (i, ip) in ips.iter().enumerate() {
        if i == ips.len() -1 {
            println!("Successfully scanned all IPs! Exiting now...");
            std::process::exit(0);
        }
        let ip_clone = ip.to_string(); // Clone IP for asynchronous closure
        let config_clone = config.clone(); // Clone config for asynchronous closure

        let res = ping_server(
            ip_clone.clone(),
            config_clone.get("port").unwrap().parse::<u16>().unwrap(),
            Duration::from_millis(
                config_clone
                    .get("timeout_ms")
                    .unwrap()
                    .parse::<u64>()
                    .unwrap(),
            ),
        )
        .await;

        match res {
            Ok(result) => match result {
                ServerResult::Success(server) => {
                    println!(
                        "{} {}",
                        &ip.to_string().bright_green(),
                        "is a minecraft server!".bright_green()
                    );

                    data.push(server);

                    let data_json = serde_json::to_string_pretty(&data).unwrap();

                    let mut file = File::create("output.json");
                    write!(file.expect("Cannot write output to JSON."), "{}", data_json);
                }
                ServerResult::Failure(err) => {
                    eprintln!(
                        "{} {}",
                        ip.to_string().red(),
                        "is not a minecraft server.".red()
                    );
                }
            },
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }
}

async fn ping_server(
    ip: String,
    port: u16,
    timeout: Duration,
) -> Result<ServerResult, Box<dyn std::error::Error + Send + Sync>> {
    let ip_clone = ip.clone();
    let res: Result<(ServerPingInfo, Duration), PingError> =
        ping_or_timeout((ip, port), timeout).await;

    //println!("{:#?}", res);

    let server: MinecraftServer = match res {
        Ok((info, _)) => MinecraftServer {
            ip: ip_clone,
            version: info.version.clone().unwrap().name,
            playerdata: info.players.clone(),
            motd: info.description,
        },
        Err(e) => return Ok(ServerResult::Failure(e)),
    };

    Ok(ServerResult::Success(server))
}

fn clear_file(file: String) -> std::io::Result<()> {
    // Open the file in write mode to truncate its contents
    let file = File::create(file)?;

    // Truncate the file by setting its length to 0
    file.set_len(0)?;

    Ok(())
}
