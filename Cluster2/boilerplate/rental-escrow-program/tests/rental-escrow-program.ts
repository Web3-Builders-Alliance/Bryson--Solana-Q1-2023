import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { RentalEscrowProgram } from "../target/types/rental_escrow_program";

describe("rental-escrow-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.RentalEscrowProgram as Program<RentalEscrowProgram>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
