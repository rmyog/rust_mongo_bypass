use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::error::Error;
// use std::fs::File;
// use std::io::Read;
use std::io;
// use bson::{doc, bson};
use bson::doc;

use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let client_uri = 
    dotenv::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let db_name = dotenv::var("MONGODB_NAME").expect("You must set the MONGODB_NAME environment var!");

    println!("Enter an email: ");
    let mut email = String::new();

    io::stdin().read_line(&mut email).expect("failed to readline");

    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

    // let mut file = File::open("user.json").unwrap();
    // let mut data = String::new();
    // file.read_to_string(&mut data).unwrap();
    // let _json = bson::from_str(&data).unwrap();
    // let _fields = bson!(data);

    let filter: mongodb::bson::Document = doc! { "email" : &email.trim_end() };
    let update: mongodb::bson::Document = doc! { "$set" : {"temp" : "5" }};
    // let update: mongodb::bson::Document = doc! { "$set": fields };

    let user: mongodb::results::UpdateResult = client
        .database(&db_name)
        .collection::<String>("users")
        .update_one(filter, update, None)
        .await
        .unwrap();

    let base_user = serde_json::to_string_pretty(&user).unwrap();

    println!("{}", base_user);

    Ok(())
}

    // email:example
    // lukman.makarim96@gmail.com // KYC conleted
    // test-123.1evyw@aleeas.com