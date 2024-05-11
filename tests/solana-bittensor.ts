import * as anchor from "@coral-xyz/anchor";
import * as token from "@solana/spl-token";
import { Program } from "@coral-xyz/anchor";
import { SolanaBittensor } from "../target/types/solana_bittensor";

interface User {
  keypair: anchor.web3.Keypair;
  taoATA: anchor.web3.PublicKey;
}

interface Subnet {
  subnetPDA: anchor.web3.PublicKey;
  subnetMiners: anchor.web3.PublicKey;
  subnetMiners1: anchor.web3.PublicKey;
  subnetMiners2: anchor.web3.PublicKey;
  subnetMiners3: anchor.web3.PublicKey;
  subnetMiners4: anchor.web3.PublicKey;
  subnetMiners5: anchor.web3.PublicKey;
  subnetMiners6: anchor.web3.PublicKey;
  subnetMiners7: anchor.web3.PublicKey;
  subnetMiners8: anchor.web3.PublicKey;
  subnetMiners9: anchor.web3.PublicKey;
  minerWeights: anchor.web3.PublicKey;
  minerWeights1: anchor.web3.PublicKey;
  minerWeights2: anchor.web3.PublicKey;
  minerWeights3: anchor.web3.PublicKey;
  minerWeights4: anchor.web3.PublicKey;
  minerWeights5: anchor.web3.PublicKey;
  minerWeights6: anchor.web3.PublicKey;
  minerWeights7: anchor.web3.PublicKey;
  minerWeights8: anchor.web3.PublicKey;
  minerWeights9: anchor.web3.PublicKey;
  subnetValidatorsPDA: anchor.web3.PublicKey;
  subnetTaoStake: anchor.web3.PublicKey;
  userTaoAta: anchor.web3.PublicKey;
  user: User;
}

interface Validator {
  subnetID: number;
  owner: anchor.web3.Keypair;
  taoATA: anchor.web3.PublicKey;
  validatorPDA: anchor.web3.PublicKey;
  subnet: Subnet;
}

interface Miner {
  subnetID: number;
  owner: anchor.web3.Keypair;
  taoATA: anchor.web3.PublicKey;
  minerPDA: anchor.web3.PublicKey;
  subnet: Subnet;
}

function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

describe("solana-bittensor", () => {
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;

  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaBittensor as Program<SolanaBittensor>;

  let users: User[] = [];
  let subnets: Subnet[] = [];
  let validators: Validator[] = [];
  let miners: Miner[] = [];

  let bittensorOwner: anchor.web3.Keypair;
  let bittensorOwnerATA: anchor.web3.PublicKey;

  let bittensorPDA: anchor.web3.PublicKey;
  let bittensorEpochPDA: anchor.web3.PublicKey;
  let taoMint: anchor.web3.PublicKey;
  let taoStake: anchor.web3.PublicKey;

  async function createUser(taoMint: anchor.web3.PublicKey): Promise<User> {
    const user = anchor.web3.Keypair.generate();

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

    const taoATA = await token.createAssociatedTokenAccount(
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
        userTaoAta: taoATA,
        owner: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: token.TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });

    return { keypair: user, taoATA };
  }

  function generateSubnet(user: User): Subnet {
    const owenr = user.keypair.publicKey;
    const userTaoAta = user.taoATA;

    const [subnetPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("subnet_state"), owenr.toBuffer()],
      program.programId
    );

    const [subnetWeightsPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("subnet_epoch"), subnetPDA.toBuffer()],
      program.programId
    );

    let subnetMinersPDAs = [];
    let minerWeightsPDAs = [];

    for (let i = 0; i < 10; i++) {
      const [subnetMinersPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(`subnet_miners ${i}`), subnetPDA.toBuffer()],
        program.programId
      );

      const [minerWeightsPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(`miner_weights ${i}`), subnetPDA.toBuffer()],
        program.programId
      );

      subnetMinersPDAs.push(subnetMinersPDA);
      minerWeightsPDAs.push(minerWeightsPDA);
    }

    const [
      subnetMiners,
      subnetMiners1,
      subnetMiners2,
      subnetMiners3,
      subnetMiners4,
      subnetMiners5,
      subnetMiners6,
      subnetMiners7,
      subnetMiners8,
      subnetMiners9,
    ] = subnetMinersPDAs;

    const [
      minerWeights,
      minerWeights1,
      minerWeights2,
      minerWeights3,
      minerWeights4,
      minerWeights5,
      minerWeights6,
      minerWeights7,
      minerWeights8,
      minerWeights9,
    ] = minerWeightsPDAs;

    const [subnetValidatorsPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("subnet_validators"), subnetPDA.toBuffer()],
      program.programId
    );

    console.log("subnetPDA", subnetValidatorsPDA.toBase58());

    const [subnetTaoStake] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("tao_stake"), subnetPDA.toBuffer()],
      program.programId
    );

    return {
      subnetPDA,
      subnetMiners,
      subnetMiners1,
      subnetMiners2,
      subnetMiners3,
      subnetMiners4,
      subnetMiners5,
      subnetMiners6,
      subnetMiners7,
      subnetMiners8,
      subnetMiners9,
      minerWeights,
      minerWeights1,
      minerWeights2,
      minerWeights3,
      minerWeights4,
      minerWeights5,
      minerWeights6,
      minerWeights7,
      minerWeights8,
      minerWeights9,
      subnetValidatorsPDA,
      subnetTaoStake,
      userTaoAta,
      user,
    };
  }

  function generateValidator(
    owner: anchor.web3.Keypair,
    taoATA: anchor.web3.PublicKey,
    subnet: Subnet,
    index: number
  ): Validator {
    const [validatorPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("validator_state"),
        subnet.subnetPDA.toBuffer(),
        owner.publicKey.toBuffer(),
      ],
      program.programId
    );

    return {
      subnetID: index,
      owner,
      taoATA,
      validatorPDA,
      subnet,
    };
  }

  function generateMiner(
    owner: anchor.web3.Keypair,
    taoATA: anchor.web3.PublicKey,
    subnet: Subnet,
    index: number
  ): Miner {
    const [minerPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("miner_state"),
        subnet.subnetPDA.toBuffer(),
        owner.publicKey.toBuffer(),
      ],
      program.programId
    );

    return {
      subnetID: index,
      owner,
      taoATA,
      minerPDA,
      subnet,
    };
  }

  it("Is initialized bittensor!", async () => {
    bittensorOwner = anchor.web3.Keypair.generate();
    // airdrop some SOL to the user
    const sig = await connection.requestAirdrop(
      bittensorOwner.publicKey,
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
        owner: bittensorOwner.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: token.TOKEN_PROGRAM_ID,
      })
      .signers([bittensorOwner])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });

    bittensorOwnerATA = await token.createAssociatedTokenAccount(
      connection,
      bittensorOwner,
      taoMint,
      bittensorOwner.publicKey
    );

    await program.methods
      .mint()
      .accounts({
        bittensorState: bittensorPDA,
        taoMint,
        userTaoAta: bittensorOwnerATA,
        owner: bittensorOwner.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: token.TOKEN_PROGRAM_ID,
      })
      .signers([bittensorOwner])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });
  });

  it("Is initlialized subnet", async () => {
    program.addEventListener("RegisterBittensorSubnetEvent", (event) => {
      console.log("RegisterBittensorSubnetEvent", event);
    });

    users = await Promise.all(
      new Array(32).fill(0).map(() => createUser(taoMint))
    );

    subnets = users.slice(0, 3).map((item) => generateSubnet(item));

    await Promise.all(
      subnets.map(async (item, index) => {
        const {
          subnetMiners,
          subnetMiners1,
          subnetMiners2,
          subnetMiners3,
          subnetMiners4,
          subnetMiners5,
          subnetMiners6,
          subnetMiners7,
          subnetMiners8,
          subnetMiners9,
          minerWeights,
          minerWeights1,
          minerWeights2,
          minerWeights3,
          minerWeights4,
          minerWeights5,
          minerWeights6,
          minerWeights7,
          minerWeights8,
          minerWeights9,
        } = item;

        program.methods
          .registerSubnet()
          .accounts({
            taoMint,
            bittensorState: bittensorPDA,
            subnetState: item.subnetPDA,
            taoStake: item.subnetTaoStake,
            // subnetValidators: item.subnetValidatorsPDA,
            owner: users[index].keypair.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: token.TOKEN_PROGRAM_ID,
          })
          .signers([users[index].keypair])
          .rpc()
          .catch((err) => {
            console.log("Error: ", err);
          });

        await sleep(10000);

        await program.methods
          .registerSubnetMiners()
          .accounts({
            bittensorState: bittensorPDA,
            subnetState: item.subnetPDA,
            subnetValidators: item.subnetValidatorsPDA,
            subnetMiners,
            subnetMiners1,
            subnetMiners2,
            subnetMiners3,
            subnetMiners4,
            subnetMiners5,
            subnetMiners6,
            subnetMiners7,
            subnetMiners8,
            subnetMiners9,
            owner: users[index].keypair.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([users[index].keypair])
          .rpc()
          .catch((err) => {
            console.log("Error: ", err);
          });

        await sleep(10000);

        await program.methods
          .registerSubnetWeights()
          .accounts({
            bittensorState: bittensorPDA,
            subnetState: item.subnetPDA,
            minerWeights,
            minerWeights1,
            minerWeights2,
            minerWeights3,
            minerWeights4,
            minerWeights5,
            minerWeights6,
            minerWeights7,
            minerWeights8,
            minerWeights9,
            owner: users[index].keypair.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([users[index].keypair])
          .rpc()
          .catch((err) => {
            console.log("Error: ", err);
          });

        await sleep(10000);

        return program.methods
          .initializeSubnet()
          .accounts({
            taoMint,
            subnetState: item.subnetPDA,
            bittensorState: bittensorPDA,
            bittensorEpoch: bittensorEpochPDA,
            subnetMiners,
            subnetMiners1,
            subnetMiners2,
            subnetMiners3,
            subnetMiners4,
            subnetMiners5,
            subnetMiners6,
            subnetMiners7,
            subnetMiners8,
            subnetMiners9,
            subnetValidators: item.subnetValidatorsPDA,
            taoStake: item.subnetTaoStake,
            owner: users[index].keypair.publicKey,
            userTaoAta: item.userTaoAta,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: token.TOKEN_PROGRAM_ID,
          })
          .signers([users[index].keypair])
          .rpc()
          .catch((err) => {
            console.log("Error: ", err);
          });
      })
    );

    const bittensor = await program.account.bittensorState.fetch(bittensorPDA);
    const subnetsState = await program.account.subnetState.all();
    console.log(
      "Bittensor state: ",
      bittensor.subnets
        .slice(0, 3)
        .map((item) => {
          return {
            id: item.id,
            owner: item.owner.toBase58(),
          };
        })
        .sort((a, b) => a.id - b.id)
    );

    console.log(
      "subnets state",
      subnetsState
        .map((item) => {
          return {
            id: item.account.id,
            owner: item.account.owner.toBase58(),
          };
        })
        .sort((a, b) => a.id - b.id)
    );
  });

  return;

  it("Is initlialized Validator", async () => {
    // 每个用户注册 10 个 validator
    validators = users
      .map((user) => {
        return subnets.map((subnet, index) =>
          generateValidator(user.keypair, user.taoATA, subnet, index)
        );
      })
      .flat();

    console.log("validators", validators.length);

    // init validators
    await Promise.all(
      validators.map(async (validator) => {
        await sleep(3000);
        return program.methods
          .initializeSubnetValidator(new anchor.BN(2 * 10 ** 9))
          .accounts({
            bittensorState: bittensorPDA,
            taoMint: taoMint,
            userTaoAta: validator.taoATA,
            validatorState: validator.validatorPDA,
            taoStake: validator.subnet.subnetTaoStake,
            subnetState: validator.subnet.subnetPDA,
            subnetValidators: validator.subnet.subnetValidatorsPDA,
            owner: validator.owner.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: token.TOKEN_PROGRAM_ID,
          })
          .signers([validator.owner])
          .rpc()
          .catch((err) => {
            console.log("Error: ", err, validator);
          });
      })
    );

    // stake tao
    // await Promise.all(
    //   validators.map(async (validator) => {
    //     await sleep(3000);
    //     program.methods
    //       .validatorStake(new anchor.BN(2 * 10 ** 9))
    //       .accounts({
    //         bittensorState: bittensorPDA,
    //         subnetState: validator.subnet.subnetPDA,
    //         taoMint: taoMint,
    //         taoStake: validator.subnet.subnetTaoStake,
    //         userTaoAta: validator.taoATA,
    //         validatorState: validator.validatorPDA,
    //         owner: validator.owner.publicKey,
    //         systemProgram: anchor.web3.SystemProgram.programId,
    //         tokenProgram: token.TOKEN_PROGRAM_ID,
    //       })
    //       .signers([validator.owner])
    //       .rpc()
    //       .catch((err) => {
    //         console.log("Error: ", err);
    //       });
    //   })
    // );

    const validatorsState = await program.account.validatorState.all();
    const subnetValidatorsState = await program.account.subnetValidators.all();

    console.log(
      "validators state",
      validatorsState.map((item) => {
        return {
          id: item.account.id,
          owner: item.account.owner.toBase58(),
          stake: item.account.stake.toString(),
        };
      })
    );

    console.log(
      "stake info",
      subnetValidatorsState.map((item) => {
        return item.account.validators
          .filter((item) => +item.stake > 0)
          .sort((a, b) => a.id - b.id)
          .map((item) => {
            return {
              id: item.id,
              owner: item.owner.toBase58(),
              stake: item.stake.toString(),
            };
          });
      })
    );
  });

  it("register validator when validators is full", async () => {
    let newUser = await createUser(taoMint);
    let newValidator = generateValidator(
      newUser.keypair,
      newUser.taoATA,
      subnets[0],
      0
    );

    let subnet0Validators = await program.account.subnetState.fetch(
      subnets[0].subnetPDA
    );

    await program.methods
      .initializeSubnetValidator(new anchor.BN(2 * 10 ** 9))
      .accounts({
        bittensorState: bittensorPDA,
        taoMint: taoMint,
        userTaoAta: newValidator.taoATA,
        validatorState: newValidator.validatorPDA,
        taoStake: newValidator.subnet.subnetTaoStake,
        subnetState: newValidator.subnet.subnetPDA,
        subnetValidators: newValidator.subnet.subnetValidatorsPDA,
        owner: newValidator.owner.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: token.TOKEN_PROGRAM_ID,
      })
      .signers([newValidator.owner])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });
  });

  it("Is initlialized miner", async () => {
    miners = users
      .map((user) => {
        return subnets.map((subnet, index) =>
          generateMiner(user.keypair, user.taoATA, subnet, index)
        );
      })
      .flat();

    // init miners
    await Promise.all(
      miners.map(async (miner) => {
        await sleep(3000);
        const {
          subnetMiners,
          subnetMiners1,
          subnetMiners2,
          subnetMiners3,
          subnetMiners4,
          subnetMiners5,
          subnetMiners6,
          subnetMiners7,
          subnetMiners8,
          subnetMiners9,
        } = miner.subnet;
        return program.methods
          .initializeSubnetMiner()
          .accounts({
            bittensorState: bittensorPDA,
            taoMint: taoMint,
            userTaoAta: miner.taoATA,
            minerState: miner.minerPDA,
            subnetState: miner.subnet.subnetPDA,
            subnetMiners,
            subnetMiners1,
            subnetMiners2,
            subnetMiners3,
            subnetMiners4,
            subnetMiners5,
            subnetMiners6,
            subnetMiners7,
            subnetMiners8,
            subnetMiners9,
            owner: miner.owner.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: token.TOKEN_PROGRAM_ID,
          })
          .signers([miner.owner])
          .rpc()
          .catch((err) => {
            console.log("Error: ", err, miner);
          });
      })
    );

    const minersState = await program.account.minerState.all();

    console.log(
      "miners state",
      minersState.map((item) => {
        return {
          id: item.account.id,
          subnetId: item.account.subnetId,
          owner: item.account.owner.toBase58(),
          stake: item.account.stake.toString(),
        };
      })
    );
  });

  return;

  it("set miner weights", async () => {
    program.addEventListener("ValidatorSetWeightsEvent", (event) => {
      console.log("ValidatorSetWeightsEvent", event);
    });
    await Promise.all(
      validators.map((validator, i) => {
        let weights = [200, 300, 500];
        if (i == 0) {
          weights = [100, 100, 0];
        } else if (i == 10 || i == 20) {
          weights = [500, 400, 0];
        }

        return program.methods
          .setMinerWeights(weights)
          .accounts({
            subnetState: validator.subnet.subnetPDA,
            subnetEpoch: validator.subnet.subnetWeightsPDA,
            validatorState: validator.validatorPDA,
            owner: validator.owner.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([validator.owner])
          .rpc()
          .catch((err) => {
            console.log("Error: ", err);
          });
      })
    );

    const weightsState = await program.account.subnetEpochState.all();

    console.log(
      "weights state",
      weightsState.map((item) => {
        return item.account.minersWeights.map((item) => item.toString());
      })
    );
  });

  it("register bittensor validator", async () => {
    await Promise.all(
      validators.slice(0, 32).map(async (validator) => {
        await sleep(3000);
        return program.methods
          .registerBittensorValidator()
          .accounts({
            bittensorEpoch: bittensorEpochPDA,
            bittensorState: bittensorPDA,
            subnetState: validator.subnet.subnetPDA,
            subnetValidators: validator.subnet.subnetValidatorsPDA,
            validatorState: validator.validatorPDA,
            owner: validator.owner.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([validator.owner])
          .rpc()
          .catch((err) => {
            console.log("Error: ", err);
          });
      })
    );

    // const bittensorState = await program.account.bittensorState.fetch(
    //   bittensorPDA
    // );

    // console.log(bittensorState.validators);
  });

  it("set subnet weights", async () => {
    await Promise.all(
      validators.slice(0, 32).map(async (validator) => {
        await sleep(3000);
        program.methods
          .setSubnetWeights([
            new anchor.BN(500),
            new anchor.BN(200),
            new anchor.BN(300),
          ])
          .accounts({
            bittensorState: bittensorPDA,
            bittensorEpoch: bittensorEpochPDA,
            subnetState: validator.subnet.subnetPDA,
            validatorState: validator.validatorPDA,
            owner: validator.owner.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([validator.owner])
          .rpc()
          .catch((err) => {
            console.log("Error: ", err);
          });
      })
    );

    const bettensorEpoch = await program.account.bittensorEpochState.fetch(
      bittensorEpochPDA
    );

    console.log(
      "Bittensor epoch weights: ",
      bettensorEpoch.weights.slice(0, 3)
    );
  });

  it("bittensor end epoch", async () => {
    await program.methods
      .endEpoch()
      .accounts({
        bittensorState: bittensorPDA,
        bittensorEpoch: bittensorEpochPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });

    const bittensor = await program.account.bittensorState.fetch(bittensorPDA);

    console.log(
      "Bittensor state: ",
      bittensor.subnets.slice(0, 3).map((item) => {
        return {
          id: item.id,
          owner: item.owner.toBase58(),
          subnetState: item.subnetState.toBase58(),
          stake: item.stake.toString(),
        };
      })
    );
  });

  it("subnet end epoch", async () => {
    program.addEventListener("SubnetEpochEndEvent", (event) => {
      console.log("SubnetEpochEndEvent", event);
    });

    await Promise.all(
      subnets.map(async (subnet) => {
        let endSubnetMedians1 = await program.methods
          .endSubnetMedians1()
          .accounts({
            bittensorState: bittensorPDA,
            subnetState: subnet.subnetPDA,
            subnetEpoch: subnet.subnetWeightsPDA,
            systemProgram: anchor.web3.SystemProgram.programId,
            owner: subnet.user.keypair.publicKey,
          })
          .instruction();

        let endSubnetMedians2 = await program.methods
          .endSubnetMedians2()
          .accounts({
            bittensorState: bittensorPDA,
            subnetState: subnet.subnetPDA,
            subnetEpoch: subnet.subnetWeightsPDA,
            systemProgram: anchor.web3.SystemProgram.programId,
            owner: subnet.user.keypair.publicKey,
          })
          // .preInstructions([endSubnetMedians1])
          .instruction();

        let endSubnetEpoch1 = await program.methods
          .endSubnetEpoch1()
          .accounts({
            bittensorState: bittensorPDA,
            subnetState: subnet.subnetPDA,
            subnetEpoch: subnet.subnetWeightsPDA,
            subnetMiners: subnet.subnetMinersPDA,
            subnetValidators: subnet.subnetValidatorsPDA,
            systemProgram: anchor.web3.SystemProgram.programId,
            owner: subnet.user.keypair.publicKey,
          })
          // .preInstructions([endSubnetMedians2])
          .instruction();

        let endSubnetEpoch2 = await program.methods
          .endSubnetEpoch2()
          .accounts({
            bittensorState: bittensorPDA,
            subnetState: subnet.subnetPDA,
            subnetEpoch: subnet.subnetWeightsPDA,
            subnetMiners: subnet.subnetMinersPDA,
            subnetValidators: subnet.subnetValidatorsPDA,
            systemProgram: anchor.web3.SystemProgram.programId,
            owner: subnet.user.keypair.publicKey,
          })
          .instruction();

        //  await program.methods
        //   .endSubnetEpoch2()
        //   .accounts({
        //     bittensorState: bittensorPDA,
        //     subnetState: subnet.subnetPDA,
        //     subnetEpoch: subnet.subnetWeightsPDA,
        //     subnetMiners: subnet.subnetMinersPDA,
        //     subnetValidators: subnet.subnetValidatorsPDA,
        //     systemProgram: anchor.web3.SystemProgram.programId,
        //     owner: subnet.user.keypair.publicKey,
        //   })
        //   .preInstructions([endSubnetEpoch1])
        //   .signers([subnet.user.keypair])
        //   .rpc()
        //   .catch((err) => {
        //     console.log("Error: ", err);
        //   });

        return program.methods
          .endSubnetEpoch()
          .accounts({
            bittensorState: bittensorPDA,
            subnetState: subnet.subnetPDA,
            subnetEpoch: subnet.subnetWeightsPDA,
            subnetMiners: subnet.subnetMinersPDA,
            subnetValidators: subnet.subnetValidatorsPDA,
            systemProgram: anchor.web3.SystemProgram.programId,
            owner: subnet.user.keypair.publicKey,
          })
          .signers([subnet.user.keypair])
          .preInstructions([
            endSubnetMedians1,
            endSubnetMedians2,
            endSubnetEpoch1,
            endSubnetEpoch2,
          ])
          .rpc()
          .catch((err) => {
            console.log("Error: ", err);
          });
      })
    );

    // let subnetEpochState = await program.account.subnetEpochState.all();

    // console.log(
    //   subnetEpochState.map(
    //     ({ account: { minerTotalWeights, validatorTotalWeights } }) => {
    //       return {
    //         minerTotalWeights,
    //         validatorTotalWeights,
    //       };
    //     }
    //   )
    // );

    const subnetMinersState = await program.account.subnetMiners.all();
    const subnetValidatorsState = await program.account.subnetValidators.all();
    const weightsState = await program.account.subnetEpochState.all();

    console.log(
      "miners state: ",
      subnetMinersState.map((item) => {
        return item.account.miners.map((item) =>
          [item.reward.toString(), item.protection.toString()].toString()
        );
      })
    );

    console.log(
      "validators state: ",
      subnetValidatorsState.map((item) => {
        return item.account.validators.map((item) =>
          [item.reward.toString(), item.protection.toString()].toString()
        );
      })
    );

    console.log(
      "weights state3: ",
      weightsState.map((item) => {
        return item.account.minersWeights.map((item) => item.toString());
      })
    );
  });

  return;

  it("miners and validators withdraw reward", async () => {
    let usersBalance = await Promise.all(
      users.map((user) => connection.getTokenAccountBalance(user.taoATA))
    );

    console.log(
      "users balance before: ",
      usersBalance.map((item) => item.value.uiAmount)
    );

    await Promise.all(
      validators.map((validator) =>
        program.methods
          .validatorReward()
          .accounts({
            bittensorState: bittensorPDA,
            taoMint: taoMint,
            taoStake: validator.subnet.subnetTaoStake,
            owner: validator.owner.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: token.TOKEN_PROGRAM_ID,
            subnetState: validator.subnet.subnetPDA,
            userTaoAta: validator.taoATA,
            validatorState: validator.validatorPDA,
          })
          .signers([validator.owner])
          .rpc()
          .catch((err) => {
            console.log("Error: ", err);
          })
      )
    );

    usersBalance = await Promise.all(
      users.map((user) => connection.getTokenAccountBalance(user.taoATA))
    );

    console.log(
      "users balance after validator reward : ",
      usersBalance.map((item) => item.value.uiAmount)
    );

    await Promise.all(
      miners.map((miner) =>
        program.methods
          .minerReward()
          .accounts({
            bittensorState: bittensorPDA,
            taoMint: taoMint,
            taoStake: miner.subnet.subnetTaoStake,
            owner: miner.owner.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: token.TOKEN_PROGRAM_ID,
            subnetState: miner.subnet.subnetPDA,
            userTaoAta: miner.taoATA,
            minerState: miner.minerPDA,
          })
          .signers([miner.owner])
          .rpc()
          .catch((err) => {
            console.log("Error: ", err);
          })
      )
    );

    const subnetsState = await program.account.subnetState.all();

    console.log(
      "miners state: ",
      subnetsState.map((item) => {
        return item.account.miners.map((item) => item.reward.toString());
      })
    );

    console.log(
      "validators state: ",
      subnetsState.map((item) => {
        return item.account.validators.map((item) => item.reward.toString());
      })
    );

    usersBalance = await Promise.all(
      users.map((user) => connection.getTokenAccountBalance(user.taoATA))
    );

    console.log(
      "users balance: ",
      usersBalance.map((item) => item.value.uiAmount)
    );
  });

  it("register validator when validators is full", async () => {
    let newUser = await createUser(taoMint);
    let newValidator = generateValidator(
      newUser.keypair,
      newUser.taoATA,
      subnets[0],
      0
    );

    await program.methods
      .initializeSubnetValidator(new anchor.BN(2 * 10 ** 9))
      .accounts({
        bittensorState: bittensorPDA,
        taoMint: taoMint,
        userTaoAta: newValidator.taoATA,
        validatorState: newValidator.validatorPDA,
        taoStake: newValidator.subnet.subnetTaoStake,
        subnetState: newValidator.subnet.subnetPDA,
        subnetEpoch: newValidator.subnet.subnetWeightsPDA,
        owner: newValidator.owner.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: token.TOKEN_PROGRAM_ID,
      })
      .signers([newValidator.owner])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });

    let subnet0State = await program.account.subnetState.fetch(
      subnets[0].subnetPDA
    );

    let validators_ = subnet0State.validators.map((item) =>
      item.pda.toBase58()
    );

    console.log(
      "subnet 0 validators: ",
      validators_,
      newValidator.validatorPDA.toBase58()
    );

    const validatorsState = await program.account.validatorState.fetch(
      newValidator.validatorPDA
    );

    console.log("validator", validatorsState);

    let validatorWasKnockedOut = validators.find(
      (item) =>
        item.subnetID == 0 &&
        !validators_.includes(item.validatorPDA.toBase58())
    );

    await program.methods
      .initializeSubnetValidator(new anchor.BN(2 * 10 ** 9))
      .accounts({
        bittensorState: bittensorPDA,
        taoMint: taoMint,
        userTaoAta: validatorWasKnockedOut.taoATA,
        validatorState: validatorWasKnockedOut.validatorPDA,
        taoStake: validatorWasKnockedOut.subnet.subnetTaoStake,
        subnetState: validatorWasKnockedOut.subnet.subnetPDA,
        subnetEpoch: validatorWasKnockedOut.subnet.subnetWeightsPDA,
        owner: validatorWasKnockedOut.owner.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: token.TOKEN_PROGRAM_ID,
      })
      .signers([validatorWasKnockedOut.owner])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });

    subnet0State = await program.account.subnetState.fetch(
      subnets[0].subnetPDA
    );

    validators_ = subnet0State.validators.map((item) => item.pda.toBase58());

    console.log(
      "subnet 0 validators: ",
      validators_,
      validatorWasKnockedOut.validatorPDA.toBase58()
    );

    const validatorWasKnockedOutState =
      await program.account.validatorState.fetch(
        validatorWasKnockedOut.validatorPDA
      );

    console.log("validatorWasKnockedOutState", validatorWasKnockedOutState);
  });

  it("register miner when miners is full", async () => {
    let newUser = await createUser(taoMint);
    let newMiner = generateMiner(
      newUser.keypair,
      newUser.taoATA,
      subnets[0],
      0
    );

    await program.methods
      .initializeSubnetMiner()
      .accounts({
        bittensorState: bittensorPDA,
        taoMint: taoMint,
        userTaoAta: newMiner.taoATA,
        minerState: newMiner.minerPDA,
        subnetState: newMiner.subnet.subnetPDA,
        subnetEpoch: newMiner.subnet.subnetWeightsPDA,
        owner: newMiner.owner.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: token.TOKEN_PROGRAM_ID,
      })
      .signers([newMiner.owner])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });

    let subnet0State = await program.account.subnetState.fetch(
      subnets[0].subnetPDA
    );

    let miners_ = subnet0State.miners.map((item) => item.owner.toBase58());

    console.log(
      "subnet 0 miners: ",
      miners_,
      newMiner.owner.publicKey.toBase58()
    );

    let minerWasKnockedOut = miners.find(
      (item) =>
        item.subnetID == 0 && !miners_.includes(item.owner.publicKey.toBase58())
    );

    await program.methods
      .initializeSubnetMiner()
      .accounts({
        bittensorState: bittensorPDA,
        taoMint: taoMint,
        userTaoAta: minerWasKnockedOut.taoATA,
        minerState: minerWasKnockedOut.minerPDA,
        subnetState: minerWasKnockedOut.subnet.subnetPDA,
        subnetEpoch: minerWasKnockedOut.subnet.subnetWeightsPDA,
        owner: minerWasKnockedOut.owner.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: token.TOKEN_PROGRAM_ID,
      })
      .signers([minerWasKnockedOut.owner])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });

    subnet0State = await program.account.subnetState.fetch(
      subnets[0].subnetPDA
    );

    miners_ = subnet0State.miners.map((item) => item.owner.toBase58());

    console.log(
      "subnet 0 miners: ",
      miners_,
      minerWasKnockedOut.owner.publicKey.toBase58()
    );
  });

  it("knock out subnet", async () => {
    let newUser = await createUser(taoMint);
    let newSubnet = generateSubnet(newUser);

    // let subnetsState = await program.account.subnetState.all();
    // console.log(subnetsState.map((item) => item.account.owner.toBase58()));

    let bittensorState = await program.account.bittensorState.fetch(
      bittensorPDA
    );

    console.log(
      bittensorState.subnets.map((item) => item.subnetState.toBase58())
    );

    const register1 = await program.methods
      .registerSubnet()
      .accounts({
        bittensorState: bittensorPDA,
        subnetState: newSubnet.subnetPDA,
        subnetEpoch: newSubnet.subnetWeightsPDA,
        owner: newUser.keypair.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .instruction();

    await program.methods
      .initializeSubnet()
      .accounts({
        taoMint,
        subnetState: newSubnet.subnetPDA,
        bittensorState: bittensorPDA,
        bittensorEpoch: bittensorEpochPDA,
        subnetEpoch: newSubnet.subnetWeightsPDA,
        taoStake: newSubnet.subnetTaoStake,
        owner: newUser.keypair.publicKey,
        userTaoAta: newSubnet.userTaoAta,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: token.TOKEN_PROGRAM_ID,
      })
      .signers([newUser.keypair])
      .preInstructions([register1])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });

    bittensorState = await program.account.bittensorState.fetch(bittensorPDA);

    let subnets_ = bittensorState.subnets.map((item) =>
      item.subnetState.toBase58()
    );

    console.log(subnets_);

    let subnetWasKnockedOut = subnets.find(
      (item) => !subnets_.includes(item.subnetPDA.toBase58())
    );

    await program.methods
      .initializeSubnet()
      .accounts({
        taoMint,
        subnetState: subnetWasKnockedOut.subnetPDA,
        bittensorState: bittensorPDA,
        bittensorEpoch: bittensorEpochPDA,
        subnetEpoch: subnetWasKnockedOut.subnetWeightsPDA,
        taoStake: subnetWasKnockedOut.subnetTaoStake,
        owner: subnetWasKnockedOut.user.keypair.publicKey,
        userTaoAta: subnetWasKnockedOut.userTaoAta,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: token.TOKEN_PROGRAM_ID,
      })
      .signers([subnetWasKnockedOut.user.keypair])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });

    bittensorState = await program.account.bittensorState.fetch(bittensorPDA);

    console.log(
      bittensorState.subnets.map((item) => item.subnetState.toBase58())
    );
  });

  it("knock bittensor validator", async () => {
    let validator = validators[37];

    let bittensorState = await program.account.bittensorState.fetch(
      bittensorPDA
    );

    let bvs = bittensorState.validators.map((item) =>
      item.validatorState.toBase58()
    );

    await program.methods
      .registerBittensorValidator()
      .accounts({
        bittensorEpoch: bittensorEpochPDA,
        bittensorState: bittensorPDA,
        subnetState: validator.subnet.subnetPDA,
        validatorState: validator.validatorPDA,
        owner: validator.owner.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([validator.owner])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });

    bittensorState = await program.account.bittensorState.fetch(bittensorPDA);

    let bvs2 = bittensorState.validators.map((item) =>
      item.validatorState.toBase58()
    );
    console.log(bvs);
    console.log(bvs2);
    console.log(validator.validatorPDA.toBase58());
  });

  it("miners and validators unstake", async () => {
    // const validatorsState = await program.account.validatorState.all();
    // const minersState = await program.account.minerState.all();
    //   const subnetState = await program.account.subnetState.all();

    //   console.log(
    //     "miners state",
    //     minersState.map((item) => {
    //       return {
    //         id: item.account.id,
    //         subnetId: item.account.subnetId,
    //         owner: item.account.owner.toBase58(),
    //         stake: item.account.stake.toString(),
    //       };
    //     })
    //   );

    //   console.log(
    //     "stake info",
    //     subnetState.map((item) => {
    //       return item.account.miners
    //         .sort((a, b) => a.id - b.id)
    //         .map((item) => {
    //           return {
    //             id: item.id,
    //             owner: item.owner.toBase58(),
    //             stake: item.stake.toString(),
    //           };
    //         });
    //     })
    //   );

    // console.log(
    //   "validators state",
    //   validatorsState.map((item) => {
    //     return {
    //       id: item.account.id,
    //       owner: item.account.owner.toBase58(),
    //       stake: item.account.stake.toString(),
    //     };
    //   })
    // );

    // console.log(
    //   "stake info",
    //   subnetState.map((item) => {
    //     return item.account.validators
    //       .filter((item) => +item.stake > 0)
    //       .sort((a, b) => a.id - b.id)
    //       .slice(0, 3)
    //       .map((item) => {
    //         return {
    //           id: item.id,
    //           owner: item.owner.toBase58(),
    //           stake: item.stake.toString(),
    //         };
    //       });
    //   })
    // );

    await Promise.all(
      validators.map(async (validator) => {
        await sleep(3000);
        return program.methods
          .validatorUnstakes(new anchor.BN(1 * 10 ** 9))
          .accounts({
            bittensorState: bittensorPDA,
            taoMint: taoMint,
            taoStake: validator.subnet.subnetTaoStake,
            owner: validator.owner.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: token.TOKEN_PROGRAM_ID,
            subnetState: validator.subnet.subnetPDA,
            userTaoAta: validator.taoATA,
            validatorState: validator.validatorPDA,
          })
          .signers([validator.owner])
          .rpc()
          .catch((err) => {
            console.log("Error: ", err);
          });
      })
    );
  });
});
