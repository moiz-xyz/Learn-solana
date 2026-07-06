import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VotingSystem } from "../target/types/voting_system";
import { assert } from "chai";

describe("voting_system", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.VotingSystem as Program<VotingSystem>;
  const proposalName = "Learn Solana Fast";
  const duration = new anchor.BN(3600); // 1 hour duration

  const [proposalPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("proposal"), Buffer.from(proposalName)],
    program.programId
  );

  // Helper to derive the voter record PDA for any given wallet public key
  const getVoterPda = (walletPk: anchor.web3.PublicKey) => {
    return anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("voter"), proposalPda.toBuffer(), walletPk.toBuffer()],
      program.programId
    )[0];
  };

  it("Initializes a proposal with a deadline", async () => {
    await program.methods
      .initializeProposal(proposalName, duration)
      .accounts({
        proposal: proposalPda,
        signer: provider.wallet.publicKey,
      })
      .rpc();

    const proposalAccount = await program.account.proposal.fetch(proposalPda);
    assert.equal(proposalAccount.name, proposalName);
    assert.isTrue(proposalAccount.endTime.gt(proposalAccount.startTime));
  });

  it("Casts a YES vote successfully", async () => {
    const voterRecordPda = getVoterPda(provider.wallet.publicKey);

    await program.methods
      .castVote("yes")
      .accounts({
        proposal: proposalPda,
        voterRecord: voterRecordPda,
        signer: provider.wallet.publicKey,
      })
      .rpc();

    const proposalAccount = await program.account.proposal.fetch(proposalPda);
    assert.equal(proposalAccount.yesVotes.toNumber(), 1);
  });

  it("Fails if the same user attempts to vote a second time", async () => {
    const voterRecordPda = getVoterPda(provider.wallet.publicKey);

    try {
      await program.methods
        .castVote("no")
        .accounts({
          proposal: proposalPda,
          voterRecord: voterRecordPda,
          signer: provider.wallet.publicKey,
        })
        .rpc();
      assert.fail("The transaction should have failed due to double-voting constraints.");
    } catch (err) {
      // The runtime rejects the tx because the PDA initialization fails if it already exists
      assert.include(err.toString(), "already in use");
    }
  });
});
