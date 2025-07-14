use mockito::{Mock, Server};
use serde_json::json;

pub struct MockAnkiServer {
    pub server: mockito::ServerGuard,
}

impl MockAnkiServer {
    pub async fn new() -> Self {
        let server = Server::new_async().await;
        std::env::set_var("ANKI_CONNECT_URL", &server.url());
        
        Self { server }
    }
    
    pub fn mock_successful_connection(&mut self) -> Mock {
        self.server.mock("POST", "/")
            .match_body(mockito::Matcher::JsonString(json!({
                "action": "version",
                "version": 6
            }).to_string()))
            .with_body(json!({
                "result": 6,
                "error": null
            }).to_string())
            .create()
    }
    
    pub fn mock_failed_connection(&mut self) -> Mock {
        self.server.mock("POST", "/")
            .match_body(mockito::Matcher::JsonString(json!({
                "action": "version",
                "version": 6
            }).to_string()))
            .with_body(json!({
                "result": null,
                "error": "AnkiConnect not available"
            }).to_string())
            .create()
    }
    
    pub fn mock_model_exists(&mut self, models: Vec<&str>) -> Mock {
        self.server.mock("POST", "/")
            .match_body(mockito::Matcher::JsonString(json!({
                "action": "modelNames",
                "version": 6
            }).to_string()))
            .with_body(json!({
                "result": models,
                "error": null
            }).to_string())
            .create()
    }
    
    pub fn mock_create_model_success(&mut self) -> Mock {
        self.server.mock("POST", "/")
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
            .create()
    }
    
    pub fn mock_create_model_failure(&mut self) -> Mock {
        self.server.mock("POST", "/")
            .match_body(mockito::Matcher::PartialJsonString(json!({
                "action": "createModel",
                "version": 6
            }).to_string()))
            .with_body(json!({
                "result": null,
                "error": "Failed to create model"
            }).to_string())
            .create()
    }
    
    pub fn mock_create_deck_success(&mut self, deck_name: &str) -> Mock {
        self.server.mock("POST", "/")
            .match_body(mockito::Matcher::JsonString(json!({
                "action": "createDeck",
                "version": 6,
                "params": {
                    "deck": deck_name
                }
            }).to_string()))
            .with_body(json!({
                "result": 1234567890,
                "error": null
            }).to_string())
            .create()
    }
    
    pub fn mock_create_deck_already_exists(&mut self, deck_name: &str) -> Mock {
        self.server.mock("POST", "/")
            .match_body(mockito::Matcher::JsonString(json!({
                "action": "createDeck",
                "version": 6,
                "params": {
                    "deck": deck_name
                }
            }).to_string()))
            .with_body(json!({
                "result": null,
                "error": "Deck already exists"
            }).to_string())
            .create()
    }
    
    pub fn mock_add_card_success(&mut self, deck_name: &str) -> Mock {
        self.server.mock("POST", "/")
            .match_body(mockito::Matcher::PartialJsonString(json!({
                "action": "addNote",
                "version": 6,
                "params": {
                    "note": {
                        "deckName": deck_name,
                        "modelName": "Wordcraft"
                    }
                }
            }).to_string()))
            .with_body(json!({
                "result": 1234567890,
                "error": null
            }).to_string())
            .create()
    }
    
    pub fn mock_add_card_duplicate(&mut self, deck_name: &str) -> Mock {
        self.server.mock("POST", "/")
            .match_body(mockito::Matcher::PartialJsonString(json!({
                "action": "addNote",
                "version": 6,
                "params": {
                    "note": {
                        "deckName": deck_name,
                        "modelName": "Wordcraft"
                    }
                }
            }).to_string()))
            .with_body(json!({
                "result": null,
                "error": "cannot create note because it is a duplicate"
            }).to_string())
            .create()
    }
    
    pub fn mock_add_card_failure(&mut self, deck_name: &str, error: &str) -> Mock {
        self.server.mock("POST", "/")
            .match_body(mockito::Matcher::PartialJsonString(json!({
                "action": "addNote",
                "version": 6,
                "params": {
                    "note": {
                        "deckName": deck_name,
                        "modelName": "Wordcraft"
                    }
                }
            }).to_string()))
            .with_body(json!({
                "result": null,
                "error": error
            }).to_string())
            .create()
    }
    
    pub fn mock_multiple_card_additions(&mut self, deck_name: &str, count: usize) -> Vec<Mock> {
        (0..count)
            .map(|i| {
                self.server.mock("POST", "/")
                    .match_body(mockito::Matcher::PartialJsonString(json!({
                        "action": "addNote",
                        "version": 6,
                        "params": {
                            "note": {
                                "deckName": deck_name,
                                "modelName": "Wordcraft"
                            }
                        }
                    }).to_string()))
                    .with_body(json!({
                        "result": 1234567890 + i,
                        "error": null
                    }).to_string())
                    .create()
            })
            .collect()
    }
    
    /// Setup a complete successful workflow with all necessary mocks
    pub fn setup_successful_workflow(&mut self, deck_name: &str, card_count: usize) -> WorkflowMocks {
        let connection = self.mock_successful_connection();
        let model_check = self.mock_model_exists(vec!["Basic", "Cloze"]); // Model doesn't exist
        let model_create = self.mock_create_model_success();
        let deck_create = self.mock_create_deck_success(deck_name);
        let card_additions = self.mock_multiple_card_additions(deck_name, card_count);
        
        WorkflowMocks {
            connection,
            model_check,
            model_create,
            deck_create,
            card_additions,
        }
    }
    
    /// Setup a workflow where the model already exists
    pub fn setup_existing_model_workflow(&mut self, deck_name: &str, card_count: usize) -> WorkflowMocks {
        let connection = self.mock_successful_connection();
        let model_check = self.mock_model_exists(vec!["Basic", "Cloze", "Wordcraft"]); // Model exists
        let deck_create = self.mock_create_deck_success(deck_name);
        let card_additions = self.mock_multiple_card_additions(deck_name, card_count);
        
        WorkflowMocks {
            connection,
            model_check,
            model_create: self.server.mock("POST", "/").create(), // Dummy mock, won't be called
            deck_create,
            card_additions,
        }
    }
}

pub struct WorkflowMocks {
    pub connection: Mock,
    pub model_check: Mock,
    pub model_create: Mock,
    pub deck_create: Mock,
    pub card_additions: Vec<Mock>,
}

impl WorkflowMocks {
    pub fn assert_all(&self) {
        self.connection.assert();
        self.model_check.assert();
        self.deck_create.assert();
        for mock in &self.card_additions {
            mock.assert();
        }
    }
    
    pub fn assert_with_model_creation(&self) {
        self.assert_all();
        self.model_create.assert();
    }
}

impl Drop for MockAnkiServer {
    fn drop(&mut self) {
        std::env::remove_var("ANKI_CONNECT_URL");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use autoflashcard::anki_adapter::AnkiAdapter;
    use serial_test::serial;
    
    #[tokio::test]
    #[serial]
    async fn test_mock_server_setup() {
        let mut mock_server = MockAnkiServer::new().await;
        let _connection_mock = mock_server.mock_successful_connection();
        
        let adapter = AnkiAdapter::new().expect("Failed to create adapter");
        let result = adapter.check_connection().await;
        
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    #[serial]
    async fn test_workflow_mocks() {
        let mut mock_server = MockAnkiServer::new().await;
        let mocks = mock_server.setup_successful_workflow("TestDeck", 2);
        
        let adapter = AnkiAdapter::new().expect("Failed to create adapter");
        
        // Test the workflow
        adapter.check_connection().await.expect("Connection failed");
        adapter.ensure_wordcraft_model_exists().await.expect("Model setup failed");
        adapter.create_deck("TestDeck").await.expect("Deck creation failed");
        
        // Add some cards
        for i in 0..2 {
            adapter.add_card(
                "TestDeck",
                &format!("front{}", i),
                &format!("back{}", i),
                &format!("example{}", i),
                &format!("translation{}", i),
            ).await.expect("Card addition failed");
        }
        
        mocks.assert_with_model_creation();
    }
}