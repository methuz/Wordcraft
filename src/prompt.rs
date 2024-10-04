use std::io::{self, Write};

// Struct to store user input for flashcard generation
pub struct FlashcardSettings {
    pub native_language: String,
    pub target_language: String,
    pub topic: String,
    pub deck_name: Option<String>,
}

impl FlashcardSettings {
    // Function to prompt user input for flashcard generation
    pub fn new() -> Self {
        let native_language = prompt_with_default(
            "Enter your native language (default: English): ",
            "English",
        );
        let target_language = prompt_with_default(
            "Enter the target language you want to learn (default: Japanese): ",
            "Japanese",
        );
        let topic = prompt_for_topic("Enter the topic you want to learn: ");
        let deck_name = prompt_existing_deck("Do you want to add to an existing deck? (y/N): ");

        FlashcardSettings {
            native_language,
            target_language,
            topic,
            deck_name,
        }
    }
}

// Function to prompt user for input with a default value
fn prompt_with_default(prompt: &str, default: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Ensure prompt is displayed immediately

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let trimmed_input = input.trim();

    if trimmed_input.is_empty() {
        default.to_string()
    } else {
        trimmed_input.to_string()
    }
}

// Function to prompt user for topic input and ensure it is not empty
fn prompt_for_topic(prompt: &str) -> String {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed_input = input.trim();

        if !trimmed_input.is_empty() {
            return trimmed_input.to_string();
        } else {
            println!("Topic cannot be empty. Please try again.");
        }
    }
}

// Function to prompt user if they want to add to an existing deck and get deck name if yes
fn prompt_existing_deck(prompt: &str) -> Option<String> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let trimmed_input = input.trim().to_lowercase();

    if trimmed_input == "y" || trimmed_input == "yes" {
        print!("Enter the name of the existing deck: ");
        io::stdout().flush().unwrap();

        let mut deck_name = String::new();
        io::stdin().read_line(&mut deck_name).unwrap();
        let deck_name_trimmed = deck_name.trim();

        if !deck_name_trimmed.is_empty() {
            Some(deck_name_trimmed.to_string())
        } else {
            println!("Deck name cannot be empty. Skipping adding to existing deck.");
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flashcard_settings() {
        // This test could be enhanced by using mocking techniques for user input
        let settings = FlashcardSettings {
            native_language: "English".to_string(),
            target_language: "Japanese".to_string(),
            topic: "Vocabulary".to_string(),
            deck_name: None,
        };

        assert_eq!(settings.native_language, "English");
        assert_eq!(settings.target_language, "Japanese");
        assert_eq!(settings.topic, "Vocabulary");
        assert!(settings.deck_name.is_none());
    }
}
