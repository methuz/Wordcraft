use autoflashcard::anki_adapter::AnkiAdapter;
use mockito::{Mock, Server};
use serde_json::json;
use serial_test::serial;

async fn setup_mock_server() -> (mockito::ServerGuard, AnkiAdapter) {
    let server = mockito::Server::new_async().await;
    let url = server.url();
    std::env::set_var("ANKI_CONNECT_URL", &url);
    
    let adapter = AnkiAdapter::new().expect("Failed to create adapter");
    (server, adapter)
}

#[tokio::test]
#[serial]
async fn test_anki_adapter_new_with_default_url() {
    std::env::remove_var("ANKI_CONNECT_URL");
    let adapter = AnkiAdapter::new().expect("Failed to create adapter");
    // Note: url field is private, so we'll test by checking the behavior instead
    // This test would pass if the adapter connects to the default URL
}

#[tokio::test]
#[serial]
async fn test_anki_adapter_new_with_custom_url() {
    std::env::set_var("ANKI_CONNECT_URL", "http://custom:9999");
    let adapter = AnkiAdapter::new().expect("Failed to create adapter");
    // Note: url field is private, so we'll test by checking the behavior instead
    std::env::remove_var("ANKI_CONNECT_URL");
}

#[tokio::test]
#[serial]
async fn test_check_connection_success() {
    let (mut server, adapter) = setup_mock_server().await;
    
    let _m = server.mock("POST", "/")
        .with_body(json!({
            "result": 6,
            "error": null
        }).to_string())
        .create();
    
    let result = adapter.check_connection().await;
    assert!(result.is_ok());
}

#[tokio::test]
#[serial]
async fn test_check_connection_failure() {
    let (mut server, adapter) = setup_mock_server().await;
    
    let _m = server.mock("POST", "/")
        .with_body(json!({
            "result": null,
            "error": "AnkiConnect is not running"
        }).to_string())
        .create();
    
    let result = adapter.check_connection().await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("AnkiConnect error"));
}

#[tokio::test]
#[serial]
async fn test_create_deck_success() {
    let (mut server, adapter) = setup_mock_server().await;
    
    let _m = server.mock("POST", "/")
        .match_body(mockito::Matcher::JsonString(json!({
            "action": "createDeck",
            "version": 6,
            "params": {
                "deck": "TestDeck"
            }
        }).to_string()))
        .with_body(json!({
            "result": 1234567890,
            "error": null
        }).to_string())
        .create();
    
    let result = adapter.create_deck("TestDeck").await;
    assert!(result.is_ok());
}

#[tokio::test]
#[serial]
async fn test_create_deck_already_exists() {
    let (mut server, adapter) = setup_mock_server().await;
    
    let _m = server.mock("POST", "/")
        .with_body(json!({
            "result": null,
            "error": "Deck already exists"
        }).to_string())
        .create();
    
    let result = adapter.create_deck("ExistingDeck").await;
    assert!(result.is_ok()); // Should not return error, just print message
}

#[tokio::test]
#[serial]
async fn test_add_card_success() {
    let (mut server, adapter) = setup_mock_server().await;
    
    let _m = server.mock("POST", "/")
        .match_body(mockito::Matcher::JsonString(json!({
            "action": "addNote",
            "version": 6,
            "params": {
                "note": {
                    "deckName": "TestDeck",
                    "modelName": "Wordcraft",
                    "fields": {
                        "Front": "こんにちは",
                        "Back": "Hello",
                        "Example": "こんにちは、元気ですか？",
                        "ExampleTranslation": "Hello, how are you?"
                    },
                    "options": {
                        "allowDuplicate": false
                    },
                    "tags": ["wordcraft", "language_learning"]
                }
            }
        }).to_string()))
        .with_body(json!({
            "result": 1234567890,
            "error": null
        }).to_string())
        .create();
    
    let result = adapter.add_card(
        "TestDeck",
        "こんにちは",
        "Hello",
        "こんにちは、元気ですか？",
        "Hello, how are you?"
    ).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
#[serial]
async fn test_add_card_duplicate_error() {
    let (mut server, adapter) = setup_mock_server().await;
    
    let _m = server.mock("POST", "/")
        .with_body(json!({
            "result": null,
            "error": "Note already exists"
        }).to_string())
        .create();
    
    let result = adapter.add_card(
        "TestDeck",
        "test",
        "test",
        "example",
        "example translation"
    ).await;
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Error adding card"));
}

#[tokio::test]
#[serial]
async fn test_ensure_wordcraft_model_exists_already_exists() {
    let (mut server, adapter) = setup_mock_server().await;
    
    let _m = server.mock("POST", "/")
        .match_body(mockito::Matcher::JsonString(json!({
            "action": "modelNames",
            "version": 6
        }).to_string()))
        .with_body(json!({
            "result": ["Basic", "Cloze", "Wordcraft"],
            "error": null
        }).to_string())
        .create();
    
    let result = adapter.ensure_wordcraft_model_exists().await;
    assert!(result.is_ok());
}

#[tokio::test]
#[serial]
async fn test_ensure_wordcraft_model_creates_new() {
    let (mut server, adapter) = setup_mock_server().await;
    
    let model_check = server.mock("POST", "/")
        .match_body(mockito::Matcher::JsonString(json!({
            "action": "modelNames",
            "version": 6
        }).to_string()))
        .with_body(json!({
            "result": ["Basic", "Cloze"],
            "error": null
        }).to_string())
        .create();
    
    let model_create = server.mock("POST", "/")
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
    
    let result = adapter.ensure_wordcraft_model_exists().await;
    assert!(result.is_ok());
    
    model_check.assert();
    model_create.assert();
}

#[tokio::test]
#[serial]
async fn test_ensure_wordcraft_model_create_error() {
    let (mut server, adapter) = setup_mock_server().await;
    
    let _model_check = server.mock("POST", "/")
        .match_body(mockito::Matcher::JsonString(json!({
            "action": "modelNames",
            "version": 6
        }).to_string()))
        .with_body(json!({
            "result": ["Basic", "Cloze"],
            "error": null
        }).to_string())
        .create();
    
    let _model_create = server.mock("POST", "/")
        .match_body(mockito::Matcher::PartialJsonString(json!({
            "action": "createModel",
            "version": 6
        }).to_string()))
        .with_body(json!({
            "result": null,
            "error": "Failed to create model"
        }).to_string())
        .create();
    
    let result = adapter.ensure_wordcraft_model_exists().await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Error creating Wordcraft model"));
}

#[tokio::test]
#[serial]
async fn test_connection_timeout() {
    let (server, adapter) = setup_mock_server().await;
    // Don't create any mock, so the server won't respond
    drop(server); // Drop server to simulate connection failure
    
    let result = adapter.check_connection().await;
    assert!(result.is_err());
}