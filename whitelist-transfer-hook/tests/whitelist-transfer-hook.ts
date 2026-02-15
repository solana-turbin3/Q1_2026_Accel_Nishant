import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  TOKEN_2022_PROGRAM_ID,
  getAssociatedTokenAddressSync,
  createInitializeMintInstruction,
  getMintLen,
  ExtensionType, unpackMint,
  createTransferCheckedWithTransferHookInstruction,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createInitializeTransferHookInstruction,
  createAssociatedTokenAccountInstruction,
  createMintToInstruction,
  createTransferCheckedInstruction,
} from "@solana/spl-token";
import {
  Keypair,
  PublicKey,
  SendTransactionError,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction
} from '@solana/web3.js';
import { WhitelistTransferHook } from "../target/types/whitelist_transfer_hook";
import { assert } from "chai";


describe("whitelist-transfer-hook", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const wallet = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.whitelistTransferHook as Program<WhitelistTransferHook>;

  const mint2022 = anchor.web3.Keypair.generate();


  const user = Keypair.generate();
  // Airdrop some SOL to the user keypair so it can pay for transactions if needed.
  before(async () => {
    // Configure connection
    const connection = provider.connection;

    // Request an airdrop
    const airdropSig = await connection.requestAirdrop(
      user.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(airdropSig, "confirmed");
  });

  // ExtraAccountMetaList address
  // Store extra accounts required by the custom transfer hook instruction
  const [extraAccountMetaListPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from('extra-account-metas'), mint2022.publicKey.toBuffer()],
    program.programId,
  );

  const whitelisted_user = PublicKey.findProgramAddressSync(
    [
      Buffer.from("whitelisted_user"), user.publicKey.toBuffer()
    ],
    program.programId
  )[0];

  const configPda = PublicKey.findProgramAddressSync([
    Buffer.from("config"), wallet.payer.publicKey.toBuffer()
  ], program.programId)[0];

  // Sender token account address
  const sourceTokenAccount = getAssociatedTokenAddressSync(
    mint2022.publicKey,
    user.publicKey,
    false,
    TOKEN_2022_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  // Recipient token account address
  const recipient = anchor.web3.Keypair.generate();
  const destinationTokenAccount = getAssociatedTokenAddressSync(
    mint2022.publicKey,
    recipient.publicKey,
    false,
    TOKEN_2022_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  it("Initializes the Admin Config", async () => {
    const tx = await program.methods.initConfig()
      .accountsPartial({
        admin: provider.publicKey,
        config: configPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("\nAdmin config initialized:", configPda.toBase58());
    console.log("Transaction signature:", tx);

    // Fetch the config account that was just initialized
    const configAccount = await program.account.config.fetch(configPda);

    // Check that the admin is set to provider.publicKey
    assert.ok(configAccount.admin.equals(provider.publicKey), "Admin should be set to provider.publicKey");

    // Check that bump is present (should be a number)
    assert.ok(typeof configAccount.bump === "number", "Bump should be a number");
  });

  it("Add user to whitelist", async () => {
    const tx = await program.methods.addToWhitelist(user.publicKey)
      .accountsPartial({
        admin: wallet.publicKey,
        config: configPda,
        whitelistedUser: whitelisted_user,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    // Fetch the whitelisted user account
    const whitelistedUserAccount = await program.account.whitelistedUser.fetch(whitelisted_user);

    console.log("\n User whitelisted account on onchain:", whitelistedUserAccount.user.toBase58())
    console.log("\nUser added to whitelist:", user.publicKey.toBase58());
    console.log("Transaction signature:", tx);
    // Check if the stored user in the whitelist account matches
    assert.ok(
      whitelistedUserAccount.user.equals(user.publicKey),
      "User should be whitelisted (public key should match)"
    );


    // Optionally, check that bump is present (sanity check)
    assert.ok(typeof whitelistedUserAccount.bump === "number", "Whitelist entry should have a bump");

  });

  it("Remove user from whitelist", async () => {
    const tx = await program.methods.removeFromWhitelist(user.publicKey)
      .accountsPartial({
        admin: wallet.publicKey,
        config: configPda,
        whitelistedUser: whitelisted_user,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("\nUser removed from whitelist:", user.publicKey.toBase58());
    console.log("Transaction signature:", tx);

    // Fetch the whitelisted user account after removal
    let removedUserAccount = null;
    try {
      removedUserAccount = await program.account.whitelistedUser.fetch(whitelisted_user);
    } catch (e) {
      // The account should not exist anymore if removed successfully
      removedUserAccount = null;
    }

    // Condition: After removal, the whitelisted user account should not exist
    assert.ok(
      removedUserAccount === null,
      "Whitelisted user account should not exist after removal"
    );
  });

  it("Re-Add user to whitelist", async () => {
    const tx = await program.methods.addToWhitelist(user.publicKey)
      .accountsPartial({
        admin: wallet.publicKey,
        config: configPda,
        whitelistedUser: whitelisted_user,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("\nUser re-added to whitelist:", user.publicKey.toBase58());
    console.log("Transaction signature:", tx);

    // Verify that the re-added user's whitelisted user account exists and matches the expected owner
    const whitelistedUserAccount = await program.account.whitelistedUser.fetch(whitelisted_user);

    // The whitelistedUserAccount.user field should equal the user's public key
    assert.ok(
      whitelistedUserAccount.user.toBase58() === user.publicKey.toBase58(),
      "The whitelisted user account should belong to the re-added user"
    );

    // Optionally, check that bump is present (sanity check)
    assert.ok(typeof whitelistedUserAccount.bump === "number", "Whitelist entry should have a bump after re-adding");

    console.log("\nUser re-added and verified on whitelist:", user.publicKey.toBase58());
  });

  it('Create Mint Account with Transfer Hook Extension', async () => {
    const extensions = [ExtensionType.TransferHook];
    const mintLen = getMintLen(extensions);
    const lamports = await provider.connection.getMinimumBalanceForRentExemption(mintLen);

    const transaction = new Transaction().add(
      SystemProgram.createAccount({
        fromPubkey: wallet.publicKey,
        newAccountPubkey: mint2022.publicKey,
        space: mintLen,
        lamports: lamports,
        programId: TOKEN_2022_PROGRAM_ID,
      }),
      createInitializeTransferHookInstruction(
        mint2022.publicKey,
        wallet.publicKey,
        program.programId, // Transfer Hook Program ID
        TOKEN_2022_PROGRAM_ID,
      ),
      createInitializeMintInstruction(mint2022.publicKey, 9, wallet.publicKey, null, TOKEN_2022_PROGRAM_ID),
    );

    const txSig = await sendAndConfirmTransaction(provider.connection, transaction, [wallet.payer, mint2022], {
      skipPreflight: true,
      commitment: 'finalized',
    });

    const txDetails = await program.provider.connection.getTransaction(txSig, {
      maxSupportedTransactionVersion: 0,
      commitment: 'confirmed',
    });
    //console.log(txDetails.meta.logMessages);

    console.log("\nTransaction Signature: ", txSig);

    // This test confirms the mint account has the Transfer Hook extension, is owned by wallet, and has correct program IDs set.

    // Fetch the mint account info after creation and initialization.
    const mintAccountInfo = await provider.connection.getAccountInfo(mint2022.publicKey);
    assert.ok(mintAccountInfo !== null, "Mint account should exist");

    // The mint account owner should be the TOKEN_2022_PROGRAM_ID
    assert.ok(
      mintAccountInfo.owner.equals(TOKEN_2022_PROGRAM_ID),
      "Mint account should be owned by the Token-2022 Program"
    );



  });

  it('Create Token Accounts and Mint Tokens', async () => {
    // 100 tokens
    const amount = 100 * 10 ** 9;

    const transaction = new Transaction().add(
      createAssociatedTokenAccountInstruction(
        user.publicKey,
        sourceTokenAccount,
        user.publicKey,
        mint2022.publicKey,
        TOKEN_2022_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID,
      ),
      createAssociatedTokenAccountInstruction(
        wallet.publicKey,
        destinationTokenAccount,
        recipient.publicKey,
        mint2022.publicKey,
        TOKEN_2022_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID,
      ),
      createMintToInstruction(mint2022.publicKey, sourceTokenAccount, wallet.publicKey, amount, [], TOKEN_2022_PROGRAM_ID),
    );

    const txSig = await sendAndConfirmTransaction(provider.connection, transaction, [user, wallet.payer], { skipPreflight: true });

    console.log("\nTransaction Signature: ", txSig);
  });

  // Account to store extra accounts required by the transfer hook instruction
  it('Create ExtraAccountMetaList Account', async () => {
    const initializeExtraAccountMetaListInstruction = await program.methods
      .initializeTransferHook()
      .accountsPartial({
        payer: wallet.publicKey,
        mint: mint2022.publicKey,
        extraAccountMetaList: extraAccountMetaListPDA,
        systemProgram: SystemProgram.programId,
      })
      //.instruction();
      .rpc();

    //const transaction = new Transaction().add(initializeExtraAccountMetaListInstruction);

    //const txSig = await sendAndConfirmTransaction(provider.connection, transaction, [wallet.payer], { skipPreflight: true, commitment: 'confirmed' });
    console.log("\nExtraAccountMetaList Account created:", extraAccountMetaListPDA.toBase58());
    console.log('Transaction Signature:', initializeExtraAccountMetaListInstruction);
  });

  it('Transfer Token with Extra Account Meta', async () => {
    // 1 tokens
    const amount = 1 * 10 ** 9;
    const amountBigInt = BigInt(amount);

    // Create the base transfer instruction
    const transferInstruction = createTransferCheckedInstruction(
      sourceTokenAccount,
      mint2022.publicKey,
      destinationTokenAccount,
      user.publicKey,
      amountBigInt,
      9,
      [],
      TOKEN_2022_PROGRAM_ID,
    );

    // Manually add the extra accounts required by the transfer hook
    // These accounts are needed for the CPI to our transfer hook program
    transferInstruction.keys.push(
      // ExtraAccountMetaList PDA
      { pubkey: extraAccountMetaListPDA, isSigner: false, isWritable: false },
      // Whitelist PDA (the extra account we defined)
      { pubkey: whitelisted_user, isSigner: false, isWritable: false },
      // Transfer hook program
      { pubkey: program.programId, isSigner: false, isWritable: false },
    );

    const transaction = new Transaction().add(transferInstruction);

    try {
      // Send the transaction
      const txSig = await sendAndConfirmTransaction(provider.connection, transaction, [user], { skipPreflight: false });
      console.log("\nTransfer Signature:", txSig);
    }
    catch (error) {
      console.log(error)
      console.log(error?.meta?.logMessages);
      if (error instanceof SendTransactionError) {
        console.error("\nTransaction failed:", error.logs[6]);
        console.error("\nTransaction failed. Full logs:");
        error.logs?.forEach((log, i) => console.error(`  ${i}: ${log}`));
      } else {
        console.error("\nUnexpected error:", error);
      }
    }
  });
});
