import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Task2 } from "../target/types/task_2";
import {
  PublicKey,
  SystemProgram,
  Keypair,
  LAMPORTS_PER_SOL,
  AccountInfo
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
  getAccount
} from "@solana/spl-token";
import { Metadata } from "@metaplex-foundation/mpl-token-metadata";
import { assert } from "chai";
import * as fs from "fs";

describe("task_2", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Task2 as Program<Task2>;
  // https://solana.com/es/developers/courses/tokens-and-nfts/token-program#make-some-token-metadata
  const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s",
  );

  // 测试用户钱包
  const keypairPath = require("os").homedir() + "/.config/solana/id.json";
  const secretKey = JSON.parse(fs.readFileSync(keypairPath, "utf-8"));
  const keypair = Keypair.fromSecretKey(Uint8Array.from(secretKey));

  console.log("user wallet PublicKey:", keypair.publicKey.toBase58());
  let user = keypair;

  // stack账户
  let stackAccountPda: PublicKey;
  let pdaStackAccountPda: PublicKey;

  // mint账户
  let mintPda: PublicKey;
  let mintBump: number;

  // mint - user ATA账户
  let userAta: PublicKey;
  // mint - stack ATA账户
  let stackAta: PublicKey;


  // 初始化
  before(async () => {
    // mint账户
    const [pda, bump] = await PublicKey.findProgramAddressSync(
      [Buffer.from("mint")],
      program.programId
    );
    mintPda = pda;
    mintBump = bump;

    // user ATA 账户
    [userAta] = await PublicKey.findProgramAddressSync(
      [user.publicKey.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), mintPda.toBuffer() ],     
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    // stack PDA
    [stackAccountPda] = await PublicKey.findProgramAddressSync(
      [user.publicKey.toBuffer()],
      program.programId
    );

    [pdaStackAccountPda] = await PublicKey.findProgramAddressSync(
      [Buffer.from("stack"), user.publicKey.toBuffer()],
      program.programId
    );

    [stackAta] = await PublicKey.findProgramAddressSync(
      [pdaStackAccountPda.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), mintPda.toBuffer() ],      
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

  });

  
  it("Is createToken and mint!", async () => {
    // metadata账户
    const [metadataPda] = await PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mintPda.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID,
    );

    const metadata = {
      name: "Task Token 1",
      symbol: "TT1",
      uri: "https://raw.githubusercontent.com/solana-developers/opos-asset/main/assets/DeveloperPortal/metadata.json",
      decimals: 9,
    };

    // 创建token
    const tx = await program.methods
      .createToken(metadata)
      .accounts({
        payer: user.publicKey,
        mint: mintPda,
        metadata: metadataPda,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        token_metadata_program: TOKEN_METADATA_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .rpc();
    console.log("createToken transaction signature", tx);

    // 给用户铸币
    const amount = 1000 * LAMPORTS_PER_SOL;
    const mintTx = await program.methods
    .mintSpl(new anchor.BN(amount))
    .accounts({
      payer: user.publicKey,
      mint: mintPda,
      payerAta: userAta,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      token_metadata_program: TOKEN_METADATA_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    })
    .rpc();

    console.log("mint transaction signature", mintTx);

    // 查询用户代币余额
    const ataAccount = await getAccount(program.provider.connection, userAta);
    console.log("userAtaAddress ", userAta, " token balance:", ataAccount.amount.toString())
  });

  it("Is deposit!", async () => {
    const amount = 100 * LAMPORTS_PER_SOL;

    const tx = await program.methods
    .depositSpl(new anchor.BN(amount))
    .accounts({
      payer: user.publicKey,
      pdaStackAccount: pdaStackAccountPda,
      stackAccount: stackAccountPda,
      mint: mintPda,
      payerAta: userAta,
      stackAccountAta: stackAta,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    })
    .rpc();

    // 查询代币余额
    const ataAccount = await getAccount(program.provider.connection, userAta);
    console.log("userAtaAddress ", userAta, " token balance:", ataAccount.amount.toString())

    const stackAtaAccount = await getAccount(program.provider.connection, stackAta);
    console.log("stackAtaAddress ", stackAta, " token balance:", stackAtaAccount.amount.toString())

  });

  it("Is withdraw!", async () => {

    const tx = await program.methods
    .withdrawSpl()
    .accounts({
      payer: user.publicKey,
      pdaStackAccount: pdaStackAccountPda,
      stackAccount: stackAccountPda,
      mint: mintPda,
      userAta: userAta,
      stackAccountAta: stackAta,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    })
    .rpc();

    // 查询代币余额
    const ataAccount = await getAccount(program.provider.connection, userAta);
    console.log("userAtaAddress ", userAta, " token balance:", ataAccount.amount.toString())

    const stackAtaAccount = await getAccount(program.provider.connection, stackAta);
    console.log("stackAtaAddress ", stackAta, " token balance:", stackAtaAccount.amount.toString())

  });


});
