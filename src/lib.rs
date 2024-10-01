use reqwest::Client;
use serde::{ Deserialize, Serialize };
use serde_json::json;
use std::env;

pub struct AnkiAdapter {
    url: String,
}

impl AnkiAdapter {
    pub fn new() -> Result<AnkiAdapter, &'static str> {
        let url = match env::var("ANKI_CONNECT_URL") {
            Ok(val) => val,
            Err(_) => {
                println!("ANKI_CONNECT_URL not set. Using default value.");
                "http://localhost:8765".to_string()
            }
        };

        Ok(AnkiAdapter { url })
    }

    pub async fn create_deck(&self, deck_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();

        let request =
            json!({
        "action": "createDeck",
        "version": 6,
        "params": {
            "deck": deck_name
        }
    });

        let response = client
            .post(&self.url)
            .json(&request)
            .send().await?
            .json::<serde_json::Value>().await?;

        if response.get("error").is_some() {
            println!("Error creating deck {:?}", response["error"]);
        } else {
            println!("Deck '{}' ccreated successfully.", deck_name);
        }

        Ok(())
    }

    pub async fn add_card(
        &self,
        deck_name: &str,
        front: &str,
        back: &str
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();

        let request =
            json!({
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
            .post(&self.url)
            .json(&request)
            .send().await?
            .json::<serde_json::Value>().await?;

        if response.get("error").is_some() {
            println!("Error adding card: {:?}", response["error"]);
        } else {
            println!("Card added successfully to deck '{}'.", deck_name);
        }

        Ok(())
    }
}
