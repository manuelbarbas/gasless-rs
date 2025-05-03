#[macro_use] extern crate tslink;
use tslink::tslink;
use node_bindgen::derive::node_bindgen;
use alloy_primitives::{hex, Address, B256, U256};
use rand::Rng;
use std::time::{Instant, Duration};

#[tslink(rename = "mineGasForTransaction")]
#[node_bindgen(name = "mineGasForTransaction")]
pub async fn mine_gas_for_transaction(gas_amount: u32, address: String, nonce: u32) -> Result<MiningOutput, String> {
    // Function implementation remains the same
    // Validate and parse input
    if !is_address(&address) {
        return Err("Invalid Address".to_string());
    }
    
    let address = Address::parse_checksummed(&address, None)
        .map_err(|_| "Invalid address format".to_string())?;
    
    // Perform mining
    let result = mine_free_gas(gas_amount, address, nonce).unwrap();
    
    Ok(MiningOutput {
        duration: result.0.as_secs_f64() * 1000.0, // Convert to milliseconds
        gas_price: format!("0x{}", hex::encode(result.1.to_be_bytes::<32>())),
    })
}

// Export struct with JavaScript-friendly name
#[tslink]
#[node_bindgen]
pub struct MiningOutput {
    pub duration: f64,
    pub gas_price: String,
}

// Helper functions remain the same
fn is_address(value: &str) -> bool {
    if !value.starts_with("0x") || value.len() != 42 {
        return false;
    }
    value[2..].chars().all(|c| c.is_ascii_hexdigit())
}

fn mine_free_gas(gas_amount: u32, address: Address, nonce: u32) -> Result<(Duration, U256), String> {
    // Implementation remains the same
    // Calculate nonce hash
    let nonce_bytes = U256::from(nonce).to_be_bytes::<32>();
    let nonce_hash = U256::from_be_bytes(B256::from(keccak256(&nonce_bytes)).0);
    
    // Calculate address hash
    let address_hash = U256::from_be_bytes(B256::from(keccak256(address.as_slice())).0);
    
    // XOR nonce and address hashes
    let nonce_address_xor = nonce_hash ^ address_hash;
    
    // Max U256 value
    let div_constant = U256::MAX;
    
    let start = Instant::now();
    let mut rng = rand::thread_rng();
    let mut iterations = 0;
    
    // Mining loop
    loop {
        // Generate random candidate
        let mut candidate_bytes = [0u8; 32];
        rng.fill(&mut candidate_bytes);
        let candidate = U256::from_be_bytes(candidate_bytes);
        
        // Calculate candidate hash
        let candidate_hash = U256::from_be_bytes(B256::from(keccak256(&candidate.to_be_bytes::<32>())).0);
        
        // XOR with previous result
        let result_hash = nonce_address_xor ^ candidate_hash;
        
        // Avoid division by zero
        if result_hash == U256::ZERO {
            continue;
        }
        
        // Calculate external gas
        let external_gas = div_constant / result_hash;
        
        // Check if we found a solution
        if external_gas >= U256::from(gas_amount) {
            let duration = start.elapsed();
            return Ok((duration, candidate));
        }
        
        // Yield to the event loop every 5000 iterations
        iterations += 1;
        if iterations % 5000 == 0 {
            std::thread::yield_now();
        }
    }
}

fn keccak256(data: impl AsRef<[u8]>) -> [u8; 32] {
    use sha3::{Digest, Keccak256};
    let mut hasher = Keccak256::new();
    hasher.update(data);
    hasher.finalize().into()
}

// Add a simple test for the API
// #[cfg(test)]
// mod tests {
//     use super::*;
    
//     #[test]
//     fn test_is_address() {
//         assert!(is_address("0x1234567890123456789012345678901234567890"));
//         assert!(!is_address("0x12345"));
//         assert!(!is_address("not_an_address"));
//     }
    
//     #[tokio::test]
//     async fn test_gas_mining() {
//         let address = "0x742d35Cc6634C0532925a3b844Bc454e4438f44e".to_string();
//         let result = mine_gas_for_transaction(21000, address, 42).await;
//         assert!(result.is_ok());
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_address() {
        assert!(is_address("0x1234567890123456789012345678901234567890"));
        assert!(!is_address("0x12345"));
        assert!(!is_address("not_an_address"));
    }
    
    #[test]
    fn test_basic_mining() {
        let address = Address::parse_checksummed("0x1234567890123456789012345678901234567890", None).unwrap();
        let result = mine_free_gas(21000, address, 1);
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_skale_pow_mining() {
        let from_address = Address::parse_checksummed("0x742d35Cc6634C0532925a3b844Bc454e4438f44e", None).unwrap();
        let nonce = 42;
        let gas = 21000;
        
        println!("Testing SKALE PoW gas mining");
        
        let start = Instant::now();
        let result = mine_free_gas(gas, from_address, nonce).unwrap();
        
        let elapsed = start.elapsed();
        println!("Mining completed successfully!");
        println!("Gas price: 0x{}", hex::encode(result.1.to_be_bytes::<32>()));
        println!("Mining took: {} seconds", result.0.as_secs_f64());
        println!("Actual elapsed time: {:.2} seconds", elapsed.as_secs_f64());
        
        assert!(result.0.as_secs_f64() > 0.0, "Duration should be positive");
    }
}
// use node_bindgen::derive::node_bindgen;
// use alloy_primitives::{hex, Address, B256, U256};
// use rand::Rng;
// use std::time::{Instant, Duration};

// #[node_bindgen]
// pub struct MiningInput {
//     pub nonce: String,
//     pub gas: String,
//     pub from: String,
// }

// /// Output of mining operation
// #[node_bindgen]
// pub struct MiningOutput {
//     pub duration: f64,
//     pub gas_price: String,
// }

// /// Convert a string to a number, supporting both numeric strings and hex strings
// fn to_number(value: &str) -> u64 {
//     if value.starts_with("0x") {
//         let hex_value = value.trim_start_matches("0x");
//         u64::from_str_radix(hex_value, 16).map_err(|e| Error::from_reason(format!("Invalid hex: {}", e)))
//     } else {
//         value.parse::<u64>().map_err(|e| Error::from_reason(format!("Invalid number: {}", e)))
//     }
// }

// /// Check if a string is a valid Ethereum address
// fn is_address(value: &str) -> bool {
//     if !value.starts_with("0x") || value.len() != 42 {
//         return false;
//     }
    
//     value[2..].chars().all(|c| c.is_ascii_hexdigit())
// }

// /// Mine gas for a transaction
// #[node_bindgen]
// pub async fn mine_gas_for_transaction(gas_amount: u32, address: String, nonce: u32) -> Result<MiningOutput> {
//     // Validate and parse input
//     if !is_address(address.as_str()) {
//         return Err("Invalid Address");
//     }
    
//     let nonce = to_number(&input.nonce)?;
//     let gas = to_number(&input.gas)?;
//     let address = Address::parse_checksummed(&input.from, None)
//         .map_err(|_| Error::from_reason("Invalid address format"))?;
    
//     // Perform mining
//     // let result = mine_free_gas(gas, address, nonce)?;
//     let result = tokio::task::spawn_blocking(move || {
//         mine_free_gas(gas, address, nonce)
//     })
//     .await
//     .map_err(|e| Error::from_reason(format!("Join error: {}", e)))??;
    
//     Ok(MiningOutput {
//         duration: result.0.as_secs_f64() * 1000.0, // Convert to milliseconds
//         gas_price: format!("0x{}", hex::encode(result.1.to_be_bytes::<32>())),
//     })
// }

// /// Internal function to mine free gas
// fn mine_free_gas(gas_amount: u64, address: Address, nonce: u64) -> Result<(Duration, U256)> {
//     // Calculate nonce hash
//     let nonce_bytes = U256::from(nonce).to_be_bytes::<32>();
//     let nonce_hash = U256::from_be_bytes(B256::from(keccak256(&nonce_bytes)).0);
    
//     // Calculate address hash by encoding the address manually
//     let mut address_bytes = [0u8; 20];
//     address_bytes.copy_from_slice(address.as_slice());
//     let address_hash = U256::from_be_bytes(B256::from(keccak256(address_bytes)).0);
    
//     // XOR nonce and address hashes
//     let nonce_address_xor = nonce_hash ^ address_hash;
    
//     // Max U256 value
//     let div_constant = U256::MAX;
    
//     let start = Instant::now();
//     let mut rng = rand::thread_rng();
//     let mut iterations = 0;
    
//     // Mining loop
//     loop {
//         // Generate random candidate
//         let mut candidate_bytes = [0u8; 32];
//         rng.fill(&mut candidate_bytes);
//         let candidate = U256::from_be_bytes(candidate_bytes);
        
//         // Calculate candidate hash
//         let candidate_hash = U256::from_be_bytes(B256::from(keccak256(&candidate.to_be_bytes::<32>())).0);
        
//         // XOR with previous result
//         let result_hash = nonce_address_xor ^ candidate_hash;
        
//         // Avoid division by zero
//         if result_hash == U256::ZERO {
//             continue;
//         }
        
//         // Calculate external gas
//         let external_gas = div_constant / result_hash;
        
//         // Check if we found a solution
//         if external_gas >= U256::from(gas_amount) {
//             let duration = start.elapsed();
//             return Ok((duration, candidate));
//         }
        
//         // Yield to the event loop every 5000 iterations
//         iterations += 1;
//         if iterations % 5000 == 0 {
//             // In native Rust, we don't need to yield, but we could add a short sleep
//             // for cooperative multitasking if needed
//             std::thread::yield_now();
//         }
//     }
// }

// /// Keccak256 hash function
// fn keccak256(data: impl AsRef<[u8]>) -> [u8; 32] {
//     use sha3::{Digest, Keccak256};
//     let mut hasher = Keccak256::new();
//     hasher.update(data);
//     hasher.finalize().into()
// }

// #[cfg(test)]
// mod tests {
//   use super::*;
//   use std::time::Instant;
    
//     #[test]
//     fn test_is_address() {
//         assert!(is_address("0x1234567890123456789012345678901234567890"));
//         assert!(!is_address("0x12345"));
//         assert!(!is_address("not_an_address"));
//     }
    
//     #[test]
//     fn test_to_number() {
//         assert_eq!(to_number("123").unwrap(), 123);
//         assert_eq!(to_number("0x7B").unwrap(), 123);
//     }
    
//     #[test]
//     fn test_basic_mining() {
//         let address = Address::parse_checksummed("0x1234567890123456789012345678901234567890", None).unwrap();
//         let result = mine_free_gas(21000, address, 1);
//         assert!(result.is_ok());
//     }
    
//     #[tokio::test]
//     async fn test_skale_pow_mining() {
//         // Test parameters matching the JavaScript test
//         let from_address = Address::parse_checksummed("0x742d35Cc6634C0532925a3b844Bc454e4438f44e", None).unwrap();
//         let nonce = 42;
//         let gas = 21000;
        
//         println!("Testing SKALE PoW gas mining");
//         // println!("Using address: {}", from_address);
//         // println!("Nonce: {}, Gas: {}", nonce, gas);
        
//         // Start timing
//         let start = Instant::now();
        
//         // Perform mining
//         let result = mine_free_gas(gas, from_address, nonce).await.expect("Mining should succeed");
        
//         // Check results
//         let elapsed = start.elapsed();
//         println!("Mining completed successfully!");
//         println!("Gas price: {}", result.gas_price);
//         println!("Mining took: {} seconds", result.duration);
//         println!("Actual elapsed time: {:.2} seconds", elapsed.as_secs_f64());
        
//         // Basic validation
//         assert!(result.gas_price.starts_with("0x"), "Gas price should be a hex string");
//         assert!(result.duration > 0.0, "Duration should be positive");
//     }
// }