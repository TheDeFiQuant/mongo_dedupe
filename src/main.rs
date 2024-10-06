use mongodb::{bson::doc, options::ClientOptions, Client, Collection};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use tokio;
use futures::stream::TryStreamExt;
use dotenv::dotenv; // Import dotenv

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
struct SignatureDoc {
    signature: String,

    #[serde(rename = "slot")]
    slot: Option<i64>,

    err: Option<String>,

    memo: Option<String>,

    #[serde(rename = "block_time")]
    block_time: Option<i64>,

    confirmation_status: Option<String>,
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    // Load environment variables from `.env` file if available
    dotenv().ok();

    // Open log file for appending logs
    let mut log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("process_logs.txt")
        .expect("Unable to open log file");

    // Connection string to MongoDB
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;

    // Access the OBv2_Data database
    let db = client.database("OBv2_Data");

    // Read collection names from environment variables
    let source_collection_name = env::var("SOURCE_COLLECTION")
        .expect("SOURCE_COLLECTION environment variable not set");
    let target_collection_name = env::var("TARGET_COLLECTION")
        .expect("TARGET_COLLECTION environment variable not set");

    // Access the collections with the SignatureDoc type
    let source_collection: Collection<SignatureDoc> = db.collection(&source_collection_name);
    let target_collection: Collection<SignatureDoc> = db.collection(&target_collection_name);

    // Function to log messages to both console and file
    let mut log = |message: &str| {
        println!("{}", message);
        if let Err(e) = writeln!(log_file, "{}", message) {
            eprintln!("Failed to write to log file: {}", e);
        }
    };

    // Load the source collection into memory
    log(&format!("Loading source collection '{}' into memory...", source_collection_name));
    let mut source_docs: HashSet<SignatureDoc> = HashSet::new();
    let mut source_cursor = source_collection.find(doc! {}).await?;
    let mut source_count = 0;
    while let Some(doc) = source_cursor.try_next().await? {
        source_docs.insert(doc);
        source_count += 1;
        // Log progress every 10k documents
        if source_count % 10_000 == 0 {
            log(&format!("Loaded {} documents from source collection into memory...", source_count));
        }
    }

    log(&format!("Source collection '{}' loaded with {} documents.", source_collection_name, source_count));

    // Load the target collection into memory
    log(&format!("Loading target collection '{}' into memory...", target_collection_name));
    let mut target_docs: HashSet<SignatureDoc> = HashSet::new();
    let mut target_cursor = target_collection.find(doc! {}).await?;
    let mut target_count = 0;
    while let Some(doc) = target_cursor.try_next().await? {
        target_docs.insert(doc);
        target_count += 1;
        // Log progress every 10k documents
        if target_count % 10_000 == 0 {
            log(&format!("Loaded {} documents from target collection into memory...", target_count));
        }
    }

    log(&format!("Target collection '{}' loaded with {} documents.", target_collection_name, target_count));

    // Filter documents to find new ones not present in target
    log("Checking for new documents to insert...");
    let mut new_docs: Vec<SignatureDoc> = Vec::new();
    let mut checked_count = 0;
    let mut insert_count = 0;
    for doc in source_docs {
        checked_count += 1;
        // Log progress every 10k documents checked
        if checked_count % 10_000 == 0 {
            log(&format!("Checked {} documents for duplicates...", checked_count));
        }

        // If not found in target, add to new_docs
        if !target_docs.contains(&doc) {
            new_docs.push(doc);
            insert_count += 1;

            // Log progress every 1k documents found for insertion
            if insert_count % 1_000 == 0 {
                log(&format!("Found {} new documents to insert...", insert_count));
            }
        }
    }

    log(&format!("Total new documents to insert: {}", new_docs.len()));

    // Insert new documents into the target collection
    if !new_docs.is_empty() {
        target_collection.insert_many(new_docs.clone()).await?;
        log(&format!("Inserted {} new documents into target collection.", new_docs.len()));
    } else {
        log("No new documents to insert.");
    }

    log("Merge complete!");

    Ok(())
}
