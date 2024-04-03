import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import * as token from "@solana/spl-token";
import { SolanaBittensor } from "../target/types/solana_bittensor";

describe("solana-bittensor", () => {
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;

  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaBittensor as Program<SolanaBittensor>;

  let user: anchor.web3.Keypair;
  let bittensorPDA: anchor.web3.PublicKey;
  let subnet1PDA: anchor.web3.PublicKey;
  let subnet1WeightsPDA: anchor.web3.PublicKey;
  let validator1PDA: anchor.web3.PublicKey;
  let miner1PDA: anchor.web3.PublicKey;
  let taoMint: anchor.web3.PublicKey;
  let taoStake: anchor.web3.PublicKey;
  let subnetTaoStake: anchor.web3.PublicKey;

  it("Is initialized bittensor!", async () => {
    user = anchor.web3.Keypair.generate();
    // airdrop some SOL to the user
    const sig = await connection.requestAirdrop(
      user.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );
    const latestBlockHash = await connection.getLatestBlockhash();

    await connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: sig,
    });

    [bittensorPDA] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("bittensor")],
      program.programId
    );

    [taoMint] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(Buffer.from("tao")), bittensorPDA.toBuffer()],
      program.programId
    );

    [taoStake] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(Buffer.from("tao_stake")), bittensorPDA.toBuffer()],
      program.programId
    );

    await program.methods
      .initializeBittensor()
      .accounts({
        bittensorState: bittensorPDA,
        taoMint,
        taoStake,
        owner: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: token.TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });

    const state = await program.account.bittensorState.fetch(bittensorPDA);

    console.log("State: ", state);

    const userTaoATA = await token.createAssociatedTokenAccount(
      connection,
      user,
      taoMint,
      user.publicKey
    );

    await program.methods
      .mint()
      .accounts({
        bittensorState: bittensorPDA,
        taoMint,
        userTaoAta: userTaoATA,
        owner: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: token.TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });

    const taoBalance = await connection.getTokenAccountBalance(userTaoATA);

    console.log("Tao balance: ", taoBalance);
  });

  it("Is initlialized subnet", async () => {
    [subnet1PDA] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("subnet_state"), user.publicKey.toBuffer()],
      program.programId
    );

    [subnet1WeightsPDA] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("weights"), subnet1PDA.toBuffer()],
      program.programId
    );

    [subnetTaoStake] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("tao_stake"), subnet1PDA.toBuffer()],
      program.programId
    );

    await program.methods
      .initializeSubnet()
      .accounts({
        taoMint,
        subnetState: subnet1PDA,
        bittensorState: bittensorPDA,
        subnetWeights: subnet1WeightsPDA,
        taoStake: subnetTaoStake,
        owner: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: token.TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });

    const subnet = await program.account.subnetState.fetch(subnet1PDA);
    const bittensor = await program.account.bittensorState.fetch(bittensorPDA);

    console.log("Subnet state: ", subnet);
    console.log("Bittensor state: ", bittensor);
  });
});
