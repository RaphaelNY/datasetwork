pub mod config_database;

use std::io;
use surrealdb::{Result, Surreal};
use surrealdb::engine::remote::ws::{Client,Ws};
use surrealdb::opt::auth::Root;
use crate::config_database::DatabaseConfig;
#[derive(Debug)]
struct UserIO {
    username: String,
    password: String,
}

impl UserIO {
    pub fn new(username: String, password: String) -> UserIO {
        // let (username, password) = get_credentials_from_console();
        UserIO {
            username,
            password,
        }
    }
}

/// io, read account from user.
#[warn(dead_code)]
fn get_credentials_from_console() -> (String, String) {
    println!("Enter username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to read username");

    println!("Enter password:");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read password");

    username = username.trim().to_string();
    password = password.trim().to_string();

    (username, password)
}

/// login to the database
pub async fn handle_signin(db: &Surreal<Client>, username: String, password: String) -> Result<()> {
    let mut retry_count = 0;
    let MAX_RETRY: usize = 5;
    loop{
        let user = UserIO::new(username.clone(), password.clone());

        match db.signin(Root {
            username: &*user.username,
            password: &*user.password,
        }).await {
            Ok(_) => {
                println!("Signin successful!");
                return Ok(())
            },
            Err(err) => {
                retry_count += 1;
                if retry_count >= MAX_RETRY {
                    eprintln!("Max retry count reached. Failed to  signin to database.");
                    return Err(err);
                }
                eprintln!("your account is worn: {}. Retrying ({}/{})", err, retry_count, &MAX_RETRY);
            }
        }
    };
}

/// connect to the database
pub async fn connect_to_db(config: DatabaseConfig) -> Result<()> {
    let mut retry_count = 0;
    let MAX_RETRY: usize = 10;

    let db = loop {
        match Surreal::new::<Ws>(config.host_port.to_string()).await {
            Ok(db) => {
                println!("Successfully connected to the database!");
                break db;
            }
            Err(err) => {
                retry_count += 1;
                if retry_count >= MAX_RETRY {
                    eprintln!("Max retry count reached. Failed to connect to the database.");
                    return Err(err.into());
                }
                eprintln!("Error connecting to the database: {}. Retrying ({}/{})", err, retry_count, &MAX_RETRY);
            }
        }
    };

    handle_signin(&db, config.username.to_string(), config.password.to_string()).await?;
    Ok(())
}

