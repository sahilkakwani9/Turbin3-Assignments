#[cfg(test)]
mod tests {
    use solana_sdk::signature::Keypair;
    use solana_sdk::signature::Signer;
    use solana_sdk::signature::read_keypair_file;
    use solana_sdk::transaction::Transaction;
    use solana_client::rpc_client::RpcClient;
    use solana_program::{system_instruction::transfer, pubkey::Pubkey};
    use bs58;
    use std::io::{self, BufRead};
    use std::str::FromStr;

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn bs58_to_wallet(){
        let stdin = io::stdin();
        println!("Input your private key as base58:");
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("this is the user input base58 string {:?}", base58);
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_bs58(){
        let stdin = io::stdin();
        println!("Input your private key as a wallet file byte array:");
        let wallet = stdin.lock().lines().next().unwrap().unwrap().trim_start_matches("[").trim_end_matches("]").split(",").map(|s| s.trim().parse::<u8>().unwrap()).collect::<Vec<u8>>();
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }

    #[test]
    fn airdrop() {
        const RPC_URL: &str = "https://api.devnet.solana.com";
        let wallet = read_keypair_file("dev-wallet.json").expect("Couldnt find wallet file");
        let client = RpcClient::new(RPC_URL);
        match client.request_airdrop(&wallet.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
            }
            Err(e) => println!("Oops, something went wrong: {}", e.to_string())
        }
    }
    #[test]
    fn transfers(){
        const RPC_URL: &str = "https://api.devnet.solana.com";
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str("Em8siRGMG68b1c3qzfEVzqJEMXvaBt1Kem9YEj7WmaHF").unwrap();
        let client = RpcClient::new(RPC_URL);
        let balance = client.get_balance(&keypair.pubkey()).unwrap();
        let recent_blockhash = client.get_latest_blockhash()
        .expect("Failed to get recent blockhash");
        let mocktransaction = Transaction::new_signed_with_payer( &[transfer(&keypair.pubkey(), &to_pubkey, balance)], Some(&keypair.pubkey()), &vec![&keypair], recent_blockhash);
        let gas = client.get_fee_for_message(mocktransaction.message()).unwrap();
        let transaction = Transaction::new_signed_with_payer( &[transfer(&keypair.pubkey(), &to_pubkey, balance - gas)], Some(&keypair.pubkey()), &vec![&keypair], recent_blockhash);
        
        let signature = client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",signature);
    }
}
