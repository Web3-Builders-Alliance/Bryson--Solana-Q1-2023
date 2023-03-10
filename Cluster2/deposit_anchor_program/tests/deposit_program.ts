import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { DepositProgram } from "../target/types/deposit_program";
import * as web3 from "@solana/web3.js";
import { associatedTokenProgram, Metaplex } from "@metaplex-foundation/js";
import { execSync } from "child_process";
import {
  createMint,
  getAssociatedTokenAddress,
  getMint,
  getOrCreateAssociatedTokenAccount,
  mintToChecked,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { ASSOCIATED_PROGRAM_ID } from "@project-serum/anchor/dist/cjs/utils/token";
import {
  mintCandyMachineV2Builder,
  tokenProgram,
} from "@metaplex-foundation/js";

describe("deposit", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
  const metaplex = Metaplex.make(provider.connection);
  const program = anchor.workspace.DepositProgram as Program<DepositProgram>;
  const deposit_account = anchor.web3.Keypair.generate();
  const deposit_authority = anchor.web3.Keypair.generate();
  let mint = anchor.web3.Keypair.generate();
  let usdc_authority = anchor.web3.Keypair.generate();

  let [pda_auth, pda_bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("auth"),
      deposit_account.publicKey.toBuffer(),
    ],
    program.programId
  );

  let [vault, sol_bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("vault"), pda_auth.toBuffer()],
    program.programId
  );

  execSync(
    `anchor idl init --filepath target/idl/deposit_program.json ${program.programId}`,
    { stdio: "inherit" }
  );

  before(async () => {
    let res = await provider.connection.requestAirdrop(
      deposit_authority.publicKey,
      100 * anchor.web3.LAMPORTS_PER_SOL
    );

    let latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });

  it("Is initialized!", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({
        depositAccount: deposit_account.publicKey,
        pdaAuth: pda_auth,
        depositAuthority: deposit_authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([deposit_account, deposit_authority])
      .rpc();

    console.log("Your transaction signature", tx);

    let result = await program.account.solAccount.fetch(
      deposit_account.publicKey
    );
    console.log(result);
  });

  it("Deposits native SOL", async () => {
    const deposit_amount = new anchor.BN(25 * anchor.web3.LAMPORTS_PER_SOL);
    const deposit_tx = await program.methods
      .deposit(deposit_amount)
      .accounts({
        depositAccount: deposit_account.publicKey,
        pdaAuth: pda_auth,
        vault: vault,
        depositAuthority: deposit_authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([deposit_authority])
      .rpc();

    let sol_vault_lamps = await provider.connection.getBalance(vault);
    console.log(sol_vault_lamps);

    let result = await program.account.solAccount.fetch(
      deposit_account.publicKey
    );
    console.log(result);
  });

  it("Withdraws native SOL", async () => {
    let withdraw_amount = new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL);

    const withdraw_native_tx = await program.methods
      .withdraw(withdraw_amount)
      .accounts({
        depositAccount: deposit_account.publicKey,
        pdaAuth: pda_auth,
        vault: vault,
        depositAuthority: deposit_authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([deposit_authority])
      .rpc();

    let vault_lamps = await provider.connection.getBalance(vault);
    console.log(vault_lamps);
  });

  it("Create Mock SPL Token", async () => {
    let spl_Token = await createMint(
      provider.connection,
      deposit_authority,
      usdc_authority.publicKey,
      usdc_authority.publicKey,
      6,
      mint,
      null,
      TOKEN_PROGRAM_ID
    );
    console.log(spl_Token);

    let mint_test = await getMint(
      provider.connection,
      mint.publicKey,
      null,
      TOKEN_PROGRAM_ID
    );
    console.log(mint_test);

    let deposit_auth_usdc_acct = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      deposit_authority,
      mint.publicKey,
      deposit_authority.publicKey,
      false,
      undefined,
      undefined,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_PROGRAM_ID
    );

    let mint_to_sig = await mintToChecked(
      provider.connection,
      deposit_authority,
      mint.publicKey,
      deposit_auth_usdc_acct.address,
      usdc_authority,
      200e6,
      6,
      [],
      undefined,
      TOKEN_PROGRAM_ID
    );
    console.log(mint_to_sig);
  });

  it("Deposit an SPL Token", async () => {
    let to_token_account = await getAssociatedTokenAddress(
      mint.publicKey,
      pda_auth,
      true,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_PROGRAM_ID
    );

    let from_token_account = await getAssociatedTokenAddress(
      mint.publicKey,
      deposit_authority.publicKey,
      false,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_PROGRAM_ID
    );

    let deposit_spl_tx = await program.methods
      .depositSpl(new anchor.BN(25e6))
      .accounts({
        depositAccount: deposit_account.publicKey,
        pdaAuth: pda_auth,
        toTokenAccount: to_token_account, //unsure how to get the ID??
        fromTokenAccount: from_token_account, // unsure how to get the ID??
        depositAuthority: deposit_authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: tokenProgram.address,
        tokenMint: mint.publicKey,
        associatedTokenProgram: associatedTokenProgram.address,
      })
      .signers([deposit_authority])
      .rpc();
    console.log(deposit_spl_tx);
  });

  xit("Withdraw an SPL Token", async () => {
    let to_token_account = getAssociatedTokenAddress(
      mint.publicKey,
      pda_auth,
      true,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_PROGRAM_ID
    );
    console.log(to_token_account);

    let from_token_account = getAssociatedTokenAddress(
      mint.publicKey,
      deposit_authority.publicKey,
      false,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_PROGRAM_ID
    );
    console.log(from_token_account);

    let withdraw_spl_tx = await program.methods
      .withdrawSpl(new anchor.BN(25e6))
      .accounts({})
      .signers([deposit_authority])
      .rpc();

    console.log(withdraw_spl_tx);
  });
});
