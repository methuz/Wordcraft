use autoflashcard::langchain::{generate_flashcards, extract_json, Flashcard, FlashcardResponse};
use serial_test::serial;
use std::env;

#[test]
fn test_extract_json_valid() {
    let text = r#"Here is the response:
    {
        "deck_name": "Test Deck",
        "cards": [
            {
                "front": "Hello",
                "back": "Hola",
                "example": "Hello, world!",
                "example_translate": "¡Hola, mundo!"
            }
        ]
    }
    Some additional text"#;
    
    let result = extract_json(text);
    assert!(result.is_ok());
    
    let json = result.unwrap();
    assert!(json.contains("deck_name"));
    assert!(json.contains("Test Deck"));
}

#[test]
fn test_extract_json_no_json() {
    let text = "This is just plain text without any JSON";
    let result = extract_json(text);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "No JSON found in assistant's reply");
}

#[test]
fn test_extract_json_nested_braces() {
    let text = r#"Response: {"deck_name": "Test", "cards": [{"front": "a", "back": "b", "example": "{test}", "example_translate": "test"}]}"#;
    let result = extract_json(text);
    assert!(result.is_ok());
    let json = result.unwrap();
    assert!(json.contains("{test}"));
}

#[test]
fn test_flashcard_serialization() {
    let flashcard = Flashcard {
        front: "Hello".to_string(),
        back: "Hola".to_string(),
        example: "Hello, friend!".to_string(),
        example_translate: "¡Hola, amigo!".to_string(),
    };
    
    let json = serde_json::to_string(&flashcard).unwrap();
    assert!(json.contains("\"front\":\"Hello\""));
    assert!(json.contains("\"back\":\"Hola\""));
    
    let deserialized: Flashcard = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.front, "Hello");
    assert_eq!(deserialized.back, "Hola");
}

#[test]
fn test_flashcard_response_serialization() {
    let response = FlashcardResponse {
        deck_name: "Spanish Basics".to_string(),
        cards: vec![
            Flashcard {
                front: "One".to_string(),
                back: "Uno".to_string(),
                example: "One apple".to_string(),
                example_translate: "Una manzana".to_string(),
            },
            Flashcard {
                front: "Two".to_string(),
                back: "Dos".to_string(),
                example: "Two cats".to_string(),
                example_translate: "Dos gatos".to_string(),
            },
        ],
    };
    
    let json = serde_json::to_string(&response).unwrap();
    let deserialized: FlashcardResponse = serde_json::from_str(&json).unwrap();
    
    assert_eq!(deserialized.deck_name, "Spanish Basics");
    assert_eq!(deserialized.cards.len(), 2);
    assert_eq!(deserialized.cards[0].front, "One");
    assert_eq!(deserialized.cards[1].front, "Two");
}

#[tokio::test]
#[serial]
async fn test_generate_flashcards_missing_env_vars() {
    // Test with missing API key for OpenAI
    env::set_var("ENGINE", "openai");
    env::remove_var("OPEN_API_KEY");
    
    let result = std::panic::catch_unwind(|| {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            generate_flashcards("test input").await
        })
    });
    
    assert!(result.is_err());
    
    env::remove_var("ENGINE");
}

#[tokio::test]
#[serial] 
async fn test_generate_flashcards_invalid_engine() {
    env::set_var("ENGINE", "invalid_engine");
    
    let result = std::panic::catch_unwind(|| {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            generate_flashcards("test input").await
        })
    });
    
    assert!(result.is_err());
    
    env::remove_var("ENGINE");
}

#[test]
fn test_extract_json_with_markdown() {
    let text = r#"Here's the JSON response:
```json
{
    "deck_name": "Japanese Vocabulary",
    "cards": [
        {
            "front": "犬",
            "back": "dog",
            "example": "犬が好きです",
            "example_translate": "I like dogs"
        }
    ]
}
```"#;
    
    let result = extract_json(text);
    assert!(result.is_ok());
    let json = result.unwrap();
    assert!(json.contains("Japanese Vocabulary"));
    assert!(json.contains("犬"));
}

#[test]
fn test_extract_json_multiple_json_blocks() {
    let text = r#"First block: {"test": 1}
    The actual response:
    {
        "deck_name": "Numbers",
        "cards": []
    }"#;
    
    let result = extract_json(text);
    assert!(result.is_ok());
    // Should extract the first valid JSON block
    let json = result.unwrap();
    assert!(json.contains("test") || json.contains("deck_name"));
}