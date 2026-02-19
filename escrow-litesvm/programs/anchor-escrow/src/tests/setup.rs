use anchor_lang::prelude::Pubkey;
use litesvm::LiteSVM;
use solana_address::Address;
use solana_keypair::Keypair;
use solana_native_token::LAMPORTS_PER_SOL;
use solana_signer::Signer;
use std::path::PathBuf;

pub static PROGRAM_ID: Pubkey = crate::ID;


pub fn setup() -> (LiteSVM, Keypair) {
    let mut svm = LiteSVM::new();
    let _ =svm.add_program(
        PROGRAM_ID,
        include_bytes!("../../../target/deploy/anchor_escrow.so"),
    );

    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 10 * LAMPORTS_PER_SOL)
        .expect("Failed to airdrop SOL to payer");
    (svm, payer)
}



pub fn pubkey_from_address(addr: Address) -> Pubkey {
    Pubkey::new_from_array(addr.to_bytes())
}

pub fn address_from_pubkey(pk: Pubkey) -> Address {
    Address::from_str(&pk.to_string()).unwrap()
}