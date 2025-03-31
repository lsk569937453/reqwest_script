mod getbinancebalance;

fn main() {
    match getbinancebalance::get_binancetest_balance() {
        Ok(balances) => {
            for i in 0..balances.len() {
                let amount = balances[i]["free"]
                    .as_str()
                    .unwrap()
                    .parse::<f32>()
                    .unwrap();
                if amount > 0.0 {
                    println!("{}: {}", balances[i]["asset"], amount);
                }
            }
            println!("Finished");
        }
        Err(_) => {
            println!("Failed to grab price");
        }
    }
}
