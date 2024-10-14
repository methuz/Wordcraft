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

use crate::prompts::SYSTEM_MESSAGE;


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

    let prompt = message_formatter![
        fmt_message!(Message::new_system_message(
            SYSTEM_MESSAGE
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
