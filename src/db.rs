//use crate::convert_server_to_bson;
use bson::{doc, Document};
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client/* , Collection, */
};

use crate::MinecraftServer;

pub async fn init(uri: String, newuser_password: String) -> Result<(), mongodb::error::Error> {
    let options =
        ClientOptions::parse_with_resolver_config(&uri, ResolverConfig::cloudflare()).await?;
    let client = Client::with_options(options)?;

    let roles = vec![doc! {
        "role": "readWrite",
        "db": "radscanner",
    }];

    let db = client.database("radscanner");

    // create database "radscanner"
    db.run_command(doc! {"create": "radscanner"}, None).await?;

    create_user(uri, "radscanner".to_string(), newuser_password, roles).await?;

    db.create_collection("servers", None).await?;

    Ok(())
}

pub async fn create_user(
    uri: String,
    name: String,
    passwd: String,
    roles: Vec<Document>,
) -> Result<(), mongodb::error::Error> {
    let options = ClientOptions::parse(uri).await?;
    let client = Client::with_options(options)?;

    let admin_db = client.database("admin");

    let cmd = doc! {
        "createUser": name,
        "pwd": passwd,
        "roles": roles,
    };

    admin_db.run_command(cmd, None).await?;

    Ok(())
}

/* pub async fn insert_data_to_collection(
    collection: Collection<bson::Document>,
    bson_document: bson::Document,
) -> Result<(), mongodb::error::Error> {
    collection.insert_one(bson_document, None).await?;

    Ok(())
} */


pub async fn get_servers_collection(
    uri: String,
) -> Result<mongodb::Collection<MinecraftServer>, mongodb::error::Error> {
    let options = ClientOptions::parse(uri).await?;
    let client = Client::with_options(options)?;

    Ok(client
        .database("radscanner")
        .collection::<MinecraftServer>("servers"))
}
