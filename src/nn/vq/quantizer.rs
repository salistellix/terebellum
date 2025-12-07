use burn::{
    Tensor,
    config::Config,
    module::Module,
    nn::{Embedding, EmbeddingConfig},
    prelude::Backend,
    tensor::Int,
};

#[derive(Config, Debug, Copy)]
pub struct QuantizerConfig {
    pub codebook_size: usize,
    pub latent_dimension: usize,
}

#[derive(Module, Debug)]
pub struct Quantizer<B: Backend> {
    codebook: Embedding<B>,
}

impl QuantizerConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> Quantizer<B> {
        let codebook = EmbeddingConfig::new(self.codebook_size, self.latent_dimension).init(device);
        Quantizer { codebook }
    }
}

#[derive(Clone, Debug)]
pub struct QuantizerOutput<B: Backend, const D: usize> {
    /// Straight-through quantized latents (same shape as input).
    pub latents: Tensor<B, D>,
    /// Codebook indices for each latent vector (flattened).
    pub indices: Tensor<B, 2, Int>,
    /// Codebook loss: ||sg(z)  - e||^2
    pub codebook_loss: Tensor<B, 1>,
    /// Commitment loss: commitment_beta * ||z - sg(e)||^2
    pub commitment_loss: Tensor<B, 1>,
}

impl<B: Backend> Quantizer<B> {
    /// Applies vector quantization on the provided tensor, as described in the
    /// VQ-VAE paper.
    ///
    /// # Arguments
    /// - `latents`: The tensor to be vector quantized
    /// - `commitment_beta`: *Beta* in the VQ-VAE paper (usually ~0.25)
    ///
    /// # Shapes
    /// - `latents`: (*, latent_dimension)
    pub fn forward<const D: usize>(&self, latents: Tensor<B, D>, commitment_beta: f32) -> QuantizerOutput<B, D> {
        // Get codebook: [codebook_size, latent_dimensions]
        let codebook = (*self.codebook.weight).clone();

        // Original shape: [..., latent_dim]
        let dimensions = latents.dims();
        let latent_dimension = dimensions[D - 1];
        let vector_count: usize = dimensions[..D - 1].iter().product();

        // Flatten the latent to [vector_count, latent_dimension]
        let flattened_latent = latents.clone().reshape([vector_count, latent_dimension]);

        // Broadcast to [vector_count, codebook_size, latent_dimension]
        let z = flattened_latent.unsqueeze_dim::<3>(1); // [vector_count, 1, latent_dimension]
        let e = codebook.unsqueeze_dim::<3>(0); // [1, codebook_size, latent_dimension]

        // Calculate squared L2 distances over last dimension: [vector_count, codebook_size]
        let difference = z - e;
        let squared_difference = difference.powi_scalar(2).sum_dim(2).squeeze::<2>();

        // Smallest distance along the codebook axis: indices in [0, codebook_size]
        let indices_2d = squared_difference.argmin(1); // Shape: [vector_count, 1] (Int)

        // Look up quantized vectors via the embedding, restore their shape
        let quantized_latents_3d = self.codebook.forward(indices_2d.clone()); // [vector_count, 1, latent_dimension]
        let flattened_quantized_latents = quantized_latents_3d.squeeze::<2>(); // [vector_count, latent_dimension]
        let quantized_latents = flattened_quantized_latents.reshape(dimensions);

        // Codebook loss: ||sg[z] - e||^2, where sg[z] = latent.detach()
        let codebook_loss = (latents.clone().detach() - quantized_latents.clone())
            .powi_scalar(2)
            .mean();

        // Commitment loss: ||z - sg[e]||^2, where sg[e] = quantized_latents.detach()
        let commitment_loss = (latents.clone() - quantized_latents.clone().detach())
            .powi_scalar(2)
            .mean()
            .mul_scalar(commitment_beta);

        // Straight-through estimator
        let latents_ste = latents.clone() + (quantized_latents - latents).detach();

        QuantizerOutput {
            latents: latents_ste,
            indices: indices_2d,
            codebook_loss,
            commitment_loss,
        }
    }
}
