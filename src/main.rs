use autoflashcard::AnkiAdapter;
use std::process;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let deck_name = "RustDeck";

    let adapter = AnkiAdapter::new().unwrap_or_else(|err| {
        eprintln!("Problem creating new adapter: {err}");
        process::exit(1);
    });

    let front = "What is Rust?";
    let back = "A systems programming";

    adapter.create_deck(deck_name).await?;
    adapter.add_card(deck_name, front, back).await?;

    Ok(())
}
