mod db;

use bson::doc;
use colored::Colorize;
use config::Config;
use webhook::client::WebhookClient;
use core::panic;
use elytra_ping::{ping_or_timeout, JavaServerInfo, PingError};
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use tokio::time::Duration;
use std::error::Error;

#[derive(Serialize)]
pub struct MinecraftServer {
    pub ip: String,
    pub version: String,
    pub playerdata: Option<elytra_ping::parse::ServerPlayers>,
    pub motd: String,
}

#[derive(Serialize)]
struct PlayerDataBson {
    online: u32,
    max: u32,
}

enum ServerResult {
    Success(MinecraftServer),
    Failure(PingError),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load config
    let cfg = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .unwrap();

    let config = cfg.try_deserialize::<HashMap<String, String>>().unwrap();

    let url_suffix = "";

    let connection_uri = format!(
        "mongodb://{}:{}@{}/{}",
        config.get("mongo_root_username").unwrap(),
        config.get("mongo_root_passwd").unwrap(),
        config.get("mongo_hostname").unwrap(),
        url_suffix
    );

    let _init = db::init(
        connection_uri.to_string(),
        config.get("mongo_newuser_passwd").unwrap().to_string(),
    )
    .await;

    let iplist = fs::read_to_string(config.get("iplist_file").unwrap());

    let ips: Vec<String> = iplist
        .unwrap()
        .to_string()
        .lines()
        .map(|s| s.to_string())
        .collect(); // convert the lines with all ips to a vector

    // now here comes the fun part: actually iterate over all ips and scan them
    for (i, ip) in ips.iter().enumerate() {
        if i == ips.len() {
            println!("Successfully scanned all IPs! Exiting now...");
            std::process::exit(0);
        }
        let ip_clone = ip.to_string(); // Clone IP for asynchronous closure
        let config_clone = config.clone(); // Clone config for asynchronous closure

        let res = ping_server(
            ip_clone.clone(),
            config_clone.get("port").unwrap().parse::<u16>().unwrap(),
            // yes, it takes this much code to parse the timeout to u64.
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
                // test if there's actually a positive result
                ServerResult::Success(server) => {
                    println!(
                        "{} {} {}{}{}",
                        &ip.to_string().bright_green(),
                        "is a minecraft server!".bright_green(),
                        (i + 1).to_string().bright_green(),
                        "/".to_string().bright_green(),
                        ips.len().to_string().bright_green()
                    );

                    //send an embed to the webhook
                    if config.get("discord_webhook").unwrap() == "true" {
                        let client = WebhookClient::new(config.get("discord_webhook_url").unwrap());

                        let online_players: &str = &format!("{}", &server.playerdata.clone().expect("Error: playerdata not found").online);
                        let max_players: &str = &format!("{}", &server.playerdata.clone().expect("Error: playerdata not found").max);
                        
                        let mut sample_players: String = "".to_string();
                        let player_sample = &server.playerdata.clone().expect("Error: playerdata not found").sample.clone();
                        
                        for p in player_sample.iter() {
                            for player in p {
                                let player_name = player.name.as_ref().unwrap();
                                sample_players.push_str(&format!("{}\n", &player_name));
                                //sample_players = &format!("{}\n{}", sample_players, player.name.unwrap());
                            }
                        }

                        if sample_players == "".to_string() {
                            sample_players = "Nobody is online.".to_string();
                        }

                        let _ = client.send(|message| message
                            .username("radscanner")
                            .embed(|embed| embed
                                .title("Server found!")
                                .footer("radscanner - the fast server scanner", None)
                                .field("IP", ip, false)
                                .field("MOTD", &server.motd, false)
                                .field("Version", &server.version, false)
                                .field("Players online", online_players, false)
                                .field("Max players", max_players, false)
                                .field("Player sample", &sample_players, false)
                            )).await;
                    }

                    let servers_collection = db::get_servers_collection(connection_uri.clone()).await;
                    match servers_collection {
                        Ok(servers_collection) => {
                            servers_collection.insert_one(server, None).await?;
                        }
                        Err(err) => {
                            panic!("Error: {}", err);
                        }
                    }
                }
                ServerResult::Failure(_err) => {
                    eprintln!(
                        "{} {} {}{}{}",
                        ip.to_string().red(),
                        "is not a minecraft server.".red(),
                        (i + 1).to_string().red(),
                        "/".to_string().red(),
                        ips.len().to_string().red()
                    );
                }
            },
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }

    Ok(())
}

async fn ping_server(
    ip: String,
    port: u16,
    timeout: Duration,
) -> Result<ServerResult, Box<dyn std::error::Error + Send + Sync>> {
    let ip_clone = ip.clone();
    let res: Result<(JavaServerInfo, Duration), PingError> =
        ping_or_timeout((ip, port), timeout).await;

    let server: MinecraftServer = match res {
        Ok((info, _)) => MinecraftServer {
            ip: ip_clone,
            version: info.version.clone().unwrap().name,
            playerdata: info.players.clone(),
            motd: info.description.text,
        },
        Err(e) => return Ok(ServerResult::Failure(e)),
    };

    Ok(ServerResult::Success(server))
}

/* fn convert_server_to_bson(server: &MinecraftServer) -> Document {
    let mut doc = Document::new();

    doc.insert("ip", &server.ip);
    doc.insert("version", &server.version);

    // Serialize the playerdata field as a custom type
    if let Some(playerdata) = &server.playerdata {
        let playerdata_as_bson = PlayerDataBson {
            online: playerdata.online,
            max: playerdata.max,
        };

        // Serialize playerdata_as_bson to BSON
        let playerdata_bson = bson::to_bson(&playerdata_as_bson).unwrap_or(Bson::Null);

        // Insert the BSON into the document
        doc.insert("playerdata", playerdata_bson);
    } else {
        doc.insert("playerdata", bson::Bson::Null);
    }

    doc.insert("motd", &server.motd);

    doc
} */
