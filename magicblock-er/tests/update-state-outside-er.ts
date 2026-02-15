
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { GetCommitmentSignature } from "@magicblock-labs/ephemeral-rollups-sdk";
import { ErStateAccount } from "../target/types/er_state_account";

describe("Update State Outside ER ( Ephemeral Rollup )", () => {
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
  console.log("\n\nBase Layer Connection: ", provider.connection.rpcEndpoint);
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

  xit("Is initialized!", async () => {
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


  it("randomize user account", async () => {
    const tx = await program.methods
      .randomizeUserAccount(50)
      .accountsPartial({
        payer: anchor.Wallet.local().publicKey,
        userAccount: userAccount,
      })
      .rpc();



    // 3. Wait ~1 second for callback, then fetch
    await new Promise(resolve => setTimeout(resolve, 1000));
    const account = await program.account.userAccount.fetch(userAccount);
    console.log("Random data:", account.data.toString()); // Random u64
  });

});



/**
--------Test 1 logs for randomize user account ------------


  Update State Outside ER ( Ephemeral Rollup )
Current balance is 13.637297748  SOL

User Account initialized:  54icq1kxKtqr2JqUAxQrdQcugV33udzSq2RmzPSgoHQBQn6xCPVzMPN4q5uGQFvUF2H4yyrjfi8RurxnFnkPRdfR
    ✔ Is initialized! (750ms)
Random data: 3583545053728053454

 
 */