use autoflashcard::anki_adapter::AnkiAdapter;
use autoflashcard::langchain::{Flashcard, FlashcardResponse};
use mockito::Server;
use serde_json::json;
use serial_test::serial;

// Integration test for the full workflow
#[tokio::test]
#[serial]
async fn test_full_workflow_with_mock_anki() {
    // Setup mock server
    let mut server = Server::new_async().await;
    let url = server.url();
    std::env::set_var("ANKI_CONNECT_URL", &url);
    
    // Mock connection check
    let connection_mock = server.mock("POST", "/")
        .match_body(mockito::Matcher::JsonString(json!({
            "action": "version",
            "version": 6
        }).to_string()))
        .with_body(json!({
            "result": 6,
            "error": null
        }).to_string())
        .create();
    
    // Mock model check (model doesn't exist)
    let model_check_mock = server.mock("POST", "/")
        .match_body(mockito::Matcher::JsonString(json!({
            "action": "modelNames",
            "version": 6
        }).to_string()))
        .with_body(json!({
            "result": ["Basic", "Cloze"],
            "error": null
        }).to_string())
        .create();
    
    // Mock model creation
    let model_create_mock = server.mock("POST", "/")
        .match_body(mockito::Matcher::PartialJsonString(json!({
            "action": "createModel",
            "version": 6,
            "params": {
                "modelName": "Wordcraft"
            }
        }).to_string()))
        .with_body(json!({
            "result": 1234567890,
            "error": null
        }).to_string())
        .create();
    
    // Mock deck creation
    let deck_create_mock = server.mock("POST", "/")
        .match_body(mockito::Matcher::JsonString(json!({
            "action": "createDeck",
            "version": 6,
            "params": {
                "deck": "Spanish Basics"
            }
        }).to_string()))
        .with_body(json!({
            "result": 1234567890,
            "error": null
        }).to_string())
        .create();
    
    // Mock card additions
    let card_add_mocks: Vec<_> = (0..2).map(|i| {
        server.mock("POST", "/")
            .match_body(mockito::Matcher::PartialJsonString(json!({
                "action": "addNote",
                "version": 6,
                "params": {
                    "note": {
                        "deckName": "Spanish Basics",
                        "modelName": "Wordcraft"
                    }
                }
            }).to_string()))
            .with_body(json!({
                "result": 1234567890 + i,
                "error": null
            }).to_string())
            .create()
    }).collect();
    
    // Create adapter and run through the workflow
    let adapter = AnkiAdapter::new().expect("Failed to create adapter");
    
    // Check connection
    adapter.check_connection().await.expect("Connection check failed");
    
    // Ensure model exists
    adapter.ensure_wordcraft_model_exists().await.expect("Model creation failed");
    
    // Create test flashcards
    let flashcard_response = FlashcardResponse {
        deck_name: "Spanish Basics".to_string(),
        cards: vec![
            Flashcard {
                front: "Hola".to_string(),
                back: "Hello".to_string(),
                example: "Hola, ¿cómo estás?".to_string(),
                example_translate: "Hello, how are you?".to_string(),
            },
            Flashcard {
                front: "Adiós".to_string(),
                back: "Goodbye".to_string(),
                example: "Adiós, hasta luego".to_string(),
                example_translate: "Goodbye, see you later".to_string(),
            },
        ],
    };
    
    // Create deck
    adapter.create_deck(&flashcard_response.deck_name).await.expect("Deck creation failed");
    
    // Add cards
    for card in &flashcard_response.cards {
        adapter.add_card(
            &flashcard_response.deck_name,
            &card.front,
            &card.back,
            &card.example,
            &card.example_translate,
        ).await.expect("Card addition failed");
    }
    
    // Verify all mocks were called
    connection_mock.assert();
    model_check_mock.assert();
    model_create_mock.assert();
    deck_create_mock.assert();
    for mock in card_add_mocks {
        mock.assert();
    }
    
    // Cleanup
    std::env::remove_var("ANKI_CONNECT_URL");
}

#[tokio::test]
#[serial]
async fn test_error_handling_in_workflow() {
    // Setup mock server
    let mut server = Server::new_async().await;
    let url = server.url();
    std::env::set_var("ANKI_CONNECT_URL", &url);
    
    // Mock connection failure
    let connection_mock = server.mock("POST", "/")
        .match_body(mockito::Matcher::JsonString(json!({
            "action": "version",
            "version": 6
        }).to_string()))
        .with_body(json!({
            "result": null,
            "error": "AnkiConnect not available"
        }).to_string())
        .create();
    
    let adapter = AnkiAdapter::new().expect("Failed to create adapter");
    
    // Connection should fail
    let result = adapter.check_connection().await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("AnkiConnect error"));
    
    connection_mock.assert();
    
    // Cleanup
    std::env::remove_var("ANKI_CONNECT_URL");
}

#[tokio::test]
#[serial]
async fn test_duplicate_card_handling() {
    // Setup mock server
    let mut server = Server::new_async().await;
    let url = server.url();
    std::env::set_var("ANKI_CONNECT_URL", &url);
    
    // Mock duplicate card error
    let duplicate_mock = server.mock("POST", "/")
        .match_body(mockito::Matcher::PartialJsonString(json!({
            "action": "addNote",
            "version": 6
        }).to_string()))
        .with_body(json!({
            "result": null,
            "error": "cannot create note because it is a duplicate"
        }).to_string())
        .create();
    
    let adapter = AnkiAdapter::new().expect("Failed to create adapter");
    
    // Adding duplicate card should fail
    let result = adapter.add_card(
        "TestDeck",
        "duplicate",
        "duplicate",
        "example",
        "example translation"
    ).await;
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Error adding card"));
    
    duplicate_mock.assert();
    
    // Cleanup
    std::env::remove_var("ANKI_CONNECT_URL");
}

#[tokio::test]
#[serial]
async fn test_model_already_exists_scenario() {
    // Setup mock server
    let mut server = Server::new_async().await;
    let url = server.url();
    std::env::set_var("ANKI_CONNECT_URL", &url);
    
    // Mock model check (model already exists)
    let model_check_mock = server.mock("POST", "/")
        .match_body(mockito::Matcher::JsonString(json!({
            "action": "modelNames",
            "version": 6
        }).to_string()))
        .with_body(json!({
            "result": ["Basic", "Cloze", "Wordcraft"],
            "error": null
        }).to_string())
        .create();
    
    let adapter = AnkiAdapter::new().expect("Failed to create adapter");
    
    // Should succeed without creating model
    let result = adapter.ensure_wordcraft_model_exists().await;
    assert!(result.is_ok());
    
    model_check_mock.assert();
    
    // Cleanup
    std::env::remove_var("ANKI_CONNECT_URL");
}

#[test]
fn test_flashcard_response_with_empty_cards() {
    let response = FlashcardResponse {
        deck_name: "Empty Deck".to_string(),
        cards: vec![],
    };
    
    assert_eq!(response.deck_name, "Empty Deck");
    assert_eq!(response.cards.len(), 0);
}

#[test]
fn test_flashcard_response_with_many_cards() {
    let cards: Vec<Flashcard> = (0..100).map(|i| {
        Flashcard {
            front: format!("Front {}", i),
            back: format!("Back {}", i),
            example: format!("Example {}", i),
            example_translate: format!("Translation {}", i),
        }
    }).collect();
    
    let response = FlashcardResponse {
        deck_name: "Large Deck".to_string(),
        cards,
    };
    
    assert_eq!(response.cards.len(), 100);
    assert_eq!(response.cards[0].front, "Front 0");
    assert_eq!(response.cards[99].front, "Front 99");
}