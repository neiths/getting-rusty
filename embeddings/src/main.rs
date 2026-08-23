use burn::config::Config;
use burn::module::Module;
use burn::nn::{Embedding as BurnEmbedding, EmbeddingConfig};
use burn::tensor::backend::Backend;
use burn::tensor::{Int, Tensor};

/// Configuration for the Embedding layer
#[derive(Config, Debug)]
pub struct EmbeddingLayerConfig {
    pub vocab_size: usize,
    pub d_model: usize,
}

impl EmbeddingLayerConfig {
    /// Initialize the Embedding module
    pub fn init<B: Backend>(&self, device: &B::Device) -> Embedding<B> {
        let embed = EmbeddingConfig::new(self.vocab_size, self.d_model).init(device);
        Embedding { embed }
    }
}

/// Embedding Module wrapping Burn's Embedding layer
#[derive(Module, Debug)]
pub struct Embedding<B: Backend> {
    embed: BurnEmbedding<B>,
}

impl<B: Backend> Embedding<B> {
    /// Look up embeddings for token IDs
    ///
    /// # Arguments
    /// * `x` - Tensor of shape `[batch_size, seq_len]` (integer token IDs)
    ///
    /// # Returns
    /// * Tensor of shape `[batch_size, seq_len, d_model]`
    pub fn forward(&self, x: Tensor<B, 2, Int>) -> Tensor<B, 3> {
        self.embed.forward(x)
    }
}

fn main() {
    use burn::backend::NdArray;

    type MyBackend = NdArray<f32>;
    let device = Default::default();

    let vocab_size = 50257;
    let d_model = 768;

    let config = EmbeddingLayerConfig::new(vocab_size, d_model);
    let embedding_layer = config.init::<MyBackend>(&device);

    // Simulated input: batch_size = 1, seq_len = 2 -> ["The", "cat"] = [464, 3797]
    let token_ids = Tensor::<MyBackend, 2, Int>::from_data([[464, 3797]], &device);

    // Forward pass
    let output = embedding_layer.forward(token_ids);

    println!("Input shape:  [1, 2]");
    println!("Output shape: {:?}", output.dims()); // [1, 2, 768]
}