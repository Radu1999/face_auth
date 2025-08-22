use anyhow::Result;
use ex03_similarity_solution::cosine_similarity_vec;
use ex04_storage_local_solution::{EmbeddingRecord, EmbeddingStorage};
use std::collections::HashMap;
use uuid::Uuid;

// Search for similar embeddings in any storage
pub fn search_similar(storage: &dyn EmbeddingStorage, embedding: &[f32], limit: usize) -> Result<Vec<(EmbeddingRecord, f32)>> {
    let records = storage.get_all_embeddings()?;
    let mut results = Vec::new();
    
    for record in records {
        let similarity = cosine_similarity_vec(embedding, &record.embedding);
        results.push((record, similarity));
    }
    
    // Sort by similarity (descending) and take top results
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    results.truncate(limit);
    
    Ok(results)
}

pub fn add_record(storage: &mut dyn EmbeddingStorage, name: &str, embedding: Vec<f32>) -> Result<String> {
    let record = EmbeddingRecord {
        id: Uuid::new_v4().to_string(),
        name: name.to_string(),
        embedding,
        created_at: chrono::Utc::now(),
        metadata: HashMap::new(),
    };
    
    let id = record.id.clone();
    storage.store_embedding(record)?;
    Ok(id)
}

// Get top-k most similar embeddings  
pub fn top_k(storage: &dyn EmbeddingStorage, query: &[f32], k: usize) -> Result<Vec<(EmbeddingRecord, f32)>> {
    search_similar(storage, query, k)
}