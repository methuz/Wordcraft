use std::process;
use dotenv::dotenv;

use autoflashcard::anki_adapter::AnkiAdapter;
use autoflashcard::open_ai_adapter::generate_flashcards;
use autoflashcard::prompt::FlashcardSettings;
use autoflashcard::prompt::ask_for_confirmation;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let settings = FlashcardSettings::new();

    let adapter = AnkiAdapter::new().unwrap_or_else(|err| {
        eprintln!("Problem creating new adapter: {err}");
        process::exit(1);
    });

    let complete_prompt = format!(
        "Native Language: {}\nTarget Language: {}\nTopic: {}\n",
        settings.native_language,
        settings.target_language,
        settings.topic
    );

    println!("Generating flashcards for:\n{}", &complete_prompt);

    let response = generate_flashcards(&complete_prompt).await?;

    response.cards.iter().for_each(|card| {
        println!("Front: {}\nBack: {}\nExample: {}\nExample Translation: {}\n", card.front, card.back, card.example, card.example_translate);
    });

    if ask_for_confirmation("Would you like to add these flashcards? (y/n)") {
        println!("Adding flashcards to Anki...");
    } else {
        println!("Exiting without adding flashcards.");
        process::exit(0);
    }

    // TODO: refactor this to use the prompt module, and add to existing deck if user chooses
    if let Some(deck_name) = &settings.deck_name {
        println!("Adding to existing deck: {}", deck_name);
        for card in &response.cards {
            adapter.add_card(deck_name, &card.front, &card.back, "", "").await?;
        }
    } else {
        println!("No existing deck provided.");
        adapter.create_deck(&response.deck_name).await?;
        
        println!("Inserting card into deck: {}", &response.deck_name);
        for card in &response.cards {
            adapter.add_card(&response.deck_name, &card.front, &card.back, "", "").await?;
        }
    }

    Ok(())
}
