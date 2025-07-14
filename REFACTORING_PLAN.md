# Wordcraft Refactoring Plan

## Overview
This document outlines the refactoring plan to improve code quality, testability, and prepare for new features.

## Current Issues Identified

### 1. Error Handling
- [ ] Replace `panic!()` with proper error propagation in `langchain.rs:74`
- [ ] Add proper error types instead of using `Box<dyn std::error::Error>`
- [ ] Validate API keys and configuration before use
- [ ] Add timeout handling for all network operations

### 2. Configuration Management
- [ ] Create a centralized `Config` struct
- [ ] Implement configuration validation
- [ ] Add support for config files (TOML/JSON)
- [ ] Normalize environment variable names

### 3. Dependency Injection
- [ ] Make `AnkiAdapter` accept HTTP client as dependency
- [ ] Abstract LLM providers behind a trait
- [ ] Create mockable interfaces for testing

### 4. Code Structure
- [ ] Extract constants to a proper constants module
- [ ] Create domain models separate from API models
- [ ] Implement proper separation of concerns

## Refactoring Steps

### Phase 1: Foundation (High Priority)

#### 1.1 Create Error Types
```rust
// src/error.rs
#[derive(Debug, thiserror::Error)]
pub enum WordcraftError {
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Anki connection error: {0}")]
    AnkiConnection(String),
    
    #[error("LLM error: {0}")]
    LLM(String),
    
    #[error("Parsing error: {0}")]
    Parsing(String),
}
```

#### 1.2 Configuration Management
```rust
// src/config.rs
#[derive(Debug, Clone)]
pub struct Config {
    pub anki_connect_url: String,
    pub engine: LLMEngine,
    pub model: String,
    pub api_key: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, WordcraftError> {
        // Load and validate configuration
    }
}
```

#### 1.3 Traits for Abstraction
```rust
// src/traits.rs
#[async_trait]
pub trait AnkiClient {
    async fn check_connection(&self) -> Result<(), WordcraftError>;
    async fn create_deck(&self, name: &str) -> Result<(), WordcraftError>;
    async fn add_card(&self, deck: &str, card: &Card) -> Result<(), WordcraftError>;
}

#[async_trait]
pub trait LLMProvider {
    async fn generate_flashcards(&self, prompt: &str) -> Result<FlashcardResponse, WordcraftError>;
}
```

### Phase 2: Implementation (Medium Priority)

#### 2.1 Refactor AnkiAdapter
- [ ] Implement `AnkiClient` trait
- [ ] Accept HTTP client as dependency
- [ ] Add proper error handling
- [ ] Implement retry logic

#### 2.2 Refactor LLM Integration
- [ ] Create separate providers for OpenAI and Ollama
- [ ] Implement `LLMProvider` trait
- [ ] Add prompt templates system
- [ ] Implement response caching

#### 2.3 Improve User Interface
- [ ] Create `UserInterface` trait for testability
- [ ] Implement progress indicators
- [ ] Add colored output support
- [ ] Implement input validation

### Phase 3: New Features (Low Priority)

#### 3.1 Batch Operations
- [ ] Implement batch card addition
- [ ] Add concurrent processing
- [ ] Implement progress tracking

#### 3.2 Card Management
- [ ] Add card editing capabilities
- [ ] Implement duplicate detection
- [ ] Add card filtering options

#### 3.3 Advanced Features
- [ ] Implement card templates
- [ ] Add audio generation support
- [ ] Implement image support
- [ ] Add export/import functionality

## Testing Strategy

### Current Test Coverage
- ✅ Unit tests for core modules
- ✅ Integration tests for workflows
- ✅ Mock server utilities
- ✅ Error scenario testing

### Planned Test Improvements
- [ ] Add property-based testing
- [ ] Implement benchmarking tests
- [ ] Add load testing for batch operations
- [ ] Create end-to-end tests

## API Design Changes

### Current API Issues
- Direct coupling between modules
- No clear separation of concerns
- Inconsistent error handling

### Proposed API Structure
```rust
// Main application service
pub struct WordcraftService {
    anki_client: Box<dyn AnkiClient>,
    llm_provider: Box<dyn LLMProvider>,
    config: Config,
}

impl WordcraftService {
    pub async fn generate_and_add_flashcards(
        &self,
        request: FlashcardRequest,
    ) -> Result<FlashcardResponse, WordcraftError> {
        // Orchestrate the entire workflow
    }
}
```

## Migration Strategy

### Step 1: Prepare
- [ ] Create comprehensive tests
- [ ] Document current behavior
- [ ] Set up CI/CD pipeline

### Step 2: Refactor Incrementally
- [ ] Implement new error types
- [ ] Add configuration management
- [ ] Create abstract interfaces

### Step 3: Migrate Modules
- [ ] Refactor `anki_adapter` module
- [ ] Refactor `langchain` module
- [ ] Refactor `prompt` module

### Step 4: Integrate and Test
- [ ] Update main application
- [ ] Run comprehensive tests
- [ ] Benchmark performance

## Performance Considerations

### Current Bottlenecks
- Sequential card addition
- No caching of LLM responses
- Blocking I/O operations

### Planned Optimizations
- [ ] Implement async batch operations
- [ ] Add response caching
- [ ] Optimize JSON parsing
- [ ] Add connection pooling

## Breaking Changes

### Version 0.2.0 (Planned)
- New error types
- Configuration file support
- Improved CLI interface

### Version 0.3.0 (Future)
- Plugin system
- GUI implementation
- Advanced features

## Timeline

### Week 1-2: Foundation
- Implement error types
- Add configuration management
- Create abstract interfaces

### Week 3-4: Core Refactoring
- Refactor existing modules
- Implement new features
- Update tests

### Week 5-6: Integration & Testing
- Integrate all changes
- Comprehensive testing
- Performance optimization

## Success Metrics

### Code Quality
- [ ] 90%+ test coverage
- [ ] Zero clippy warnings
- [ ] Consistent error handling

### Performance
- [ ] <100ms response time for card addition
- [ ] Support for 1000+ cards in batch
- [ ] Efficient memory usage

### User Experience
- [ ] Clear error messages
- [ ] Progress indicators
- [ ] Intuitive CLI interface