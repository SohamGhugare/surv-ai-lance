use sha2::{Digest, Sha256};

pub fn calculate_hash(id: u64, timestamp: i64, previous_hash: &str, data: &str) -> String {
    let data = serde_json::json!({
        "id": id,
        "previous_hash": previous_hash,
        "data": data,
        "timestamp": timestamp,
    });
    let mut hasher = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    let hash = hasher.finalize().as_slice().to_owned();
    hex::encode(hash)
}
