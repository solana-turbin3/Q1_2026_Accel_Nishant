import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { GetCommitmentSignature } from "@magicblock-labs/ephemeral-rollups-sdk";
import { ErStateAccount } from "../target/types/er_state_account";

describe("er-state-account", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const providerEphemeralRollup = new anchor.AnchorProvider(
    new anchor.web3.Connection(
      process.env.EPHEMERAL_PROVIDER_ENDPOINT ||
      "https://devnet.magicblock.app/",
      {
        wsEndpoint:
          process.env.EPHEMERAL_WS_ENDPOINT || "wss://devnet.magicblock.app/",
      },
    ),
    anchor.Wallet.local(),
  );
  console.log("Base Layer Connection: ", provider.connection.rpcEndpoint);
  console.log(
    "Ephemeral Rollup Connection: ",
    providerEphemeralRollup.connection.rpcEndpoint,
  );
  console.log(`Current SOL Public Key: ${anchor.Wallet.local().publicKey}`);

  before(async function () {
    const balance = await provider.connection.getBalance(
      anchor.Wallet.local().publicKey,
    );
    console.log("Current balance is", balance / LAMPORTS_PER_SOL, " SOL", "\n");
  });

  const program = anchor.workspace.erStateAccount as Program<ErStateAccount>;

  const userAccount = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("user"), anchor.Wallet.local().publicKey.toBuffer()],
    program.programId,
  )[0];

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accountsPartial({
        user: anchor.Wallet.local().publicKey,
        userAccount: userAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("User Account initialized: ", tx);
  });

  it("Update State!", async () => {
    const tx = await program.methods
      .update(new anchor.BN(42))
      .accountsPartial({
        user: anchor.Wallet.local().publicKey,
        userAccount: userAccount,
      })
      .rpc();
    console.log("\nUser Account State Updated: ", tx);
  });

  it("Delegate to Ephemeral Rollup!", async () => {
    let tx = await program.methods
      .delegate()
      .accountsPartial({
        user: anchor.Wallet.local().publicKey,
        userAccount: userAccount,
        validator: new PublicKey("MAS1Dt9qreoRMQ14YQuhg8UTZMMzDdKhmkZMECCzk57"),
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc({ skipPreflight: true });

    console.log("\nUser Account Delegated to Ephemeral Rollup: ", tx);
  });

  it("Update State and Commit to Base Layer!", async () => {
    let tx = await program.methods
      .updateCommit(new anchor.BN(43))
      .accountsPartial({
        user: providerEphemeralRollup.wallet.publicKey,
        userAccount: userAccount,
      })
      .transaction();

    tx.feePayer = providerEphemeralRollup.wallet.publicKey;

    tx.recentBlockhash = (
      await providerEphemeralRollup.connection.getLatestBlockhash()
    ).blockhash;
    tx = await providerEphemeralRollup.wallet.signTransaction(tx);
    const txHash = await providerEphemeralRollup.sendAndConfirm(tx, [], {
      skipPreflight: false,
    });
    const txCommitSgn = await GetCommitmentSignature(
      txHash,
      providerEphemeralRollup.connection,
    );

    console.log("\nUser Account State Updated: ", txHash);
  });

  it("Commit and undelegate from Ephemeral Rollup!", async () => {
    let info = await providerEphemeralRollup.connection.getAccountInfo(
      userAccount,
    );

    console.log("User Account Info: ", info);

    console.log("User account", userAccount.toBase58());

    let tx = await program.methods
      .undelegate()
      .accounts({
        user: providerEphemeralRollup.wallet.publicKey,
      })
      .transaction();

    tx.feePayer = providerEphemeralRollup.wallet.publicKey;

    tx.recentBlockhash = (
      await providerEphemeralRollup.connection.getLatestBlockhash()
    ).blockhash;
    tx = await providerEphemeralRollup.wallet.signTransaction(tx);
    const txHash = await providerEphemeralRollup.sendAndConfirm(tx, [], {
      skipPreflight: false,
    });
    const txCommitSgn = await GetCommitmentSignature(
      txHash,
      providerEphemeralRollup.connection,
    );

    console.log("\nUser Account Undelegated: ", txHash);
  });

  it("Update State!", async () => {
    let tx = await program.methods
      .update(new anchor.BN(45))
      .accountsPartial({
        user: anchor.Wallet.local().publicKey,
        userAccount: userAccount,
      })
      .rpc();

    console.log("\nUser Account State Updated: ", tx);
  });

  it("Close Account!", async () => {
    const tx = await program.methods
      .close()
      .accountsPartial({
        user: anchor.Wallet.local().publicKey,
        userAccount: userAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("\nUser Account Closed: ", tx);
  });
});

/**
 ****** Logs ******
  er-state-account
Current balance is 3.793924748  SOL 

User Account initialized:  2MSW6hE7E52K7P3KU8P3J1QthhTVCVrSFXNZ17sMHx5j1thP8kRmoDQSPWvfKCzYfZXtVdZ7nGpLHkBEceStayBn
    ✔ Is initialized! (3413ms)

User Account State Updated:  3XaoQcwz72vp4abNuJJ6nw4UoCCmb6xFuQbbSpvwvtAgbgpvBGtLobDD5Hj5Wdyv24TxTQBsrZZB1uiyb3mjhbdr
    ✔ Update State! (3187ms)

User Account Delegated to Ephemeral Rollup:  22MJ8o92k4ZSPFji8sKwpTM9n6f76vv9gQPjrtbyXfheDeqjLgjnhjTxMQxYu5oFpPNFKKC5DNXEXy5C3Shfkrpn
    ✔ Delegate to Ephemeral Rollup! (876ms)

User Account State Updated:  38qUMwHzSBeGoRKU2Ld1JZDtPD24kWA9xVYDuwvuTgJrPYPLNS5MKXnUtbZBh8GrdSmpcMVSNTTAxs3Uuc6yyDnC
    ✔ Update State and Commit to Base Layer! (3094ms)
User Account Info:  {
  lamports: 1231920,
  data: <Buffer d3 21 88 10 ba 6e f2 7f 3a c1 42 4c c2 29 92 7c 5b 93 60 c8 b0 89 e3 82 d3 c7 8b 48 e3 53 5b 7f 98 f1 88 a1 10 9b e6 cc 2b 00 00 00 00 00 00 00 ff>,
  owner: PublicKey [PublicKey(EQkMxVqHWsEPHD44yAicQZ55Av8AGbfLdgsuZPJmUBqm)] {
    _bn: <BN: c73d5144e8dd99e00d39ee3d94d8e492cc0428a4c7cf8af85c3c0b4ef49eec8c>
  },
  executable: false,
  rentEpoch: 18446744073709552000,
  space: 49
}
User account 83Trwc6fs2kzBUd2PDnzbwnfTJgjmdbV5saSGXqawBjU

User Account Undelegated:  2o56FhRPKiNJmQEupV4drzLw7Z47YLMN85on9kGf5KrXnBwc3uk5hpcUGdWUKYgKTY69zLgsTaJ59fQCZ6yXZ5V8
    ✔ Commit and undelegate from Ephemeral Rollup! (2080ms)

User Account State Updated:  66ADjarjSLt2AxMiSbkR8evD7qrhg8Vuuy726csebMstvwraavykxXJQL2qVJT7TJo27h7rUi83hfsojW7Sp5pho
    ✔ Update State! (4746ms)

User Account Closed:  5BmAdTYcf2gkmNqyfs72ND6FvfdSu2uMofeiDj9uYehK5gad33DEcr3Ci9pCyATrUSLzUoEVHWRccEiXVVVfJ92X
    ✔ Close Account! (7506ms)


  7 passing (26s)

✨  Done in 27.25s.


 */