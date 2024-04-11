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
  let bittensorEpochPDA: anchor.web3.PublicKey;
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
    [bittensorEpochPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("bittensor_epoch"), bittensorPDA.toBuffer()],
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
        bittensorEpoch: bittensorEpochPDA,
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

    userTaoATA = await token.createAssociatedTokenAccount(
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

    const bittensor = await program.account.bittensorState.fetch(bittensorPDA);
    const subnet = await program.account.subnetState.fetch(subnet1PDA);
    console.log("Bittensor state: ", bittensor.subnets[0], user.publicKey);
    console.log("subnet state", subnet.owner);
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
  });

  it("set miner weights", async () => {
    await program.methods
      .setMinerWeights([new anchor.BN(200)])
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

    console.log("Weights state: ", weights);
  });

  it("register bittensor validator", async () => {
    await program.methods
      .registerBittensorValidator()
      .accounts({
        bittensorState: bittensorPDA,
        subnetState: subnet1PDA,
        validatorState: validator1PDA,
        owner: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });

    const bittensor = await program.account.bittensorState.fetch(bittensorPDA);

    console.log("Bittensor validators: ", bittensor.validators[0]);
    console.log("Bittensor last id : ", bittensor.lastValidatorId);
  });

  it("set subnet weights", async () => {
    await program.methods
      .setSubnetWeights([new anchor.BN(200)])
      .accounts({
        bittensorState: bittensorPDA,
        bittensorEpoch: bittensorEpochPDA,
        subnetState: subnet1PDA,
        validatorState: validator1PDA,
        owner: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });

    const bettensorEpoch = await program.account.bittensorEpochState.fetch(
      bittensorEpochPDA
    );

    console.log("Bittensor epoch weights: ", bettensorEpoch.weights);
  });

  it("bittensor end epoch", async () => {
    const solbalance1 = await connection.getBalance(user.publicKey);

    // const modifyComputeUnits =
    //   anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
    //     units: 10000000,
    //   });

    // const addPriorityFee = anchor.web3.ComputeBudgetProgram.setComputeUnitPrice(
    //   {
    //     microLamports: 1,
    //   }
    // );

    await program.methods
      .endEpoch()
      .accounts({
        bittensorState: bittensorPDA,
        bittensorEpoch: bittensorEpochPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      // .preInstructions([modifyComputeUnits, addPriorityFee])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });

    const bittensor = await program.account.bittensorState.fetch(bittensorPDA);
    const solbalance = await connection.getBalance(user.publicKey);

    console.log("Bittensor state: ", bittensor.subnets[0]);
    console.log("User balance: ", solbalance1 - solbalance);
  });

  // it("subnet end epoch", async () => {
  //   await program.methods
  //     .endSubnetEpoch()
  //     .accounts({
  //       subnetState: subnet1PDA,
  //       subnetEpoch: subnet1WeightsPDA,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     })
  //     .rpc()
  //     .catch((err) => {
  //       console.log("Error: ", err);
  //     });

  //   const subnet = await program.account.subnetState.fetch(subnet1PDA);
  //   const weights = await program.account.subnetEpochState.fetch(
  //     subnet1WeightsPDA
  //   );

  //   console.log("Subnet state: ", subnet);

  //   console.log("Weights state: ", weights);
  // });
});
