use std::process;
use dotenv::dotenv;

use autoflashcard::anki_adapter::AnkiAdapter;
use autoflashcard::open_ai_adapter::generate_flashcards;
use autoflashcard::prompt::FlashcardSettings;

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
