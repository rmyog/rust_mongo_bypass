use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use bson::doc;
use std::error::Error;
use std::io;
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

    let filter: mongodb::bson::Document = doc! { "email" : &email.trim_end() };
    let update: mongodb::bson::Document = doc! { "$set": {
        "peruri": {
            "register": {
                "name":"Lukman Makarim",
                "phone":"+6281356286096",
                "type":"INDIVIDUAL",
                "email":"lukman@gmail.com",
                "ktp":"14045",
                "selfPhoto":null,
                "address":"Jl. Intan RSPP No.16C Cilandak Barat",
                "city":"South Jakarta",
                "province":"Jakarta",
                "gender":"M",
                "placeOfBirth":"Jakarta",
                "dateOfBirth":"14\\/02\\/1996"
            }
        },
        "peruri_status": "APPROVED",
        "peruri_response": {
            "register":{
                "resultCode":"0",
                "resultDesc":"Success"
            },
            "video":{
                "resultCode":"0",
                "resultDesc":"Success"
            },
            "speciment":{
                "resultCode":"0",
                "resultDesc":"Sukses update speciment"
            }
        }
    }};

    let user: mongodb::results::UpdateResult = client
        .database(&db_name)
        .collection::<String>("users")
        .update_one(filter, update, None)
        .await?;

    let base_user = serde_json::to_string_pretty(&user)?;

    println!("{}", base_user);

    Ok(())
}

// email:example
// lukman.makarim96@gmail.com // KYC conleted
// test-123.1evyw@aleeas.com