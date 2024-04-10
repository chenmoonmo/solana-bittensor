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
  let userTaoATA: anchor.web3.PublicKey;

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

    [bittensorPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("bittensor")],
      program.programId
    );

    [taoMint] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(Buffer.from("tao")), bittensorPDA.toBuffer()],
      program.programId
    );

    [taoStake] = anchor.web3.PublicKey.findProgramAddressSync(
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

    userTaoATA = await token.createAssociatedTokenAccount(
      connection,
      user,
      taoMint,
      user.publicKey
    );

    console.log("User tao ATA: ", userTaoATA);

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
      [Buffer.from("subnet_epoch"), subnet1PDA.toBuffer()],
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
        subnetEpoch: subnet1WeightsPDA,
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
    const subnetEpoch = await program.account.subnetEpochState.fetch(
      subnet1WeightsPDA
    );
    const bittensor = await program.account.bittensorState.fetch(bittensorPDA);

    console.log("Subnet state: ", subnet);
    console.log("Subnet epoch state: ", subnetEpoch);
    console.log("Bittensor state: ", bittensor);
  });

  it("Is initlialized Validator", async () => {
    [validator1PDA] = await anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("validator_state"),
        subnet1PDA.toBuffer(),
        user.publicKey.toBuffer(),
      ],
      program.programId
    );

    await program.methods
      .initializeSubnetValidator()
      .accounts({
        bittensorState: bittensorPDA,
        taoMint: taoMint,
        userTaoAta: userTaoATA,
        validatorState: validator1PDA,
        subnetState: subnet1PDA,
        owner: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: token.TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });

    const validator = await program.account.validatorState.fetch(validator1PDA);
    const subnet = await program.account.subnetState.fetch(subnet1PDA);

    console.log("Validator state: ", validator);
    console.log("Subnet state: ", subnet);
  });

  it("Is initlialized Miner", async () => {
    [miner1PDA] = await anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("miner_state"),
        subnet1PDA.toBuffer(),
        user.publicKey.toBuffer(),
      ],
      program.programId
    );

    await program.methods
      .initializeSubnetMiner()
      .accounts({
        bittensorState: bittensorPDA,
        taoMint: taoMint,
        userTaoAta: userTaoATA,
        minerState: miner1PDA,
        subnetState: subnet1PDA,
        owner: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });

    const miner = await program.account.minerState.fetch(miner1PDA);
    const subnet = await program.account.subnetState.fetch(subnet1PDA);

    console.log("miner state: ", miner);
    console.log("Subnet state: ", subnet);
  });

  it("set_miner_weights", async () => {
    await program.methods
      .setMinerWeights([new anchor.BN(1)], [new anchor.BN(200)])
      .accounts({
        subnetState: subnet1PDA,
        subnetEpoch: subnet1WeightsPDA,
        validatorState: validator1PDA,
        owner: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });

    const weights = await program.account.subnetEpochState.fetch(
      subnet1WeightsPDA
    );

    console.log("Weights state: ", weights.validatorWeights[0]);
  });
});
