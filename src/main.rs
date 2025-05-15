mod miner {
    pub mod miner_no_threads;
    pub mod miner_with_threads;
}

#[tokio::main]
async fn main() {

    let mut total_duration_no_threads = 0.0;
    let mut total_duration_with_threads = 0.0;
    let bench_iterations = 200;

    for _i in 1..=bench_iterations {

        let address = "0x742d35Cc6634C0532925a3b844Bc454e4438f44e".to_string();
        let nonce = 42;
        let gas = 21000;

        let result = miner::miner_no_threads::mine_gas_for_transaction(gas,&address,nonce).await;

        println!("No Threads Used");

        match result {
            Ok(output) => {
                total_duration_no_threads += output.duration;
                println!("Duration: {} ms", output.duration);
                println!("Found gas price: {}", output.gas_price);
            },
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }


        let result2 = miner::miner_with_threads::mine_gas_for_transaction(gas,&address,nonce).await;

        println!("With Threads Used");

        match result2 {
            Ok(output) => {
                total_duration_with_threads += output.duration;
                println!("Duration: {} ms", output.duration);
                println!("Found gas price: {}", output.gas_price);
            },
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
        }

        let avg_duration_no_threads = total_duration_no_threads / bench_iterations as f64;
        let avg_duration_with_threads = total_duration_with_threads / bench_iterations as f64;
        println!("Number of runs: {}", bench_iterations);
        println!("Average Duration No Threads: {} ms", avg_duration_no_threads);
        println!("Average Duration With Threads: {} ms", avg_duration_with_threads);

}