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
  owner: anchor.web3.Keypair;
  taoATA: anchor.web3.PublicKey;
  validatorPDA: anchor.web3.PublicKey;
}

interface Miner {
  owner: anchor.web3.Keypair;
  taoATA: anchor.web3.PublicKey;
  minerPDA: anchor.web3.PublicKey;
  groupPubkey: anchor.web3.PublicKey;
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
  let subnet: Subnet;
  let validators: Validator[] = [];
  let miners: Miner[] = [];

  let owner: User;

  let bittensorPDA: anchor.web3.PublicKey;
  let bittensorEpochPDA: anchor.web3.PublicKey;
  let taoMint: anchor.web3.PublicKey;
  let taoStake: anchor.web3.PublicKey;

  async function createUser(): Promise<User> {
    const user = anchor.web3.Keypair.generate();

    const signature = await connection.requestAirdrop(
      user.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    const latestBlockHash = await connection.getLatestBlockhash();

    await connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature,
    });

    return { keypair: user, taoATA: null };
  }

  async function createATA(user: User) {
    const userTaoAta = await token.createAssociatedTokenAccount(
      connection,
      user.keypair,
      taoMint,
      user.keypair.publicKey
    );

    await program.methods
      .mint()
      .accounts({
        taoMint,
        userTaoAta,
        subnetState: subnet.subnetPDA,
        owner: user.keypair.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: token.TOKEN_PROGRAM_ID,
      })
      .signers([user.keypair])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });

    user.taoATA = userTaoAta;
  }

  function generateSubnet(user: User): Subnet {
    const userTaoAta = user.taoATA;

    const [subnetPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("subnet_state")],
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
    taoATA: anchor.web3.PublicKey
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
      owner,
      taoATA,
      validatorPDA,
    };
  }

  function generateMiner(
    owner: anchor.web3.Keypair,
    taoATA: anchor.web3.PublicKey
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
      owner,
      taoATA,
      minerPDA,
      groupPubkey: null,
    };
  }

  it("Is initlialized subnet", async () => {
    const [subnetPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("subnet_state")],
      program.programId
    );

    [taoMint] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(Buffer.from("tao")), subnetPDA.toBuffer()],
      program.programId
    );

    owner = await createUser();

    // airdrop some SOL to the user
    const sig = await connection.requestAirdrop(
      owner.keypair.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    const latestBlockHash = await connection.getLatestBlockhash();

    await connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: sig,
    });

    subnet = generateSubnet(owner);

    users = await Promise.all(new Array(32).fill(0).map(() => createUser()));

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
    } = subnet;

    program.methods
      .registerSubnet()
      .accounts({
        taoMint,
        subnetState: subnet.subnetPDA,
        owner: owner.keypair.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: token.TOKEN_PROGRAM_ID,
      })
      .signers([owner.keypair])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });

    await sleep(10000);

    await program.methods
      .registerSubnetMiners()
      .accounts({
        subnetState: subnet.subnetPDA,
        subnetValidators: subnet.subnetValidatorsPDA,
        taoStake: subnet.subnetTaoStake,
        taoMint,
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
        owner: owner.keypair.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([owner.keypair])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });

    await sleep(10000);

    await program.methods
      .registerSubnetWeights()
      .accounts({
        subnetState: subnet.subnetPDA,
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
        owner: owner.keypair.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([owner.keypair])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });

    await Promise.all(users.map(async (item) => createATA(item)));
    // const subnetsState = await program.account.subnetState.all();

    // console.log(
    //   "subnets state",
    //   subnetsState
    //     .map((item) => {
    //       return {
    //         id: item.account.id,
    //         owner: item.account.owner.toBase58(),
    //       };
    //     })
    //     .sort((a, b) => a.id - b.id)
    // );
  });

  it("Is initlialized Validator", async () => {
    // 每个用户注册 10 个 validator
    validators = users.map((user) =>
      generateValidator(user.keypair, user.taoATA)
    );

    console.log("validators", validators.length);

    // init validators
    await Promise.all(
      validators.map(async (validator) => {
        await sleep(3000);
        return program.methods
          .initializeSubnetValidator(new anchor.BN(2 * 10 ** 9))
          .accounts({
            taoMint: taoMint,
            userTaoAta: validator.taoATA,
            validatorState: validator.validatorPDA,
            taoStake: subnet.subnetTaoStake,
            subnetState: subnet.subnetPDA,
            subnetValidators: subnet.subnetValidatorsPDA,
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
    await Promise.all(
      validators.map(async (validator) => {
        await sleep(3000);
        program.methods
          .validatorStake(new anchor.BN(2 * 10 ** 9))
          .accounts({
            subnetState: subnet.subnetPDA,
            subnetValidators: subnet.subnetValidatorsPDA,
            taoMint: taoMint,
            taoStake: subnet.subnetTaoStake,
            userTaoAta: validator.taoATA,
            validatorState: validator.validatorPDA,
            owner: validator.owner.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: token.TOKEN_PROGRAM_ID,
          })
          .signers([validator.owner])
          .rpc()
          .catch((err) => {
            console.log("Error: ", err);
          });
      })
    );

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
    let newUser = await createUser();
    await createATA(newUser);

    let newValidator = generateValidator(newUser.keypair, newUser.taoATA);

    await program.methods
      .initializeSubnetValidator(new anchor.BN(2 * 10 ** 9))
      .accounts({
        taoMint: taoMint,
        userTaoAta: newValidator.taoATA,
        validatorState: newValidator.validatorPDA,
        taoStake: subnet.subnetTaoStake,
        subnetState: subnet.subnetPDA,
        subnetValidators: subnet.subnetValidatorsPDA,
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
    miners = users.map((user) => generateMiner(user.keypair, user.taoATA));

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
        } = subnet;

        miner.groupPubkey = subnetMiners;

        return program.methods
          .initializeSubnetMiner()
          .accounts({
            taoMint: taoMint,
            userTaoAta: miner.taoATA,
            minerState: miner.minerPDA,
            subnetState: subnet.subnetPDA,
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
          owner: item.account.owner.toBase58(),
          stake: item.account.stake.toString(),
          group: item.account.groupPubkey.toBase58(),
        };
      })
    );
  });

  it("set miner weights", async () => {
    // program.addEventListener("ValidatorSetWeightsEvent", (event) => {
    //   console.log("ValidatorSetWeightsEvent", event);
    // });

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
            subnetState: subnet.subnetPDA,
            validatorState: validator.validatorPDA,
            subnetValidators: subnet.subnetValidatorsPDA,
            minerWeights: subnet.minerWeights,
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

    const minerWeights = await program.account.minerWeights.fetch(
      subnet.minerWeights
    );

    console.log("miner weights: ", minerWeights.minersWeights.slice(0, 4));
  });
  

  it("end subnet miner weights", async () => {
    const {
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
    } = subnet;

    let weights = [
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
    ];

    await Promise.all(
      weights.map(async (item) => {
        await program.methods
          .endEpochWeights()
          .accounts({
            subnetState: subnet.subnetPDA,
            minerWeights: item,
            subnetValidators: subnet.subnetValidatorsPDA,
          })
          .rpc()
          .catch((err) => {
            console.log("Error: ", err);
          });

        await sleep(3000);

        return program.methods
          .endEpochWeights()
          .accounts({
            subnetState: subnet.subnetPDA,
            minerWeights: item,
            subnetValidators: subnet.subnetValidatorsPDA,
          })
          .rpc()
          .catch((err) => {
            console.log("Error: ", err);
          });
      })
    );

    let subnetState = await program.account.subnetState.fetch(subnet.subnetPDA);

    const weightsStates = await program.account.minerWeights.fetch(
      subnet.minerWeights
    );

    console.log(
      "miner weights: ",
      subnetState.epochTotalWeights.toString(),
      subnetState.weightsStaus,
      weightsStates.minerTotalWeights.map((item) => item.toString())
    );

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
    } = subnet;

    const miners = [
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
    ];

    await Promise.all(
      weights.map(async (item, i) => {
        return program.methods
          .rewardSubnetMiners()
          .accounts({
            subnetState: subnet.subnetPDA,
            subnetMiners: miners[i],
            minerWeights: item,
            subnetValidators: subnet.subnetValidatorsPDA,
            owner: subnet.user.keypair.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([subnet.user.keypair])
          .rpc()
          .catch((err) => {
            console.log("Error: ", err);
          });
      })
    );

    subnetState = await program.account.subnetState.fetch(subnet.subnetPDA);

    const subnetMinersState = await program.account.subnetMiners.all();

    console.log(
      "miners state",
      subnetState.weightsStaus,
      subnetMinersState.map((i) => {
        return i.account.miners.map((j) => {
          return j.reward.toString();
        });
      })
    );
  });

  it("subnet end epoch", async () => {
    await program.methods
      .rewardSubnetValidators()
      .accounts({
        subnetState: subnet.subnetPDA,
        subnetValidators: subnet.subnetValidatorsPDA,
      })
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });
  });

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
            subnetValidators: subnet.subnetValidatorsPDA,
            taoMint: taoMint,
            taoStake: subnet.subnetTaoStake,
            owner: validator.owner.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: token.TOKEN_PROGRAM_ID,
            subnetState: subnet.subnetPDA,
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
      miners.map((miner) => {
        return program.methods
          .minerReward()
          .accounts({
            taoMint: taoMint,
            taoStake: subnet.subnetTaoStake,
            subnetMiners: miner.groupPubkey,
            subnetState: subnet.subnetPDA,
            userTaoAta: miner.taoATA,
            minerState: miner.minerPDA,
            owner: miner.owner.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: token.TOKEN_PROGRAM_ID,
          })
          .signers([miner.owner])
          .rpc()
          .catch((err) => {
            console.log("Error: ", err);
          });
      })
    );

    const subnetState = await program.account.subnetState.fetch(
      subnet.subnetPDA
    );

    usersBalance = await Promise.all(
      users.map((user) => connection.getTokenAccountBalance(user.taoATA))
    );

    console.log(
      "users balance: ",
      usersBalance.map((item) => item.value.uiAmount)
    );
  });

  return;

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
