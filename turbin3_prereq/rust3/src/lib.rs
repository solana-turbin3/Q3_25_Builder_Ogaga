#[cfg(test)]
mod tests {
    use solana_sdk;
    
    #[test]
    fn keygen() {

    use solana_sdk::{signature::{Keypair, Signer}, pubkey::Pubkey};
    
    // Create a new keypair
    let kp = Keypair::new();
    
    println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
    println!("");
    println!("To save your wallet, copy and paste the following into a JSON file:");
    println!("{:?}", kp.to_bytes());
   }
    
    #[test]
    fn airdrop() {
        use solana_client::rpc_client::RpcClient;
        use solana_sdk::{
            signature::{Keypair, Signer, read_keypair_file},
        };
        
        const RPC_URL: &str = "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";
        
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        
        // Connect to Solana devnet
        let client = RpcClient::new(RPC_URL);
        
        // Request 2 SOL (2 billion lamports)
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(sig) => {
                println!("Success! Check your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
            }
            Err(err) => {
                println!("Airdrop failed: {}", err);
            }
        }
    }
    
    #[test]
    fn transfer_sol() {
        use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey, system_instruction::transfer};
    use solana_sdk::{
        signature::{Keypair, Signer, read_keypair_file},
        transaction::Transaction,
    };
    use std::str::FromStr;
    
    const RPC_URL: &str = "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";
    
    // Load  devnet keypair
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    
    // Define destination (replace with  Turbin3 public key)
    let to_pubkey = Pubkey::from_str("3itV3LViDVHD22KrQ6CTN1wm3B7Q2AmCy1ULYzDpfdgs").unwrap();
    
    // Connect to devnet
    let rpc_client = RpcClient::new(RPC_URL);
    
    // Get recent blockhash
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");
    
    // Create and sign transaction (0.1 SOL = 100,000,000 lamports)
    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&keypair.pubkey(), &to_pubkey, 100_000_000)],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash,
    );
    
    // Send transaction
    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");
    
    println!(
        "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
        
    }


    #[test]
fn empty_wallet() {
    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey, system_instruction::transfer};
    use solana_sdk::{
        message::Message,
        signature::{Keypair, Signer, read_keypair_file},
        transaction::Transaction,
    };
    use std::str::FromStr;
    
    const RPC_URL: &str = "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";
    
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    let to_pubkey = Pubkey::from_str("").unwrap();
    let rpc_client = RpcClient::new(RPC_URL);
    
    // Get current balance
    let balance = rpc_client
        .get_balance(&keypair.pubkey())
        .expect("Failed to get balance");
    
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");
    
    // Calculate fee
    let message = Message::new_with_blockhash(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
        Some(&keypair.pubkey()),
        &recent_blockhash,
    );
    
    let fee = rpc_client
        .get_fee_for_message(&message)
        .expect("Failed to get fee calculator");
    
    // Transfer balance minus fee
    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash,
    );
    
    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send final transaction");
    
    println!(
        "Success! Entire balance transferred: https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
}

    
#[test]
fn submit_rust_prerequisite_transaction() {
    use solana_client::rpc_client::RpcClient;
    use solana_sdk::{
        signature::{Keypair, Signer, read_keypair_file},
        transaction::Transaction,
        pubkey::Pubkey,
        system_program,
        instruction::{Instruction, AccountMeta}
    };
    use std::str::FromStr;

    //  Define the connection and load wallet
    const RPC_URL: &str = "https://api.devnet.solana.com";
    let signer = read_keypair_file("Turbin3-wallet.json")
        .expect("Couldn't find wallet file. Make sure your wallet from the TS prerequisite is named Turbin3-wallet.json and is in the root folder.");

    let rpc_client = RpcClient::new(RPC_URL);

    // Set up the public keys for the programs and accounts.
    let turbin3_prereq_program = Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap(); 
    let collection = Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();
    let mpl_core_program = Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();
    let signer_pubkey = signer.pubkey();

    // Derive the necessary PDA.
    let prereq_seeds = &[b"prereqs", signer_pubkey.as_ref()]; 
    let (prereq_pda, _prereq_bump) = Pubkey::find_program_address(prereq_seeds, &turbin3_prereq_program);

    // This PDA is the authority for the NFT collection. 
    let authority_seeds = &[b"collection", collection.as_ref()];
    let (authority_pda, _authority_bump) = Pubkey::find_program_address(authority_seeds, &turbin3_prereq_program);

    // 'submit_rs' instruction.
    let mint = Keypair::new();

    // This discriminator uniquely identifies the 'submit_rs' instruction.
    let data = vec![77, 124, 82, 163, 21, 133, 181, 206];

    // Define the accounts required by the instruction, as seen in the IDL.
    let accounts = vec![
        AccountMeta::new(signer.pubkey(), true),      
        AccountMeta::new(prereq_pda, false),           
        AccountMeta::new(mint.pubkey(), true),        
        AccountMeta::new(collection, false),          
        AccountMeta::new_readonly(authority_pda, false), 
        AccountMeta::new_readonly(mpl_core_program, false), 
        AccountMeta::new_readonly(system_program::id(), false), 
    ];

    let instruction = Instruction {
        program_id: turbin3_prereq_program,
        accounts,
        data,
    };

    // CREATE AND SEND TRANSACTION
    let blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash"); 

    // Create the transaction.
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&signer.pubkey()),
        &[&signer, &mint],
        blockhash,
    );

    // Send the transaction to the Solana devnet.
    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");

    println!(
        "Success! Check out your TX here:\nhttps://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
}
}