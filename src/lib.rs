pub mod config_database;

use std::io;
use surrealdb::{Result, Surreal};
use surrealdb::engine::remote::ws::{Client,Ws};
use surrealdb::opt::auth::{Root,Scope};
use crate::config_database::DatabaseConfig;

#[derive(Debug)]
struct UserIO {
    username: String,
    password: String,
    namespace: Option<String>,
    database: Option<String>,
    scope: Option<String>,
}

impl UserIO {
    pub fn new(username: String, password: String) -> UserIO {
        // let (username, password) = get_credentials_from_console();
        UserIO {
            username,
            password,
            namespace: None,
            database: None,
            scope: None,
        }
    }
}

/// io, read account from user.
fn get_credentials_from_console() -> (String, String, String) {
    println!("Enter host:");
    let mut host = String::new();
    io::stdin().read_line(&mut host).expect("Failed to read password");

    println!("Enter username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to read username");

    println!("Enter password:");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read password");

    username = username.trim().to_string();
    password = password.trim().to_string();

    (host, username, password)
}

/// login to the database
pub async fn handle_signin(db: &Surreal<Client>, username: String, password: String, choice: String) -> Result<()> {
    let mut retry_count = 0;
    let max_retry: usize = 5;
    loop{
        let mut user = UserIO::new(username.clone(), password.clone());
        if choice.trim() == "A" || choice.trim() == "C" {
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
                    if retry_count >= max_retry {
                        eprintln!("Max retry count reached. Failed to  signin to database.");
                        return Err(err);
                    }
                    eprintln!("your account is worn: {}. Retrying ({}/{})", err, retry_count, &max_retry);
                }
            }
        }
        else if choice.trim() == "B" {
            user.namespace = Some("test".to_string());
            user.database = Some("test".to_string());
            user.scope = Some("user".to_string());
            match db.signup(Scope {
                namespace: user.namespace.as_ref().unwrap(),
                database: user.database.as_ref().unwrap(),
                scope: user.scope.as_ref().unwrap(),
                params: Root {
                    username: &*user.username,
                    password: &*user.password,
                },
            }).await {
                Ok(_) => {
                    println!("Signup successful!");
                    return Ok(())
                },
                Err(err) => {
                    retry_count += 1;
                    if retry_count >= max_retry {
                        eprintln!("Max retry count reached. Failed to  signup to database.");
                        return Err(err);
                    }
                    eprintln!("your account is worn: {}. Retrying ({}/{})", err, retry_count, &max_retry);
                }
            }
        }
    };
}

/// connect to the database
pub async fn connect_to_db(config: DatabaseConfig, choice: String) -> Result<()> {
    let mut retry_count = 0;
    let max_retry: usize = 10;

    let db = loop {
        match Surreal::new::<Ws>(config.host_port.to_string()).await {
            Ok(db) => {
                println!("Successfully connected to the database!");
                break db;
            }
            Err(err) => {
                retry_count += 1;
                if retry_count >= max_retry {
                    eprintln!("Max retry count reached. Failed to connect to the database.");
                    return Err(err.into());
                }
                eprintln!("Error connecting to the database: {}. Retrying ({}/{})", err, retry_count, &max_retry);
            }
        }
    };

    handle_signin(&db, config.username.to_string(), config.password.to_string(), choice).await?;
    Ok(())
}

// login function
pub async fn login() -> anyhow::Result<(), Box<dyn std::error::Error>> {
    let mut choice = String::new();

    // order to sign in or sign up
    loop {
        println!("A.signin or B.signup? or C.loading from Config :");
        match io::stdin().read_line(&mut choice) {
            Ok(_) => {
                match choice.trim() {
                    "A" => {
                        println!("Signup");
                        break
                    },
                    "B" => {
                        println!("Signin");
                        break
                    },
                    "C" => {
                        println!("Loading from Config");
                        break
                    },
                    _ => {
                        println!("Invalid choice");
                        continue;
                    }
                }
            },
            Err(err) => {
                eprintln!("Error: {}", err);
                return Err(Box::try_from(err).unwrap());
            }
        };
    }

    // get the login config
    let config = if choice.trim() == "A" {
        let (host_port, username, password) = get_credentials_from_console();
        Some(DatabaseConfig {
            host_port,
            username,
            password,
        })
    }
    else if choice.trim() == "B" {
        let (host_port, username, password) = get_credentials_from_console();
        Some(DatabaseConfig {
            host_port,
            username,
            password,
        })
    }
    else if choice.trim() == "C" {
        match DatabaseConfig::from_file("config.toml") {
            Ok(config) => {
                println!("{:?}", config);
                Some(config)
            },
            Err(err) => {
                eprintln!("Error: {}", err);
                return Err(Box::try_from(err).unwrap());
            }
        }
    }
    else {
        None
    };

    // log in to the database
    if let Some(config) = config {
        if let Err(err) = connect_to_db(config, choice).await {
            eprintln!("Error connecting to the database: {}", err);
            return Err(Box::try_from(err).unwrap());
        }
    }

    Ok(())
}