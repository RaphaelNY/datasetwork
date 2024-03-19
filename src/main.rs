#![allow(unused)] // while exploring,remove for prod.
use std::io;
use std::collections::BTreeMap;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{thing, Datetime, Object, Thing, Value};
use surrealdb::{Error, Response};
use surrealdb::kvs::Datastore;
use surrealdb::dbs::Session;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;
use datasetwork::connect_to_db;
use datasetwork::config_database::DatabaseConfig;

#[derive(Debug, Serialize)]
struct Name<'a> {
    first: &'a str,
    last: &'a str,
}

#[derive(Debug, Serialize)]
struct Person<'a> {
    title: &'a str,
    name: Name<'a>,
    marketing: bool,
}

#[derive(Debug, Serialize)]
struct Responsibility {
    marketing: bool,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the server
    let config = match DatabaseConfig::from_file("config.toml") {
        Ok(config) => {
            println!("{:?}", config);
            config
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            return Err(Box::try_from(err).unwrap());
        }
    };

    if let Err(err) = connect_to_db(config).await {
        eprintln!("Error connecting to the database: {}", err);
        // 在这里可以选择如何处理错误，比如退出程序或者进行其他逻辑处理
    }

/*
    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;

    // Create a new person with a random id
    let created: Vec<Record> = db
        .create("person")
        .content(Person {
            title: "Founder & CEO",
            name: Name {
                first: "Tobie",
                last: "Morgan Hitchcock",
            },
            marketing: true,
        })
        .await?;
    dbg!(created);

    // Update a person record with a specific id
    let updated: Option<Record> = db
        .update(("person", "jaime"))
        .merge(Responsibility { marketing: true })
        .await?;
    dbg!(updated);

    // Select all people records
    let people: Vec<Record> = db.select("person").await?;
    dbg!(people);

    // Perform a custom advanced query
    let groups = db
        .query("SELECT marketing, count() FROM type::table($table) GROUP BY marketing")
        .bind(("table", "person"))
        .await?;
    dbg!(groups);
*/
    Ok(())
}