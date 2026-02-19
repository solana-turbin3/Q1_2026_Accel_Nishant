use {
    anchor_escrow::{
        accounts::{Make as MakeAccounts, Refund, Take},
        instruction::{Make as MakeIx, Refund as RefundIx, Take as TakeIx},
        state::Escrow,
    },
    anchor_lang::{AccountDeserialize, InstructionData, ToAccountMetas},
    anchor_spl::associated_token::get_associated_token_address,
    litesvm::LiteSVM,
    litesvm_token::{
        spl_token::ID as TOKEN_PROGRAM_ID, CreateAssociatedTokenAccount, CreateMint, MintTo,
    },
    solana_address::Address,
    solana_instruction::{account_meta::AccountMeta, Instruction},
    solana_keypair::{read_keypair_file, Keypair},
    solana_pubkey::Pubkey,
    solana_sdk_ids::system_program::ID as SYSTEM_PROGRAM_ID,
    solana_signer::Signer,
    solana_transaction::Transaction,
    std::str::FromStr,
};

// Constants
const ESCROW_SEED: u64 = 123;
const INITIAL_MINT_AMOUNT: u64 = 1_000_000_000; // 1000 tokens (6 decimals)
const AIRDROP_LAMPORTS: u64 = 10_000_000_000;

#[test]
fn escrow() {
    let program_keypair =
        read_keypair_file("../../target/deploy/anchor_escrow-keypair.json").unwrap();
    let program_id = program_keypair.pubkey();
    let mut svm = setup_svm(&program_keypair);

    let (maker, taker, mint_a, mint_b, maker_ata_a, taker_ata_a, taker_ata_b, maker_ata_b) =
        setup_tokens(&mut svm);

    let program_id_key = pubkey_from_address(program_id);
    let escrow = Pubkey::find_program_address(
        &[
            b"escrow",
            maker.pubkey().as_ref(),
            &ESCROW_SEED.to_le_bytes(),
        ],
        &program_id_key,
    )
    .0;
    let vault = get_associated_token_address(&escrow, &pubkey_from_address(mint_a));

    let associated_token_program = spl_associated_token_account::ID;
    let token_program = TOKEN_PROGRAM_ID;
    let system_program = SYSTEM_PROGRAM_ID;

    // Fund maker's ATA
    MintTo::new(&mut svm, &maker, &mint_a, &maker_ata_a, INITIAL_MINT_AMOUNT)
        .send()
        .expect("Mint to maker ATA");

    // --- Make (first escrow) ---
    send_tx(
        &mut svm,
        make_instruction(
            program_id,
            &maker,
            mint_a,
            mint_b,
            maker_ata_a,
            escrow,
            vault,
            associated_token_program,
            token_program,
            system_program,
            10,
            10,
        ),
        &[&maker],
    );

    let escrow_state = load_escrow_state(&svm, escrow);
    assert_eq!(escrow_state.seed, ESCROW_SEED);
    assert_eq!(escrow_state.receive, 10);

    // --- Take ---
    MintTo::new(&mut svm, &taker, &mint_b, &taker_ata_b, INITIAL_MINT_AMOUNT)
        .send()
        .expect("Mint to taker ATA");

    send_tx(
        &mut svm,
        take_instruction(
            program_id,
            &maker,
            &taker,
            mint_a,
            mint_b,
            taker_ata_a,
            taker_ata_b,
            maker_ata_b,
            escrow,
            vault,
            associated_token_program,
            token_program,
            system_program,
        ),
        &[&taker],
    );

    assert!(
        svm.get_account(&address_from_pubkey(escrow)).is_none(),
        "Escrow should be closed after take"
    );

    // --- Make (second escrow for refund test) ---
    send_tx(
        &mut svm,
        make_instruction(
            program_id,
            &maker,
            mint_a,
            mint_b,
            maker_ata_a,
            escrow,
            vault,
            associated_token_program,
            token_program,
            system_program,
            100,
            100,
        ),
        &[&maker],
    );

    // --- Refund ---
    send_tx(
        &mut svm,
        refund_instruction(
            program_id,
            &maker,
            mint_a,
            maker_ata_a,
            escrow,
            vault,
            token_program,
            system_program,
        ),
        &[&maker],
    );

    let escrow_after = svm.get_account(&address_from_pubkey(escrow));
    assert!(
        escrow_after.is_none() || escrow_after.as_ref().unwrap().lamports == 0,
        "Escrow should be closed after refund"
    );

    let maker_balance = get_token_balance(&svm, maker_ata_a);
    assert_eq!(
        maker_balance,
        INITIAL_MINT_AMOUNT - 10,
        "Maker should have initial minus first deposit after refund"
    );
}

// --- Helpers ---

fn setup_svm(program_keypair: &Keypair) -> LiteSVM {
    let mut svm = LiteSVM::new();
    let _ = svm.add_program(
        program_keypair.pubkey(),
        include_bytes!("../../../target/deploy/anchor_escrow.so"),
    );
    svm
}

fn setup_tokens(
    svm: &mut LiteSVM,
) -> (
    Keypair,
    Keypair,
    Address,
    Address,
    Address,
    Address,
    Address,
    Address,
) {
    let maker = Keypair::new();
    let taker = Keypair::new();
    svm.airdrop(&maker.pubkey(), AIRDROP_LAMPORTS).unwrap();
    svm.airdrop(&taker.pubkey(), AIRDROP_LAMPORTS).unwrap();

    let mint_a = CreateMint::new(svm, &maker)
        .authority(&maker.pubkey())
        .decimals(6)
        .send()
        .unwrap();

    let mint_b = CreateMint::new(svm, &maker)
        .authority(&taker.pubkey())
        .decimals(6)
        .send()
        .unwrap();

    let maker_ata_a = CreateAssociatedTokenAccount::new(svm, &maker, &mint_a)
        .owner(&maker.pubkey())
        .send()
        .unwrap();

    let taker_ata_b = CreateAssociatedTokenAccount::new(svm, &taker, &mint_b)
        .owner(&taker.pubkey())
        .send()
        .unwrap();

    let taker_ata_a = CreateAssociatedTokenAccount::new(svm, &taker, &mint_a)
        .owner(&taker.pubkey())
        .send()
        .unwrap();

    let maker_ata_b = CreateAssociatedTokenAccount::new(svm, &maker, &mint_b)
        .owner(&maker.pubkey())
        .send()
        .unwrap();

    (
        maker,
        taker,
        mint_a,
        mint_b,
        maker_ata_a,
        taker_ata_a,
        taker_ata_b,
        maker_ata_b,
    )
}

fn to_account_metas<M>(accounts: M) -> Vec<AccountMeta>
where
    M: ToAccountMetas,
{
    accounts
        .to_account_metas(None)
        .into_iter()
        .map(|m| AccountMeta {
            pubkey: address_from_pubkey(m.pubkey),
            is_signer: m.is_signer,
            is_writable: m.is_writable,
        })
        .collect()
}

fn send_tx(svm: &mut LiteSVM, ix: Instruction, signers: &[&Keypair]) {
    let payer = signers[0].pubkey();
    let tx =
        Transaction::new_signed_with_payer(&[ix], Some(&payer), signers, svm.latest_blockhash());
    svm.send_transaction(tx).expect("Transaction failed");
}

fn make_instruction(
    program_id: Address,
    maker: &Keypair,
    mint_a: Address,
    mint_b: Address,
    maker_ata_a: Address,
    escrow: Pubkey,
    vault: Pubkey,
    associated_token_program: Address,
    token_program: Address,
    system_program: Pubkey,
    deposit: u64,
    receive: u64,
) -> Instruction {
    Instruction {
        program_id,
        accounts: to_account_metas(MakeAccounts {
            maker: pubkey_from_address(maker.pubkey()),
            mint_a: pubkey_from_address(mint_a),
            mint_b: pubkey_from_address(mint_b),
            maker_ata_a: pubkey_from_address(maker_ata_a),
            escrow,
            vault,
            associated_token_program: pubkey_from_address(associated_token_program),
            token_program: pubkey_from_address(token_program),
            system_program,
        }),
        data: MakeIx {
            deposit,
            seed: ESCROW_SEED,
            receive,
        }
        .data(),
    }
}

fn take_instruction(
    program_id: Address,
    maker: &Keypair,
    taker: &Keypair,
    mint_a: Address,
    mint_b: Address,
    taker_ata_a: Address,
    taker_ata_b: Address,
    maker_ata_b: Address,
    escrow: Pubkey,
    vault: Pubkey,
    associated_token_program: Address,
    token_program: Address,
    system_program: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: to_account_metas(Take {
            taker: pubkey_from_address(taker.pubkey()),
            maker: pubkey_from_address(maker.pubkey()),
            mint_a: pubkey_from_address(mint_a),
            mint_b: pubkey_from_address(mint_b),
            taker_ata_a: pubkey_from_address(taker_ata_a),
            taker_ata_b: pubkey_from_address(taker_ata_b),
            maker_ata_b: pubkey_from_address(maker_ata_b),
            escrow,
            vault,
            associated_token_program: pubkey_from_address(associated_token_program),
            token_program: pubkey_from_address(token_program),
            system_program,
        }),
        data: TakeIx {}.data(),
    }
}

fn refund_instruction(
    program_id: Address,
    maker: &Keypair,
    mint_a: Address,
    maker_ata_a: Address,
    escrow: Pubkey,
    vault: Pubkey,
    token_program: Address,
    system_program: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: to_account_metas(Refund {
            maker: pubkey_from_address(maker.pubkey()),
            mint_a: pubkey_from_address(mint_a),
            maker_ata_a: pubkey_from_address(maker_ata_a),
            escrow,
            vault,
            token_program: pubkey_from_address(token_program),
            system_program,
        }),
        data: RefundIx {}.data(),
    }
}

fn load_escrow_state(svm: &LiteSVM, escrow: Pubkey) -> Escrow {
    let account = svm.get_account(&address_from_pubkey(escrow)).unwrap();
    Escrow::try_deserialize(&mut account.data.as_slice()).unwrap()
}

fn get_token_balance(svm: &LiteSVM, ata: Address) -> u64 {
    let account = svm.get_account(&ata).unwrap();
    anchor_spl::token::TokenAccount::try_deserialize(&mut account.data.as_slice())
        .unwrap()
        .amount
}

fn pubkey_from_address(addr: Address) -> Pubkey {
    Pubkey::new_from_array(addr.to_bytes())
}

fn address_from_pubkey(pk: Pubkey) -> Address {
    Address::from_str(&pk.to_string()).unwrap()
}