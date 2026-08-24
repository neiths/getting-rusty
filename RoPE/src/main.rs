use burn::backend::NdArray;
use burn::config::Config;
use burn::module::Module;
use burn::tensor::backend::Backend;
use burn::tensor::Tensor;

/// Configuration for Rotary Positional Embedding (RoPE)
#[derive(Config, Debug)]
pub struct RotaryEmbeddingConfig {
    pub d_model: usize,
    #[config(default = 2048)]
    pub max_seq_len: usize,
    #[config(default = 10000.0)]
    pub theta: f64,
}

impl RotaryEmbeddingConfig {
    /// Constructor with validation logic
    pub fn init<B: Backend>(&self, device: &B::Device) -> RotaryPositionalEmbedding<B> {
        assert!(
            self.d_model % 2 == 0,
            "d_model ({}) must be even for RoPE.",
            self.d_model
        );

        let half_dim = self.d_model / 2;

        // 1. Calculate inverse frequencies: theta^(-2i / d_model)
        let mut inv_freq_vec = Vec::with_capacity(half_dim);
        for i in 0..half_dim {
            let exponent = (2.0 * i as f64) / (self.d_model as f64);
            let freq = 1.0 / self.theta.powf(exponent);
            inv_freq_vec.push(freq as f32);
        }

        // 2. Precompute outer product: positions * inv_freq
        let mut emb_data = Vec::with_capacity(self.max_seq_len * self.d_model);
        for pos in 0..self.max_seq_len {
            for &freq in &inv_freq_vec {
                let angle = (pos as f32) * freq;
                // Repeat interleave by 2 (theta_i, theta_i)
                emb_data.push(angle);
                emb_data.push(angle);
            }
        }

        // 3. Create tensors and compute cos / sin
        let emb_tensor =
            Tensor::<B, 1>::from_floats(emb_data.as_slice(), device)
                .reshape([self.max_seq_len, self.d_model]);

        let cos_cached = emb_tensor.clone().cos();
        let sin_cached = emb_tensor.sin();

        RotaryPositionalEmbedding {
            cos_cached,
            sin_cached,
        }
    }
}

/// Rotary Positional Embedding Module
#[derive(Module, Debug)]
pub struct RotaryPositionalEmbedding<B: Backend> {
    cos_cached: Tensor<B, 2>, // [max_seq_len, d_model]
    sin_cached: Tensor<B, 2>, // [max_seq_len, d_model]
}

impl<B: Backend> RotaryPositionalEmbedding<B> {
    /// Static Helper: Rotates pairs of dimensions [-x1, x0, -x3, x2, ...]
    pub fn rotate_half(x: Tensor<B, 4>) -> Tensor<B, 4> {
        let [batch, heads, seq_len, head_dim] = x.dims();
        let half_dim = head_dim / 2;

        let x_pairs = x.reshape([batch, heads, seq_len, half_dim, 2]);

        let x_even = x_pairs.clone().slice([0..batch, 0..heads, 0..seq_len, 0..half_dim, 0..1]);
        let x_odd = x_pairs.slice([0..batch, 0..heads, 0..seq_len, 0..half_dim, 1..2]);

        let neg_x_odd = x_odd.neg();
        let rotated = Tensor::cat(vec![neg_x_odd, x_even], 4);

        rotated.reshape([batch, heads, seq_len, head_dim])
    }

    /// Forward pass: Applies RoPE to queries (Q) or keys (K)
    pub fn forward(&self, x: Tensor<B, 4>, seq_len: usize) -> Tensor<B, 4> {
        let [_batch, _heads, _seq, head_dim] = x.dims();

        let cos = self.cos_cached.clone().slice([0..seq_len, 0..head_dim]);
        let sin = self.sin_cached.clone().slice([0..seq_len, 0..head_dim]);

        let cos = cos.reshape([1, 1, seq_len, head_dim]);
        let sin = sin.reshape([1, 1, seq_len, head_dim]);

        let x_rotated = Self::rotate_half(x.clone());
        (x * cos) + (x_rotated * sin)
    }
}

fn main() {
    type MyBackend = NdArray<f32>;
    let device = Default::default();

    // 1. Initialize RoPE module (head_dim = 64, max_seq_len = 2048)
    let config = RotaryEmbeddingConfig::new(64);
    let rope = config.init::<MyBackend>(&device);

    // 2. Create a dummy Query tensor [batch=1, heads=8, seq_len=4, head_dim=64]
    let q = Tensor::<MyBackend, 4>::zeros([1, 8, 4, 64], &device);

    // 3. Apply RoPE
    let q_rotated = rope.forward(q, 4);

    println!("Input shape:   [1, 8, 4, 64]");
    println!("Rotated shape: {:?}", q_rotated.dims());
}