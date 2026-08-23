use tiktoken_rs::{get_bpe_from_model, CoreBPE, Rank};

/// Configuration struct mirroring TokenizerConfig
#[derive(Debug, Clone)]
pub struct TokenizerConfig {
    pub name: String,
    pub vocab_size: usize,
}

impl Default for TokenizerConfig {
    fn default() -> Self {
        Self {
            name: "gpt2".to_string(),
            vocab_size: 50257,
        }
    }
}

/// Tokenizer wrapper around tiktoken-rs CoreBPE
pub struct SimpleTokenizer {
    pub config: TokenizerConfig,
    pub enc: CoreBPE,
    pub eos_token: &'static str,
    pub eos_token_id: Rank,
}

impl SimpleTokenizer {
    pub fn new(config: Option<TokenizerConfig>) -> Result<Self, Box<dyn std::error::Error>> {
        let config = config.unwrap_or_default();
        let enc = get_bpe_from_model(&config.name)?;

        let eos_token = "<|endoftext|>";

        // encode_with_special_tokens returns Vec<Rank> directly
        let tokens = enc.encode_with_special_tokens(eos_token);
        let eos_token_id = tokens
            .first()
            .copied()
            .ok_or("Failed to retrieve EOS token ID")?;

        Ok(Self {
            config,
            enc,
            eos_token,
            eos_token_id,
        })
    }

    /// Encode input text to a vector of token IDs (Rank)
    pub fn encode(&self, text: &str) -> Vec<Rank> {
        self.enc.encode_with_special_tokens(text)
    }

    /// Decode a slice of token IDs back into a UTF-8 String
    pub fn decode(&self, ids: &[Rank]) -> Result<String, Box<dyn std::error::Error>> {
        Ok(self.enc.decode(ids.to_vec())?)
    }

    /// Number of vocabulary items configured
    pub fn vocab_size(&self) -> usize {
        self.config.vocab_size
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tokenizer = SimpleTokenizer::new(None)?;

    // Test 1: Basic text
    let test_text = "The cat sat on the mat.";
    let encoded = tokenizer.encode(test_text);
    let decoded = tokenizer.decode(&encoded)?;
    println!("Test 1 — Basic:");
    println!("  Original: '{}'", test_text);
    println!("  Encoded:  {:?}", encoded);
    println!("  Decoded:  '{}'", decoded);
    println!("  Match:    {}", test_text == decoded);

    // Test 2: EOS token
    let eos = tokenizer.encode(tokenizer.eos_token);
    println!("\nTest 2 — EOS token:");
    println!("  String: '{}'", tokenizer.eos_token);
    println!("  Token ID: {}", tokenizer.eos_token_id);
    println!("  Encode result: {:?}", eos);

    // Test 3: Rare/unseen word
    let rare = tokenizer.encode("antidisestablishmentarianism");
    let decoded_rare = tokenizer.decode(&rare)?;
    let pieces: Vec<String> = rare
        .iter()
        .map(|&t| tokenizer.decode(&[t]).unwrap_or_default())
        .collect();
    println!("\nTest 3 — Rare word:");
    println!("  Encoded: {:?}", rare);
    println!("  Pieces:  {:?}", pieces);
    println!("  Decoded: '{}'", decoded_rare);

    // Test 4: Emoji/Unicode
    let emoji = tokenizer.encode("Hello 😊 world");
    println!("\nTest 4 — Emoji:");
    println!("  Encoded: {:?}", emoji);
    println!("  Decoded: '{}'", tokenizer.decode(&emoji)?);

    println!("\n  Vocab size: {}", tokenizer.vocab_size());

    Ok(())
}