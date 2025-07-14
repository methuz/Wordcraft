#[cfg(test)]
pub mod test_helpers {
    use crate::{Flashcard, FlashcardResponse};
    
    pub fn create_test_flashcard(front: &str, back: &str) -> Flashcard {
        Flashcard {
            front: front.to_string(),
            back: back.to_string(),
            example: format!("Example with {}", front),
            example_translate: format!("Translation of example with {}", back),
        }
    }
    
    pub fn create_test_flashcard_response(deck_name: &str, count: usize) -> FlashcardResponse {
        let cards = (0..count)
            .map(|i| create_test_flashcard(&format!("Front {}", i), &format!("Back {}", i)))
            .collect();
            
        FlashcardResponse {
            deck_name: deck_name.to_string(),
            cards,
        }
    }
    
    pub fn create_anki_success_response() -> serde_json::Value {
        serde_json::json!({
            "result": null,
            "error": null
        })
    }
    
    pub fn create_anki_error_response(error: &str) -> serde_json::Value {
        serde_json::json!({
            "result": null,
            "error": error
        })
    }
    
    pub fn create_anki_add_note_response(note_id: i64) -> serde_json::Value {
        serde_json::json!({
            "result": note_id,
            "error": null
        })
    }
    
    pub fn create_model_names_response(models: Vec<&str>) -> serde_json::Value {
        serde_json::json!({
            "result": models,
            "error": null
        })
    }
}