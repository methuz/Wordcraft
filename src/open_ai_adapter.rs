// src/lib.rs
use serde::{Deserialize, Serialize};
use serde_json::json;
use reqwest::Client;
use std::env;
use std::process;
use regex::Regex;

use langchain_rust::{
    chain::{Chain, LLMChainBuilder},
    fmt_message, fmt_placeholder, fmt_template,
    language_models::llm::LLM,
    llm::openai::{OpenAI, OpenAIModel, OpenAIConfig},
    message_formatter,
    prompt::HumanMessagePromptTemplate,
    prompt_args,
    schemas::messages::Message,
    template_fstring,
};


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
     let open_ai = OpenAI::default().with_config(
        OpenAIConfig::default()
            .with_api_key(env::var("OPEN_API_KEY").unwrap()),
    );

    let system_message = r#"You are a language teacher. You generate flashcards for students based on their request.
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
}",
        "Make a personalized greeting for Joe"

    )

    // Create a chain with the system and user messages
    let chain = Chain::new(model)
        .system_message(r#"You are a language teacher. You generate flashcards for students based on their request.
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
}"#;
    let prompt = message_formatter![
        fmt_message!(Message::new_system_message(
            system_message
        )),
        fmt_template!(HumanMessagePromptTemplate::new(template_fstring!(
            "{input}", "input"
        )))
    ];

    let chain = LLMChainBuilder::new()
        .prompt(prompt)
        .llm(open_ai.clone())
        .build()
        .unwrap();

    let text = match chain
        .invoke(prompt_args! {
        "input" => user_input,
           })
        .await
    {
        Ok(result) => {
            result
        }
        Err(e) => panic!("Error invoking LLMChain: {:?}", e),
    };

    // Extract JSON from the response
    let json_text = extract_json(&text)?;

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
