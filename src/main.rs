use autoflashcard::anki_adapter::AnkiAdapter;
use autoflashcard::open_ai_adapter::generate_flashcards;
use std::process;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let deck_name = "New Deck about Greetings";

    let adapter = AnkiAdapter::new().unwrap_or_else(|err| {
        eprintln!("Problem creating new adapter: {err}");
        process::exit(1);
    });


    let user_input = "Generate flashcard deck to teach me Japanese greetings";

    println!("Generating flashcards for: {}", user_input);
    let response = generate_flashcards(user_input).await?;

    println!("Inserting deck into Anki: {}", response.deck_name);
    
    adapter.create_deck(&response.deck_name).await?;

    println!("Inserting card into deck: {}", response.deck_name);
    for card in &response.cards {
        adapter
            .add_card(&response.deck_name, &card.front, &card.back, "", "")
            .await?;
    }

    Ok(())
}
