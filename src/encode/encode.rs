use uuid::Uuid;
use sha1::{Digest, Sha1};

// Function to generate a unique code of a specified limit
pub fn unique_code(limit: usize) -> String {
    // Generate a new UUID
    let id = Uuid::new_v4();

    // Convert the UUID to a hexadecimal string and hash it using SHA-1
    let sha1_digest = Sha1::digest(format!("{:x}", id).as_bytes());

    // Convert the SHA-1 hash to a base36 string and truncate to the specified limit
    let base36_string = base_convert(sha1_digest.into());

    // Collect the first 'limit' characters from the base36 string
    base36_string.chars().take(limit).collect()
}

// Function to convert a SHA-1 digest to a base36 string
fn base_convert(sha1_digest: [u8; 20]) -> String {
    // Define the characters for base36 conversion
    let base36_chars: Vec<char> = "0123456789abcdefghijklmnopqrstuvwxyz".chars().collect();
    
    // Initialize an empty string to store the result
    let mut result = String::new();

    // Iterate over each byte in the SHA-1 digest
    for &byte in sha1_digest.iter() {
        // Append the corresponding base36 character to the result string
        result.push(base36_chars[(byte % 36) as usize]);
    }

    // Return the resulting base36 string
    result
}
