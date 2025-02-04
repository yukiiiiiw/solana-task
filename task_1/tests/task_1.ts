import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Task1 } from "../target/types/task_1";
import { PublicKey, SystemProgram, Keypair, LAMPORTS_PER_SOL, AccountInfo } from "@solana/web3.js";
import { assert } from "chai";
import * as fs from "fs";

describe("task1", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider;
  anchor.setProvider(provider.env());
  const program = anchor.workspace.Task1 as Program<Task1>;

  // 用户钱包
  const keypairPath = require("os").homedir() + "/.config/solana/id.json";
  const secretKey = JSON.parse(fs.readFileSync(keypairPath, "utf-8"));
  const keypair = Keypair.fromSecretKey(Uint8Array.from(secretKey));

  console.log("user wallet PublicKey:", keypair.publicKey.toBase58());

  let user = keypair;
  // 质押账户
  let stackPDA: PublicKey;
  let stackAccountData: PublicKey;
  // 充值金额
  const depositAmount = 0.1 * LAMPORTS_PER_SOL; // 0.1 SOL

  it("Creates a StackAccount PDA", async () => {
    // 生成PDA
    [stackPDA] = await PublicKey.findProgramAddressSync(
      [Buffer.from("stack"), user.publicKey.toBuffer()],
      program.programId
    );

    [stackAccountData] = await PublicKey.findProgramAddressSync(
      [user.publicKey.toBuffer()],
      program.programId
    );

    // 空投 SOL 给用户
    // await program.provider.connection.requestAirdrop(user.publicKey, 2 * LAMPORTS_PER_SOL);
    const userBalance = await program.provider.connection.getBalance(user.publicKey);
    console.log("[", user.publicKey, "]user balance:", userBalance / LAMPORTS_PER_SOL);
  });

  it("Deposit", async () => {
    const tx = await program.methods
      .deposit(new anchor.BN(depositAmount))
      .accounts({
        user: user.publicKey,
        stackAccountPda: stackPDA,
        stackAccount: stackAccountData,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    // 查询质押余额
    const account = await program.account.stackAccount.fetch(stackAccountData);
    console.log("[", stackAccountData, "]deposit transaction successful! tx: ", tx, "stack balance: ", account.balance.toNumber);
  });

  it("Deposit2", async () => {
    const tx = await program.methods
      .deposit(new anchor.BN(depositAmount))
      .accounts({
        user: user.publicKey,
        stackAccountPda: stackPDA,
        stackAccount: stackAccountData,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    // 查询质押余额
    const account = await program.account.stackAccount.fetch(stackAccountData);
    console.log("[", stackAccountData, "]deposit transaction successful! tx: ", tx, "stack balance: ", account.balance.toNumber);
  });

  it("Withdraw", async () => {
    // 获取 stackAccount 的当前余额
    const stackAccountBalance = await program.account.stackAccount.fetch(stackAccountData);
    const balanceBeforeWithdraw = stackAccountBalance.balance.toNumber();

    // 调用 withdraw 方法
    const tx = await program.methods
      .withdraw()
      .accounts({
        user: user.publicKey,
        stackAccountPda: stackPDA,
        stackAccount: stackAccountData,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    // 检查stackAccount的余额
    const updatedStackBalance = await program.provider.connection.getBalance(stackAccountData);;
    console.log("[", stackAccountData, "]withdraw transaction successful! tx: ", tx, "stack balance: ", updatedStackBalance);
    assert.strictEqual(updatedStackBalance.toString, LAMPORTS_PER_SOL.toString);

    // 检查用户账户余额
    const updateUserBalance = await program.provider.connection.getBalance(user.publicKey);;
    console.log("[", user.publicKey, "]balance: ", updateUserBalance);
    assert.isAbove(updateUserBalance, balanceBeforeWithdraw);
  });


});



