import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VotingSystem } from "../target/types/voting_system";
import { assert } from "chai";

describe("voting_system", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.VotingSystem as Program<VotingSystem>;

  const proposalName = "Learn Solana";

  // Derive the PDA for our proposal account based on the seeds used in Rust
  const [proposalPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("proposal"), Buffer.from(proposalName)],
    program.programId
  );

  it("Initializes a proposal", async () => {
    await program.methods
      .initializeProposal(proposalName)
      .accounts({
        proposal: proposalPda,
        signer: provider.wallet.publicKey,
      })
      .rpc();

    // Fetch the account data from the blockchain
    const proposalAccount = await program.account.proposal.fetch(proposalPda);
    
    assert.equal(proposalAccount.name, proposalName);
    assert.equal(proposalAccount.yesVotes.toNumber(), 0);
  });

  it("Casts a YES vote", async () => {
    await program.methods
      .castVote("yes")
      .accounts({
        proposal: proposalPda,
        signer: provider.wallet.publicKey,
      })
      .rpc();

    const proposalAccount = await program.account.proposal.fetch(proposalPda);
    assert.equal(proposalAccount.yesVotes.toNumber(), 1);
  });
});