// src/lib.rs
use serde::{Deserialize, Serialize};
use serde_json::json;
use reqwest::Client;
use std::env;
use std::process;
use regex::Regex;

#[derive(Debug, Serialize, Deserialize)]
struct MessageContent {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: Vec<MessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlashcardResponse {
    pub deck_name: String,
    pub cards: Vec<Flashcard>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Flashcard {
    pub front: String,
    pub back: String,
    pub example: String,
    pub example_translate: String,
}

pub async fn generate_flashcards(user_input: &str) -> Result<FlashcardResponse, Box<dyn std::error::Error>> {
    // Load the OpenAI API key from the environment variable
    let api_key : String;

    if let Ok(key) = env::var("OPEN_API_KEY") {
        api_key = key
    } else {
        println!("OPEN_API_KEY not set. Please set the OPEN_API_KEY environment variable.");
        process::exit(1);
    }

    let client = Client::new();

    // Define the system and user messages
    let system_message_content = MessageContent {
        content_type: "text".to_string(),
        text: r#"You are a language teacher. You generate flashcards for students based on their request.
Flashcards are made of front and back. The request will contain two languages: their native language and their target language.

User's default native language is English.
Default target language is Japanese.

You only respond with this type of answer:
- Generate flashcard deck
Return JSON for the user to insert into a Flashcard application
Result should contains at least 15 flashcards.
Front of the flashcard should be in the target language. If the word is in Kanji, add readings in Hiragana and Romaji after the Kanji.
Back of the flashcard should be in the user's native language.
Example should be in the target language.
Example translation should be in the user's native language.

Example JSON format:
{
    "deck_name":"Places in Japanese",
    "cards":[
      {
        "front":"家 (いえ) (ie)",
        "back":"Home",
        "example":"私は家にいます (わたしはいえにいます) Watashi wa ie ni imasu",
        "example_translate":"I am home."
      }
    ]
}"#.to_string(),
    };

    let user_message_content = MessageContent {
        content_type: "text".to_string(),
        text: user_input.to_string(),
    };

    let messages = vec![
        Message {
            role: "system".to_string(),
            content: vec![system_message_content],
        },
        Message {
            role: "user".to_string(),
            content: vec![user_message_content],
        },
    ];

    // Prepare the request body
    let request_body = json!({
        "model": "gpt-4",
        "messages": messages,
        "temperature": 1,
        "max_tokens": 2048,
        "top_p": 1,
        "frequency_penalty": 0,
        "presence_penalty": 0
    });

    // Send the request to OpenAI API
    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&request_body)
        .send()
        .await?;

    let res_json: serde_json::Value = res.json().await?;

    // Extract the assistant's reply
    let assistant_reply = &res_json["choices"][0]["message"]["content"];

    // Extract JSON from the assistant's reply
    let assistant_reply_str = assistant_reply.as_str().unwrap();
    let json_text = extract_json(assistant_reply_str)?;

    // Parse the JSON into FlashcardResponse
    let flashcard_response: FlashcardResponse = serde_json::from_str(&json_text)?;

    Ok(flashcard_response)
}

// Helper function to extract JSON from the assistant's reply
fn extract_json(text: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Use regex to extract JSON between braces
    let re = Regex::new(r"(?s)\{.*\}")?;
    if let Some(mat) = re.find(text) {
        Ok(mat.as_str().to_string())
    } else {
        Err("No JSON found in assistant's reply".into())
    }
}
