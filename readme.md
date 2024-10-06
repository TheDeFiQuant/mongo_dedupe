# MongoDB Dedupe

A Rust-based tool to deduplicate two collection which hold (Solana) transaction signatures documents between two MongoDB collections. The script loads both the source and target collections into memory, compares their contents, and inserts any unique documents from the source into the target. Optimized for high RAM environments.

## Document Format

```
{
  "_id": {
    "$oid": "66fd31eecddff3dc2931728e"
  },
  "signature": "3FQaQWvj4mDQDopuaAt2YarZPnPYndAqsjNvNwpwc7Xf2rRKKSHrcuEZsmDic8Gyssk9PVTydZSKyxc7XYf6451k",
  "slot": {
    "$numberLong": "293396930"
  },
  "err": null,
  "memo": null,
  "block_time": {
    "$numberLong": "1227819640"
  },
  "confirmation_status": "Finalized"
}
```

## Features

- Loads entire collections into memory for fast comparison.
- Uses environment variables for configuration.
- Extensive logging to track the progress of loading, checking, and inserting documents.
- Efficiently handles large MongoDB collections.

## Prerequisites

- **Rust & Cargo:** Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.
- **MongoDB Database:** Set up and configure MongoDB collections to deduplicate.
- **Environment Variables:** Configure collection names using environment variables or a `.env` file.

## Installation

1. **Clone the repository:**
    ```bash
    git clone https://github.com/your-username/mongo_dedupe.git
    cd mongo_dedupe
    ```

2. **Configure environment variables:**

You can either use a `.env` file in the project root or set environment variables directly.

### Option 1: Create a `.env` file
Create a `.env` file in the project root with the following content:

    ```env
    SOURCE_COLLECTION=SOL_USDC_SIGNATURES
    TARGET_COLLECTION=temp_SOL_USDC_SIGNATURES
    ```

### Option 2: Set environment variables directly

    ```bash
    export SOURCE_COLLECTION=your_source_collection_name
    export TARGET_COLLECTION=your_target_collection_name
    ``` 

Adjust the collection names as needed.

3. **Install dependencies:** Ensure you have the necessary Rust crates by running:

    ```bash
    cargo build
    ```

## Usage

To run the deduplication process, simply execute:

    ```bash
    cargo run
    ```

### Logs

The script logs all progress to the console and also writes detailed logs to a file called `process_logs.txt` in the project directory. This includes:

- Number of documents loaded from both the source and target collections.
- Progress updates for every 10,000 documents loaded.
- Progress updates for every 10,000 documents checked for duplicates.
- Progress updates for every 1,000 new documents found for insertion.

### Output

- If new documents are found in the source collection that are not in the target, they are inserted into the target collection.
- A summary is provided at the end of the process.

## Configuration

You can adjust the following environment variables:

- **`SOURCE_COLLECTION`**: The name of the collection from which to copy unique documents.
- **`TARGET_COLLECTION`**: The name of the collection to which unique documents will be inserted.

## Project Structure

- **`src/main.rs`**: Main Rust script containing the logic for loading, checking, and inserting documents between the source and target collections.
- **`process_logs.txt`**: Log file generated during the script execution.

## Dependencies

- **mongodb**: MongoDB driver for Rust.
- **serde**: Serialization and deserialization for Rust data structures.
- **tokio**: Asynchronous runtime for Rust.
- **dotenv**: Loads environment variables from a `.env` file.
- **futures**: Asynchronous streams and tasks.

## Notes

- The script is designed to work best on systems with a large amount of available RAM, as it loads the entire source and target collections into memory for faster comparison.
- The script assumes both collections have a field called `signature` to identify unique documents.
