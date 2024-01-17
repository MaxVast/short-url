use uuid::Uuid;
use sha1::{Digest, Sha1};

pub fn unique_code(limit: usize) -> String {
    let id = Uuid::new_v4();
    
    let sha1_digest = Sha1::digest(format!("{:x}", id).as_bytes());
    let base36_string = base_convert(sha1_digest.into());

    base36_string.chars().take(limit).collect()
}

fn base_convert(sha1_digest: [u8; 20]) -> String {
    let base36_chars: Vec<char> = "0123456789abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut result = String::new();

    for &byte in sha1_digest.iter() {
        result.push(base36_chars[(byte % 36) as usize]);
    }

    result
}