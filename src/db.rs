use std::{env, time::Duration};

use tokio::sync::OnceCell;

use mongodb::{
    bson::doc,
    options::{ClientOptions, GridFsBucketOptions, ServerApi, ServerApiVersion, WriteConcern},
    Client, Database, GridFsBucket,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mongo_url: String,
}

static CONNECTION: OnceCell<Database> = OnceCell::const_new();

pub async fn connection() -> &'static Database {
    let config = get_env_config();
    let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());
    let db_name: &str;
    if run_mode.to_lowercase() == "test" {
        db_name = "images-test";
    } else {
        db_name = "images";
    }

    let mut client_options = ClientOptions::parse_async(config.mongo_url).await.unwrap();

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    CONNECTION
        .get_or_init(|| async {
            let client = Client::with_options(client_options).unwrap();

            println!("init client db");
            let database = client.database(&db_name);
            let _ = create_bucket(&database);
            // let write_concern = WriteConcern::builder()
            //     .w_timeout(Duration::new(5, 0))
            //     .build();
            // let options = GridFsBucketOptions::builder()
            //     .bucket_name("image_bucket".to_string())
            //     .write_concern(write_concern)
            //     .build();
            // database.gridfs_bucket(options);
            println!("Connected to Mongo Database: {}", &db_name);
            database
        })
        .await
}

pub async fn create_bucket(database: &Database) -> () {
    let write_concern = WriteConcern::builder()
        .w_timeout(Duration::new(5, 0))
        .build();
    let options = GridFsBucketOptions::builder()
        .bucket_name("image_bucket".to_string())
        .write_concern(write_concern)
        .build();
    database.gridfs_bucket(options);
}

pub async fn get_bucket() -> Result<GridFsBucket, Box<dyn std::error::Error>> {
    let connection = connection().await;

    let write_concern = WriteConcern::builder()
        .w_timeout(Duration::new(5, 0))
        .build();
    let options = GridFsBucketOptions::builder()
        .bucket_name("image_bucket".to_string())
        .write_concern(write_concern)
        .build();

    let bucket = connection.gridfs_bucket(options);
    Ok(bucket)
}

pub fn get_env_config() -> Config {
    let env_vars = std::fs::read_to_string("env.toml").expect("unable to read config file");
    let config: Config = toml::from_str(&env_vars).expect("unable to parse toml file");
    config
}
