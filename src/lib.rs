use reqwest::Client;
use serde::{ Deserialize, Serialize };
use serde_json::json;
use std::env;

pub struct AnkiAdapter {
    url: String,
    client: Client,
}

impl AnkiAdapter {
    pub fn new() -> Result<AnkiAdapter, &'static str> {
        let mut adapter = AnkiAdapter{
            url: "http://localhost:8765".to_string(),
            client: Client::new(),
        };

        if let Ok(url) = env::var("ANKI_CONNECT_URL") {
            adapter.url = url;
        } else {
            println!("ANKI_CONNECT_URL not set. Using default url: {}", adapter.url);
        };

        Ok(adapter)
    }

    pub async fn create_deck(&self, deck_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let request =
            json!({
        "action": "createDeck",
        "version": 6,
        "params": {
            "deck": deck_name
        }
    });

        let response = &self.client
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

        let response = &self.client
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
