use node_bindgen::derive::node_bindgen;
use alloy_primitives::{hex, Address, B256, U256};
use rand::Rng;
use std::time::{Instant, Duration};
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};

#[node_bindgen(name = "mineGasForTransaction")]
pub async fn mine_gas_for_transaction(gas_amount: u32, address: String, nonce: u32) -> Result<MiningOutput, String> {
    // Validate and parse input
    if !is_address(&address) {
        return Err("Invalid Address".to_string());
    }
    
    let address = Address::parse_checksummed(&address, None)
        .map_err(|_| "Invalid address format".to_string())?;
    
    // Perform mining
    let result = mine_free_gas_parallel(gas_amount, address, nonce).map_err(|e| e.to_string())?;
    
    Ok(MiningOutput {
        duration: result.0.as_secs_f64() * 1000.0, // Convert to milliseconds
        gas_price: format!("0x{}", hex::encode(result.1.to_be_bytes::<32>())),
    })
}

// Export struct with JavaScript-friendly name
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

fn mine_free_gas_parallel(gas_amount: u32, address: Address, nonce: u32) -> Result<(Duration, U256), String> {
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
    
    // Determine number of threads - use one less than available to avoid overwhelming the system
    let num_threads = thread::available_parallelism().map(|n| n.get()).unwrap_or(4).max(1);
    let num_threads = if num_threads > 1 { num_threads - 1 } else { 1 };
    
    // Shared flag to signal when a solution is found
    let solution_found = Arc::new(AtomicBool::new(false));
    
    // Shared storage for the solution
    let solution = Arc::new(Mutex::new(None));
    
    // Spawn worker threads
    let mut handles = vec![];
    
    println!("Number threads {}", num_threads);

    for _ in 0..num_threads {
        let solution_found_clone = solution_found.clone();
        let solution_clone = solution.clone();

        let handle: thread::JoinHandle<()> = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let mut iterations = 0;
            
            // Mining loop
            while !solution_found_clone.load(Ordering::Relaxed) {
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
                    // We found a solution, store it
                    let mut solution_guard = solution_clone.lock().unwrap();
                    *solution_guard = Some(candidate);
                    
                    // Signal other threads to stop
                    solution_found_clone.store(true, Ordering::Relaxed);
                    break;
                }
                
                // Yield to the scheduler occasionally to prevent CPU hogging
                iterations += 1;
                if iterations % 10000 == 0 {
                    if solution_found_clone.load(Ordering::Relaxed) {
                        break;
                    }
                    
                    // Give other threads a chance to run
                    thread::yield_now();
                }
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        let _ = handle.join();
    }
    
    // Check if a solution was found
    let duration = start.elapsed();
    
    let solution_guard = solution.lock().unwrap();
    if let Some(candidate) = *solution_guard {
        return Ok((duration, candidate));
    } else {
        return Err("No solution found".to_string());
    }
}

fn keccak256(data: impl AsRef<[u8]>) -> [u8; 32] {
    use sha3::{Digest, Keccak256};
    let mut hasher = Keccak256::new();
    hasher.update(data);
    hasher.finalize().into()
}

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
        let result = mine_free_gas_parallel(21000, address, 1);
        assert!(result.is_ok());
    }
    
    #[tokio::test]  
    async fn test_skale_pow_mining() {
        let from_address = Address::parse_checksummed("0x742d35Cc6634C0532925a3b844Bc454e4438f44e", None).unwrap();
        let nonce = 42;
        let gas = 21000;
        
        println!("Testing SKALE PoW gas mining");
        
        let start = Instant::now();
        let result = mine_free_gas_parallel(gas, from_address, nonce).unwrap();
        
        let elapsed = start.elapsed();
        println!("Mining completed successfully!");
        println!("Gas price: 0x{}", hex::encode(result.1.to_be_bytes::<32>()));
        println!("Mining took: {} seconds", result.0.as_secs_f64());
        println!("Actual elapsed time: {:.2} seconds", elapsed.as_secs_f64());
        
        assert!(result.0.as_secs_f64() > 0.0, "Duration should be positive");
    }
}
