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
  minerWeightsPDA: anchor.web3.PublicKey;
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
  let taoMint: anchor.web3.PublicKey;

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

    const [subnetMinersPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(`subnet_miners`), subnetPDA.toBuffer()],
      program.programId
    );

    const [minerWeightsPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(`miner_weights`), subnetPDA.toBuffer()],
      program.programId
    );

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
      subnetMiners: subnetMinersPDA,
      minerWeightsPDA,
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
      100 * anchor.web3.LAMPORTS_PER_SOL
    );

    const latestBlockHash = await connection.getLatestBlockhash();

    await connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: sig,
    });

    subnet = generateSubnet(owner);

    for (let i = 0; i < 30; i++) [users.push(await createUser())];
    // users = await Promise.all(
    //   new Array(300).fill(0).map(async () => {
    //     return createUser();
    //   })
    // );

    await program.methods
      .initializeSubnet()
      .accounts({
        taoMint,
        taoStake: subnet.subnetTaoStake,
        subnetState: subnet.subnetPDA,
        subnetMiners: subnet.subnetMiners,
        subnetValidators: subnet.subnetValidatorsPDA,
        minerWeights: subnet.minerWeightsPDA,
        owner: owner.keypair.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: token.TOKEN_PROGRAM_ID,
      })
      .signers([owner.keypair])
      .rpc()
      .catch((err) => {
        console.log("Error: ", err);
      });

    // await Promise.all(users.map((user) => createATA(user)));

    for (const user of users) {
      await createATA(user);
    }
  });

  it("increase miners", async () => {
    const subnetState = await program.account.subnetState.fetch(
      subnet.subnetPDA
    );

    let maxLen = Math.ceil((85 * subnetState.maxMiners) / 10240);

    console.log("maxMiners", subnetState.maxMiners);
    console.log("maxLen", maxLen);

    let len = 2;

    while (len <= maxLen) {
      console.log(len * 10240);
      try {
        await program.methods
          .increaseMiners(len * 10240)
          .accounts({
            subnetMiners: subnet.subnetMiners,
            minerWeights: subnet.minerWeightsPDA,
            signer: owner.keypair.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([owner.keypair])
          .rpc();
        // 查询 subnetMiners 的lamports

        let accountInfo = await connection.getAccountInfo(subnet.subnetMiners);

        // console.log("Account size: ", accountInfo.data.length);
        // if (accountInfo.data.length >= len * 10240) {
        //   len += 1;
        // }
        len += 1;
      } catch (e) {
        console.log("Error: ", e);
      }
      await sleep(1000);
    }

    let accountInfo = await connection.getAccountInfo(subnet.subnetMiners);
    let subnetMinersBalance = accountInfo.lamports;

    console.log("Account size: ", accountInfo.data.length, subnetMinersBalance);
  });

  it("increase miner weights", async () => {
    const subnetState = await program.account.subnetState.fetch(
      subnet.subnetPDA
    );

    let maxLen = Math.ceil((72 * subnetState.maxMiners) / 10240);

    console.log("maxMiners", subnetState.maxMiners);
    console.log("maxLen", maxLen);

    let len = 2;

    while (len <= maxLen) {
      console.log(len * 10240);
      try {
        await program.methods
          .increaseMinerWeights(len * 10240)
          .accounts({
            minerWeights: subnet.minerWeightsPDA,
            signer: owner.keypair.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([owner.keypair])
          .rpc();
        // 查询 subnetMiners 的lamports

        len += 1;
      } catch (e) {
        console.log("Error: ", e);
      }
      await sleep(1000);
    }

    let accountInfo = await connection.getAccountInfo(subnet.minerWeightsPDA);
    let balance = accountInfo.lamports;

    console.log("Account size: ", accountInfo.data.length, balance);
  });

  it("Is initlialized Validator", async () => {
    validators = users
      .slice(0, 32)
      .map((user) => generateValidator(user.keypair, user.taoATA));

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
      validatorsState
        .sort((a, b) => a.account.id - b.account.id)
        .map((item) => {
          return {
            id: item.account.id,
            owner: item.account.owner.toBase58(),
            stake: item.account.stake.toString(),
          };
        })
    );

    // console.log(
    //   "stake info",
    //   subnetValidatorsState.map((item) => {
    //     return item.account.validators
    //       .filter((item) => +item.stake > 0)
    //       .sort((a, b) => a.id - b.id)
    //       .map((item) => {
    //         return {
    //           id: item.id,
    //           owner: item.owner.toBase58(),
    //           stake: item.stake.toString(),
    //         };
    //       });
    //   })
    // );
  });

  // it("register validator when validators is full", async () => {
  //   let newUser = await createUser();
  //   await createATA(newUser);

  //   let newValidator = generateValidator(newUser.keypair, newUser.taoATA);

  //   await program.methods
  //     .initializeSubnetValidator(new anchor.BN(2 * 10 ** 9))
  //     .accounts({
  //       taoMint: taoMint,
  //       userTaoAta: newValidator.taoATA,
  //       validatorState: newValidator.validatorPDA,
  //       taoStake: subnet.subnetTaoStake,
  //       subnetState: subnet.subnetPDA,
  //       subnetValidators: subnet.subnetValidatorsPDA,
  //       owner: newValidator.owner.publicKey,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //       tokenProgram: token.TOKEN_PROGRAM_ID,
  //     })
  //     .signers([newValidator.owner])
  //     .rpc()
  //     .catch((err) => {
  //       console.log("Error: ", err);
  //     });
  // });

  it("Is initlialized miner", async () => {
    miners = users.map((user) => generateMiner(user.keypair, user.taoATA));

    let subnetMinersState = await program.account.subnetMiners.fetch(
      subnet.subnetMiners
    );

    console.log(
      subnetMinersState.miners.map((item) => {
        return {
          id: item.id,
          reward: item.reward.toString(10),
          lastWeight: item.lastWeight.toString(10),
        };
      })
    );

    for (const miner of miners) {
      await program.methods
        .initializeSubnetMiner()
        .accounts({
          taoMint: taoMint,
          userTaoAta: miner.taoATA,
          minerState: miner.minerPDA,
          subnetState: subnet.subnetPDA,
          minerWeights: subnet.minerWeightsPDA,
          subnetMiners: subnet.subnetMiners,
          owner: miner.owner.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: token.TOKEN_PROGRAM_ID,
        })
        .signers([miner.owner])
        .rpc()
        .catch((err) => {
          console.log("Error: ", err, miner);
        });

      subnetMinersState = await program.account.subnetMiners.fetch(
        subnet.subnetMiners
      );
      console.log(subnetMinersState.lastMinerId);
    }

    subnetMinersState = await program.account.subnetMiners.fetch(
      subnet.subnetMiners
    );

    console.log(
      subnetMinersState.miners.map((item) => {
        return {
          id: item.id,
          reward: item.reward.toString(10),
          lastWeight: item.lastWeight.toString(10),
        };
      })
    );

    console.log(subnetMinersState.lastMinerId);
  });

  it("set miner weights", async () => {
    // program.addEventListener("ValidatorSetWeightsEvent", (event) => {
    //   console.log("ValidatorSetWeightsEvent", event);
    // });

    await Promise.all(
      validators.map((validator, i) => {
        // weights 的累加值为 1000，随机分配给 5 个 miner
        // 使用随机函数分配

        function generateArrays(): {
          sumArray: number[];
          uniqueArray: number[];
        } {
          // Helper function to generate a random integer between min and max (inclusive)
          function getRandomInt(min: number, max: number): number {
            return Math.floor(Math.random() * (max - min + 1)) + min;
          }

          // Generate an array of 5 numbers that sum to 1000
          function generateSumArray(
            targetSum: number,
            length: number
          ): number[] {
            let array = new Array(length).fill(0);
            let remainingSum = targetSum;

            for (let i = 0; i < length - 1; i++) {
              // Generate a random number that is a multiple of 100
              let maxVal = Math.floor(remainingSum / 100);
              array[i] = getRandomInt(0, maxVal) * 100;
              remainingSum -= array[i];
            }
            array[length - 1] = remainingSum;

            // Shuffle the array to ensure randomness
            for (let i = array.length - 1; i > 0; i--) {
              const j = getRandomInt(0, i);
              [array[i], array[j]] = [array[j], array[i]];
            }

            return array;
          }

          // Generate an array of 5 unique numbers between 0 and 32
          function generateUniqueArray(
            min: number,
            max: number,
            length: number
          ): number[] {
            let set = new Set<number>();

            while (set.size < length) {
              set.add(getRandomInt(min, max));
            }

            return Array.from(set);
          }

          const sumArray = generateSumArray(1000, 5);
          const uniqueArray = generateUniqueArray(0, 5, 5);

          return { sumArray, uniqueArray };
        }

        // Example usage:
        const { sumArray: weights, uniqueArray: ids } = generateArrays();

        console.log("weights", weights, ids);

        return program.methods
          .setMinerWeights(weights, ids)
          .accounts({
            subnetState: subnet.subnetPDA,
            validatorState: validator.validatorPDA,
            subnetValidators: subnet.subnetValidatorsPDA,
            minerWeights: subnet.minerWeightsPDA,
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
      subnet.minerWeightsPDA
    );

    console.log("miner weights: ", minerWeights.minersWeights.slice(0, 8));
  });

  it("end subnet miner weights", async () => {
    let isEnd = false;

    while (!isEnd) {
      await program.methods
        .endEpochWeights()
        .accounts({
          subnetState: subnet.subnetPDA,
          minerWeights: subnet.minerWeightsPDA,
          subnetValidators: subnet.subnetValidatorsPDA,
          subnetMiners: subnet.subnetMiners,
        })
        .preInstructions([
          anchor.web3.ComputeBudgetProgram.requestHeapFrame({
            bytes: 8 * 32 * 1024,
          }),
          anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
            units: 1400_000,
          }),
        ])
        .rpc()
        .catch((err) => {
          console.log("Error: ", err);
        });

      await sleep(3000);

      const subnetState = await program.account.subnetState.fetch(
        subnet.subnetPDA
      );

      const minerWeightsState = await program.account.minerWeights.fetch(
        subnet.minerWeightsPDA
      );

      console.log("lastCalculateId", minerWeightsState.lastCalculateId);
      isEnd = subnetState.endStep === 1;
    }

    const minerWeights = await program.account.minerWeights.fetch(
      subnet.minerWeightsPDA
    );

    console.log("miner weights: ", minerWeights.minersWeights.slice(0, 8));
  });

  it("subnet end epoch", async () => {
    let isEnd = false;

    const minerWeights = await program.account.minerWeights.fetch(
      subnet.minerWeightsPDA
    );

    console.log("weights", minerWeights.minerTotalWeights);

    let subnetMiners = await program.account.subnetMiners.fetch(
      subnet.subnetMiners
    );

    console.log(
      subnetMiners.miners.map((item) => {
        return item.reward;
      })
    );

    while (!isEnd) {
      await program.methods
        .rewardSubnetMiners()
        .accounts({
          subnetState: subnet.subnetPDA,
          subnetMiners: subnet.subnetMiners,
          minerWeights: subnet.minerWeightsPDA,
          owner: owner.keypair.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .preInstructions([
          anchor.web3.ComputeBudgetProgram.requestHeapFrame({
            bytes: 8 * 32 * 1024,
          }),
          anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
            units: 1400_000,
          }),
        ])
        .signers([owner.keypair])
        .rpc()
        .catch((err) => {
          console.log("Error: ", err);
        });

      const subnetState = await program.account.subnetState.fetch(
        subnet.subnetPDA
      );

      const minerWeightsState = await program.account.minerWeights.fetch(
        subnet.minerWeightsPDA
      );

      console.log("lastRewardId", minerWeightsState.lastRewardId);

      isEnd = subnetState.endStep === 2;
    }

    subnetMiners = await program.account.subnetMiners.fetch(
      subnet.subnetMiners
    );

    console.log(
      subnetMiners.miners.map((item) => {
        return item.reward.toString();
      })
    );

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

    const subnetValidators = await program.account.subnetValidators.fetch(
      subnet.subnetValidatorsPDA
    );

    console.log(
      "subnet validators rewards",
      subnetValidators.validators.map((item) => {
        return item.reward.toString();
      })
    );
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
            subnetState: subnet.subnetPDA,
            subnetMiners: subnet.subnetMiners,
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

    // const minersInfo = await program.account.subnetMiners.fetch(
    //   subnet.subnetMiners
    // );

    // console.log(minersInfo.miners.slice(0, 30));

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
