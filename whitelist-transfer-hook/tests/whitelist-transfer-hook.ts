import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  TOKEN_2022_PROGRAM_ID,
  getAssociatedTokenAddressSync,
  createInitializeMintInstruction,
  getMintLen,
  ExtensionType,
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
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const wallet = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.whitelistTransferHook as Program<WhitelistTransferHook>;

  // Keypairs and addresses
  const mint2022 = Keypair.generate();
  const user = Keypair.generate();
  const recipient = Keypair.generate();

  // PDA helpers
  const getPda = (prefix: string, pubkey: PublicKey) =>
    PublicKey.findProgramAddressSync([Buffer.from(prefix), pubkey.toBuffer()], program.programId)[0];

  const extraAccountMetaListPDA = getPda("extra-account-metas", mint2022.publicKey);
  const whitelistedUserPDA = getPda("whitelisted_user", user.publicKey);
  const configPda = PublicKey.findProgramAddressSync(
    [Buffer.from("config"), wallet.payer.publicKey.toBuffer()],
    program.programId
  )[0];

  // Token accounts
  const sourceTokenAccount = getAssociatedTokenAddressSync(
    mint2022.publicKey, user.publicKey, false, TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID
  );
  const destinationTokenAccount = getAssociatedTokenAddressSync(
    mint2022.publicKey, recipient.publicKey, false, TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID
  );

  before(async () => {
    // Fund the user
    const airdropSig = await provider.connection.requestAirdrop(
      user.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdropSig, "confirmed");
  });

  it("Initializes the Admin Config", async () => {
    const tx = await program.methods.initConfig().accountsPartial({
      admin: provider.publicKey,
      config: configPda,
      systemProgram: SystemProgram.programId,
    }).rpc();

    console.log("\nAdmin config initialized:", configPda.toBase58(), "\nTransaction signature:", tx);

    const configAccount = await program.account.config.fetch(configPda);
    assert.ok(configAccount.admin.equals(provider.publicKey), "Admin does not match provider");
    assert.ok(typeof configAccount.bump === "number", "Config bump missing");
  });

  it("Add user to whitelist", async () => {
    const tx = await program.methods.addToWhitelist(user.publicKey).accountsPartial({
      admin: wallet.publicKey,
      config: configPda,
      whitelistedUser: whitelistedUserPDA,
      systemProgram: SystemProgram.programId,
    }).rpc();

    const whitelistedUserAccount = await program.account.whitelistedUser.fetch(whitelistedUserPDA);
    console.log("\nWhitelisted user account onchain:", whitelistedUserAccount.user.toBase58());
    console.log("\nUser added to whitelist:", user.publicKey.toBase58(), "\nTransaction signature:", tx);

    assert.ok(whitelistedUserAccount.user.equals(user.publicKey), "Whitelist public key mismatch");
    assert.ok(typeof whitelistedUserAccount.bump === "number", "Whitelist bump missing");
  });

  it("Remove user from whitelist", async () => {
    const tx = await program.methods.removeFromWhitelist(user.publicKey).accountsPartial({
      admin: wallet.publicKey,
      config: configPda,
      whitelistedUser: whitelistedUserPDA,
      systemProgram: SystemProgram.programId,
    }).rpc();

    console.log("\nUser removed from whitelist:", user.publicKey.toBase58(), "\nTransaction signature:", tx);

    let removedUserAccount = null;
    try {
      removedUserAccount = await program.account.whitelistedUser.fetch(whitelistedUserPDA);
    } catch (_) {
      removedUserAccount = null;
    }
    assert.ok(removedUserAccount === null, "Whitelisted user account still exists after removal");
  });

  it("Re-Add user to whitelist", async () => {
    const tx = await program.methods.addToWhitelist(user.publicKey).accountsPartial({
      admin: wallet.publicKey,
      config: configPda,
      whitelistedUser: whitelistedUserPDA,
      systemProgram: SystemProgram.programId,
    }).rpc();

    console.log("\nUser re-added to whitelist:", user.publicKey.toBase58(), "\nTransaction signature:", tx);

    const whitelistedUserAccount = await program.account.whitelistedUser.fetch(whitelistedUserPDA);
    assert.ok(whitelistedUserAccount.user.equals(user.publicKey), "Re-added whitelist user mismatch");
    assert.ok(typeof whitelistedUserAccount.bump === "number", "Whitelist bump missing after re-adding");
    console.log("\nUser re-added and verified on whitelist:", user.publicKey.toBase58());
  });

  it("Create Mint Account with Transfer Hook Extension", async () => {
    const extensions = [ExtensionType.TransferHook];
    const mintLen = getMintLen(extensions);
    const lamports = await provider.connection.getMinimumBalanceForRentExemption(mintLen);

    const transaction = new Transaction().add(
      SystemProgram.createAccount({
        fromPubkey: wallet.publicKey,
        newAccountPubkey: mint2022.publicKey,
        space: mintLen,
        lamports,
        programId: TOKEN_2022_PROGRAM_ID,
      }),
      createInitializeTransferHookInstruction(
        mint2022.publicKey,
        wallet.publicKey,
        program.programId,
        TOKEN_2022_PROGRAM_ID,
      ),
      createInitializeMintInstruction(
        mint2022.publicKey,
        9,
        wallet.publicKey,
        null,
        TOKEN_2022_PROGRAM_ID
      )
    );

    const txSig = await sendAndConfirmTransaction(
      provider.connection,
      transaction,
      [wallet.payer, mint2022],
      { skipPreflight: true, commitment: 'finalized' }
    );

    // const txDetails = await provider.connection.getTransaction(txSig, { maxSupportedTransactionVersion: 0, commitment: 'confirmed' });
    console.log("\nTransaction Signature:", txSig);

    const mintAccountInfo = await provider.connection.getAccountInfo(mint2022.publicKey);
    assert.ok(mintAccountInfo !== null, "Mint account missing");
    assert.ok(mintAccountInfo.owner.equals(TOKEN_2022_PROGRAM_ID), "Mint owner mismatch");
  });

  it("Create Token Accounts and Mint Tokens", async () => {
    const amount = 100 * 10 ** 9;

    const transaction = new Transaction().add(
      createAssociatedTokenAccountInstruction(
        user.publicKey, sourceTokenAccount, user.publicKey,
        mint2022.publicKey, TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID
      ),
      createAssociatedTokenAccountInstruction(
        wallet.publicKey, destinationTokenAccount, recipient.publicKey,
        mint2022.publicKey, TOKEN_2022_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID
      ),
      createMintToInstruction(
        mint2022.publicKey, sourceTokenAccount, wallet.publicKey,
        amount, [], TOKEN_2022_PROGRAM_ID
      )
    );

    const txSig = await sendAndConfirmTransaction(
      provider.connection, transaction, [user, wallet.payer], { skipPreflight: true }
    );
    console.log("\nTransaction Signature:", txSig);
  });

  it("Create ExtraAccountMetaList Account", async () => {
    const tx = await program.methods
      .initializeTransferHook()
      .accountsPartial({
        payer: wallet.publicKey,
        mint: mint2022.publicKey,
        extraAccountMetaList: extraAccountMetaListPDA,
        systemProgram: SystemProgram.programId,
      }).rpc();

    console.log("\nExtraAccountMetaList Account created:", extraAccountMetaListPDA.toBase58());
    console.log("Transaction Signature:", tx);
  });

  it("Transfer Token with Extra Account Meta", async () => {
    const amount = 1 * 10 ** 9;

    const transferInstruction = createTransferCheckedInstruction(
      sourceTokenAccount,
      mint2022.publicKey,
      destinationTokenAccount,
      user.publicKey,
      amount,
      9,
      [],
      TOKEN_2022_PROGRAM_ID
    );

    transferInstruction.keys.push(
      { pubkey: extraAccountMetaListPDA, isSigner: false, isWritable: false },
      { pubkey: whitelistedUserPDA, isSigner: false, isWritable: false },
      { pubkey: program.programId, isSigner: false, isWritable: false }
    );

    const transaction = new Transaction().add(transferInstruction);
    try {
      const txSig = await sendAndConfirmTransaction(
        provider.connection, transaction, [user], { skipPreflight: false }
      );
      console.log("\nTransfer Signature:", txSig);
    } catch (error: any) {
      console.error(error);
      if (error?.meta?.logMessages) {
        error.meta.logMessages.forEach((log: string, i: number) => console.error(`  ${i}: ${log}`));
      }
      if (error instanceof SendTransactionError && error.logs) {
        console.error("\nTransaction failed:", error.logs[6]);
        error.logs.forEach((log, i) => console.error(`  ${i}: ${log}`));
      } else {
        console.error("\nUnexpected error:", error);
      }
    }
  });
})