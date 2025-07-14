use reqwest::Client;
use serde::{ Deserialize, Serialize };
use serde_json::{Value, json};
use std::env;

pub struct AnkiAdapter {
    pub(crate) url: String,
    client: Client,
}

impl AnkiAdapter {
    pub fn new() -> Result<AnkiAdapter, &'static str> {
        let mut adapter = AnkiAdapter {
            url: "http://localhost:8765".to_string(),
            client: Client::new(),
        };

        if let Ok(url) = env::var("ANKI_CONNECT_URL") {
            adapter.url = url;
        } else {
            println!("ANKI_CONNECT_URL not set. Using default url: {}", adapter.url);
        }

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

        if let Some(error) = response.get("error") {
            if !error.is_null() {
                println!("Error creating deck: {:?}", error.to_string());
            } else {
                println!("Deck '{}' created successfully.", deck_name);
            }
        } else {
            println!("Deck '{}' ccreated successfully.", deck_name);
        }

        Ok(())
    }

    pub async fn add_card(
        &self,
        deck_name: &str,
        front: &str,
        back: &str,
        example: &str,
        example_translate: &str
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request = json!({
            "action": "addNote",
            "version": 6,
            "params": {
                "note": {
                    "deckName": deck_name,
                    "modelName": "Wordcraft",
                    "fields": {
                        "Front": front,
                        "Back": back,
                        "Example": example,
                        "ExampleTranslation": example_translate
                    },
                    "options": {
                        "allowDuplicate": false
                    },
                    "tags": ["wordcraft", "language_learning"]
                }
            }
        });

        let response = self.client
            .post(&self.url)
            .json(&request)
            .send().await?
            .json::<serde_json::Value>().await?;

        if let Some(error) = response.get("error") {
            if !error.is_null() {
                return Err(format!("Error adding card: {}", error).into());
            }
        }

        println!("Card added successfully.");
        Ok(())
    }

    pub async fn check_connection(&self) -> Result<(), Box<dyn std::error::Error>> {
        let request = json!({
            "action": "version",
            "version": 6
        });

        let response: Value = self.client.post(&self.url)
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        if response["error"].is_null() {
            Ok(())
        } else {
            Err(format!("AnkiConnect error: {}", response["error"]).into())
        }
    }

    pub async fn ensure_wordcraft_model_exists(&self) -> Result<(), Box<dyn std::error::Error>> {
        let model_name = "Wordcraft";
        
        // Check if the model exists
        let request = json!({
            "action": "modelNames",
            "version": 6
        });

        let response = self.client
            .post(&self.url)
            .json(&request)
            .send().await?
            .json::<serde_json::Value>().await?;

        let model_names = response["result"].as_array()
            .ok_or("Failed to get model names")?;

        if !model_names.contains(&json!(model_name)) {
            // Create the model if it doesn't exist
            let request = json!({
                "action": "createModel",
                "version": 6,
                "params": {
                    "modelName": model_name,
                    "inOrderFields": ["Front", "Back", "Example", "ExampleTranslation"],
                    "css": ".card {
                        font-family: arial;
                        font-size: 20px;
                        text-align: center;
                        color: black;
                        background-color: white;
                    }
                    .front {
                        font-weight: bold;
                    }
                    .example {
                        font-style: italic;
                        color: #AAA;
                    }",
                    "cardTemplates": [
                        {
                            "Name": "Card 1",
                            "Front": "<div class='front'>{{Front}}</div><br><div class='example'>{{Example}}</div>",
                            "Back": "<div class='front'>{{Front}}</div><hr id=answer><div>{{Back}}</div><br><div class='example'>{{Example}}</div><br><div>{{ExampleTranslation}}</div>"
                        }
                    ]
                }
            });

            let response = self.client
                .post(&self.url)
                .json(&request)
                .send().await?
                .json::<serde_json::Value>().await?;

            if let Some(error) = response.get("error") {
                if !error.is_null() {
                    return Err(format!("Error creating Wordcraft model: {}", error).into());
                }
            }

            println!("Wordcraft model created successfully.");
        } else {
            println!("Wordcraft model already exists.");
        }

        Ok(())
    }
}
