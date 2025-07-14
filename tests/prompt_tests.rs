use autoflashcard::prompt::FlashcardSettings;
use std::io::Cursor;

#[test]
fn test_flashcard_settings_struct() {
    let settings = FlashcardSettings {
        native_language: "English".to_string(),
        target_language: "Spanish".to_string(),
        topic: "Colors".to_string(),
        deck_name: Some("Spanish Colors".to_string()),
    };
    
    assert_eq!(settings.native_language, "English");
    assert_eq!(settings.target_language, "Spanish");
    assert_eq!(settings.topic, "Colors");
    assert_eq!(settings.deck_name, Some("Spanish Colors".to_string()));
}

#[test]
fn test_flashcard_settings_with_no_deck() {
    let settings = FlashcardSettings {
        native_language: "English".to_string(),
        target_language: "French".to_string(),
        topic: "Animals".to_string(),
        deck_name: None,
    };
    
    assert_eq!(settings.native_language, "English");
    assert_eq!(settings.target_language, "French");
    assert_eq!(settings.topic, "Animals");
    assert!(settings.deck_name.is_none());
}

// Note: The following tests would require refactoring the prompt module to accept
// custom input/output streams for testing. Here's an example of how it could be done:

#[cfg(test)]
mod mock_tests {
    use super::*;
    use std::io::Cursor;
    
    // These tests demonstrate what we would test if the module was refactored
    // to accept input/output streams as parameters
    
    #[test]
    fn test_prompt_with_default_empty_input() {
        // This would test that empty input returns the default value
        // Expected: "English" for native language prompt
    }
    
    #[test]
    fn test_prompt_with_default_custom_input() {
        // This would test that non-empty input returns the user's input
        // Input: "Spanish\n"
        // Expected: "Spanish"
    }
    
    #[test]
    fn test_prompt_for_topic_empty_retry() {
        // This would test that empty topic input prompts for retry
        // Input: "\nColors\n"
        // Expected: "Colors" after retry message
    }
    
    #[test]
    fn test_prompt_existing_deck_yes() {
        // This would test deck prompt with "y" response
        // Input: "y\nMyDeck\n"
        // Expected: Some("MyDeck")
    }
    
    #[test]
    fn test_prompt_existing_deck_no() {
        // This would test deck prompt with "n" response
        // Input: "n\n"
        // Expected: None
    }
    
    #[test]
    fn test_ask_for_confirmation_yes() {
        // This would test confirmation with "y"
        // Input: "y\n"
        // Expected: true
    }
    
    #[test]
    fn test_ask_for_confirmation_no() {
        // This would test confirmation with "n"
        // Input: "n\n"
        // Expected: false
    }
    
    #[test]
    fn test_ask_for_confirmation_invalid_then_yes() {
        // This would test confirmation with invalid input then valid
        // Input: "invalid\ny\n"
        // Expected: true after retry
    }
}

// Example of how the prompt module could be refactored for better testability:
#[cfg(test)]
mod refactoring_example {
    use std::io::{BufRead, Write};
    
    // This is an example of how functions could be refactored to be testable
    fn prompt_with_default_testable<R: BufRead, W: Write>(
        reader: &mut R,
        writer: &mut W,
        prompt: &str,
        default: &str,
    ) -> std::io::Result<String> {
        write!(writer, "{}", prompt)?;
        writer.flush()?;
        
        let mut input = String::new();
        reader.read_line(&mut input)?;
        let trimmed = input.trim();
        
        Ok(if trimmed.is_empty() {
            default.to_string()
        } else {
            trimmed.to_string()
        })
    }
    
    #[test]
    fn test_prompt_with_default_using_mock_io() {
        let input = b"Spanish\n";
        let mut reader = std::io::Cursor::new(input);
        let mut writer = Vec::new();
        
        let result = prompt_with_default_testable(
            &mut reader,
            &mut writer,
            "Enter language: ",
            "English"
        ).unwrap();
        
        assert_eq!(result, "Spanish");
        assert_eq!(String::from_utf8(writer).unwrap(), "Enter language: ");
    }
}