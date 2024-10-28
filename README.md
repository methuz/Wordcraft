# Readme

## About Wordcraft

Wordcraft is an intelligent tool designed to streamline language learning by automatically generating customized Anki cards. Wordcraft leverages OpenAI’s API to create flashcards that suit your specific learning needs.

## Motivation

I find that many language learning software programs do not meet my needs. For example, Duolingo can be too easy for learning English and cannot be customized to focus on what I want to learn.

I’ve found that Anki is one of the best tools for learning any language. Anki uses cognitive science techniques such as active recall and spaced repetition to help users with memorization. The name comes from the Japanese word for "memorization."

Anki serves me well, and I find that many community-made decks help me get started with new languages. However, my main issue is that I often want decks in specific areas, such as colors in Japanese. To address this, I need to create my own decks, which is very time-consuming.

That’s why I created this project: to use the LLM's API (ChatGPT, Ollama) to automatically generate cards and add them to Anki.

I aim to create a graphical user interface (GUI) and implement Retrieval-Augmented Generation (RAG). The RAG feature will fetch known words from the user’s existing Anki decks, enabling the AI to assess the user’s language level and automatically generate lessons that are appropriately tailored to their needs.

This is my first public project built in Rust. Feel free to submit pull requests or request features.

## Milestone

 - [x] It works!
 - [x] Can add cards to existing deck
 - [x] Support any language
 - [x] Custom card type, Add example and solution
 - [x] Support local LLM
 - [ ] GUI
 - [ ] Automated generate audio
 - [ ] fetch word from Anki if existing deck is provided
 - [ ] Refactor & Test Coverage
 - [ ] Executable file (BIN)
 - [ ] Retrieval Augmented Generation (RAG) : Fetch user's known words from Anki and generate lesson using AI


### Anki

Anki can be used on Website at https://ankiweb.net/decks.
Anki download : https://apps.ankiweb.net/
Anki connect and how to install : https://ankiweb.net/shared/info/2055492159


You need your own OpenAPI Key to use this project
https://platform.openai.com/api-keys

### Create .env file

ANKI_CONNECT_URL=http://[Machine's IP]:[Port] (Optional)
OPEN_API_KEY={Your OPEN API KEY} 

### How to run on WSL

1. Config AnkiConnect to bind to 0.0.0.0
{
  "webBindAddress": "0.0.0.0",
  "webBindPort": 8765,
  // ... other settings
}

2. Add filewall rule
wf.msc

3. Find Windows IP
ip route | grep default | awk '{print $3}'

4. Manually set Windows IP in .env file
ANKI_CONNECT_URL=http://[Window IP]:8765


