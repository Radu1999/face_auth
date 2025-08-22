use anyhow::Result;
use candle_core::Tensor;
use candle_nn::Func;

// Import functions from ex02_embeddings solution
use ex02_embeddings_solution::{build_model, compute_embedding};

fn normalize_l2(v: &Tensor) -> Result<Tensor> {
    unimplemented!("TODO: normalize the tensor using L2 normalization")
}
/// Exercise: Implement cosine similarity using the app utilities and verify thresholds.
pub fn cosine_similarity(_emb_a: &Tensor, _emb_b: &Tensor) -> Result<f32> {
    unimplemented!("TODO: use face_auth::embeddings::embeddings::compute_similarity")
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::Device;
    use candle_nn::Func;
    use face_auth::image_utils::imagenet;

    #[test]
    fn same_person_higher_similarity() -> Result<()> {
        let device = &Device::Cpu;
        let img1 = imagenet::load_image224("../../app/test_images/brad1.png")?.to_device(device)?;
        let img2 = imagenet::load_image224("../../app/test_images/brad2.png")?.to_device(device)?;
        let img3 = imagenet::load_image224("../../app/test_images/tom.png")?.to_device(device)?;
        let model: Func = build_model()?;
        let e1 = compute_embedding(&model, &img1)?;
        let e2 = compute_embedding(&model, &img2)?;
        let e3 = compute_embedding(&model, &img3)?;
        let s_same = cosine_similarity(&e1, &e2)?;
        let s_diff = cosine_similarity(&e1, &e3)?;
        assert!(s_same > s_diff);
        Ok(())
    }
}


