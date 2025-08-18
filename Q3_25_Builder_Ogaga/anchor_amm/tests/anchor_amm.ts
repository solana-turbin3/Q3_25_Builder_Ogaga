import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorAmm } from "../target/types/anchor_amm";

describe("anchor_amm", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.anchorAmm as Program<AnchorAmm>;

  it("Is initialized!", async () => {
    // Add your test here.
    const seed = new anchor.BN(12345);
    const fee = 30; // 0.3% fee
    const authority = anchor.web3.Keypair.generate().publicKey;
    
    const tx = await program.methods.initialize(seed, fee, authority).rpc();
    console.log("Your transaction signature", tx);
  });
});
