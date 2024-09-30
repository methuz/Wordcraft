use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let deck_name = "RustDeck";
    create_deck(deck_name).await?;

    let front = "What is Rust?";
    let back = "A systems programming";

    add_card(deck_name, front, back).await?;

    Ok(())
}

async fn create_deck(deck_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();    

    let request = json!({
        "action": "createDeck",
        "version": 6,
        "params": {
            "deck": deck_name
        }
    });

    let response = client
        .post("http://localhost:8765")
        .json(&request)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    if response.get("error").is_some() {
        println!("Error creating deck {:?}", response["error"]);
    } else {
        println!("Deck '{}' ccreated successfully.", deck_name);
    }

    Ok(())
}

async fn add_card(deck_name: &str, front: &str, back: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let request = json!({
        "action":"addNote",
        "version": 6,
        "params": {
            "note":{
                "deckName": deck_name,
                "modelName": "Basic",
                "fields": {
                    "Front": front,
                    "Back": back
                },
                "options": {
                    "allowDuplicate": false
                },
                "tags": ["rust", "programming"]
            }
        }
    });

    let response = client
        .post("http://localhost:8765")
        .json(&request)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    if response.get("error").is_some() {
        println!("Error adding card: {:?}", response["error"]);
    } else {
        println!("Card added successfully to deck '{}'.", deck_name);
    }

    Ok(())
}
