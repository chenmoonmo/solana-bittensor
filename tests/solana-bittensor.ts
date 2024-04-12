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
  subnetWeightsPDA: anchor.web3.PublicKey;
  subnetTaoStake: anchor.web3.PublicKey;
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
      anchor.web3.LAMPORTS_PER_SOL
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

  function generateSubnet(owenr: anchor.web3.PublicKey) {
    const [subnetPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("subnet_state"), owenr.toBuffer()],
      program.programId
    );

    const [subnetWeightsPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("subnet_epoch"), subnetPDA.toBuffer()],
      program.programId
    );

    const [subnetTaoStake] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("tao_stake"), subnetPDA.toBuffer()],
      program.programId
    );

    return {
      subnetPDA,
      subnetWeightsPDA,
      subnetTaoStake,
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
    users = await Promise.all(
      new Array(3).fill(0).map(() => createUser(taoMint))
    );
    subnets = users.map((item) => generateSubnet(item.keypair.publicKey));

    await Promise.all(
      subnets.map((item, index) =>
        program.methods
          .initializeSubnet()
          .accounts({
            taoMint,
            subnetState: item.subnetPDA,
            bittensorState: bittensorPDA,
            subnetEpoch: item.subnetWeightsPDA,
            taoStake: item.subnetTaoStake,
            owner: users[index].keypair.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: token.TOKEN_PROGRAM_ID,
          })
          .signers([users[index].keypair])
          .rpc()
          .catch((err) => {
            console.log("Error: ", err);
          })
      )
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

  it("Is initlialized Validator", async () => {
    // 每个用户注册 10 个 validator
    validators = users
      .map((user) => {
        return subnets.map((subnet, index) =>
          generateValidator(user.keypair, user.taoATA, subnet, index)
        );
      })
      .flat();

    // console.log("validators", validators);

    // init validators
    await Promise.all(
      validators.map((validator) =>
        program.methods
          .initializeSubnetValidator()
          .accounts({
            bittensorState: bittensorPDA,
            taoMint: taoMint,
            userTaoAta: validator.taoATA,
            validatorState: validator.validatorPDA,
            subnetState: validator.subnet.subnetPDA,
            owner: validator.owner.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: token.TOKEN_PROGRAM_ID,
          })
          .signers([validator.owner])
          .rpc()
          .catch((err) => {
            console.log("Error: ", err, validator);
          })
      )
    );

    // stake tao
    await Promise.all(
      validators.map((validator) =>
        program.methods
          .validatorStake(new anchor.BN(100 * 10 ** 9))
          .accounts({
            bittensorState: bittensorPDA,
            subnetState: validator.subnet.subnetPDA,
            taoMint: taoMint,
            taoStake: validator.subnet.subnetTaoStake,
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
          })
      )
    );

    const validatorsState = await program.account.validatorState.all();
    const subnetState = await program.account.subnetState.all();

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
      subnetState.map((item) => {
        return item.account.validators
          .filter((item) => +item.stake > 0)
          .sort((a, b) => a.id - b.id)
          .slice(0, 3)
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

  it("Is initlialized Miner", async () => {
    miners = users
      .map((user) => {
        return subnets.map((subnet, index) =>
          generateMiner(user.keypair, user.taoATA, subnet, index)
        );
      })
      .flat();

    // init miners
    await Promise.all(
      miners.map((miner) =>
        program.methods
          .initializeSubnetMiner()
          .accounts({
            bittensorState: bittensorPDA,
            taoMint: taoMint,
            userTaoAta: miner.taoATA,
            minerState: miner.minerPDA,
            subnetState: miner.subnet.subnetPDA,
            owner: miner.owner.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: token.TOKEN_PROGRAM_ID,
          })
          .signers([miner.owner])
          .rpc()
          .catch((err) => {
            console.log("Error: ", err, miner);
          })
      )
    );

    const minersState = await program.account.minerState.all();
    // TODO:
    console.log(
      "miners state",
      minersState.map((item) => {
        return {
          id: item.account.id,
          subnetId: item.account.subnetId,
          owner: item.account.owner.toBase58(),
        };
      })
    );
  });

  it("set miner weights", async () => {
    await Promise.all(
      validators.map((validator) =>
        program.methods
          .setMinerWeights([
            new anchor.BN(200),
            new anchor.BN(300),
            new anchor.BN(500),
          ])
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
          })
      )
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
      validators.map((validator) =>
        program.methods
          .registerBittensorValidator()
          .accounts({
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
          })
      )
    );

    const bittensorState = await program.account.bittensorState.fetch(
      bittensorPDA
    );

    console.log(bittensorState.validators);
  });

  it("set subnet weights", async () => {
    await Promise.all(
      validators.map((validator) =>
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
          })
      )
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

    console.log("Bittensor state: ", bittensor.subnets.slice(0, 3));
  });

  it("subnet end epoch", async () => {
    await Promise.all(
      subnets.map((subnet) =>
        program.methods
          .endSubnetEpoch()
          .accounts({
            bittensorState: bittensorPDA,
            subnetState: subnet.subnetPDA,
            subnetEpoch: subnet.subnetWeightsPDA,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .rpc()
          .catch((err) => {
            console.log("Error: ", err);
          })
      )
    );

    const subnetsState = await program.account.subnetState.all();
    const weightsState = await program.account.subnetEpochState.all();

    console.log(
      "miners state: ",
      subnetsState.map((item) => {
        return item.account.miners.map((item) => item.reward.toString());
      })
    );

    console.log(
      "weights state: ",
      weightsState.map((item) => {
        return item.account.minersWeights.map((item) => item.toString());
      })
    );
  });
});
