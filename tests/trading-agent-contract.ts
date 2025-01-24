import * as anchor from "@coral-xyz/anchor";
import { AnchorError, Program } from "@coral-xyz/anchor";
import { PublicKey, Keypair, SendTransactionError } from "@solana/web3.js";
import { TradingAgentContract } from "../target/types/trading_agent_contract";
import { assert, expect } from "chai";

describe("trading-agent-contract", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .TradingAgentContract as Program<TradingAgentContract>;

  const payer = anchor.web3.Keypair.generate();
  const randomUser = anchor.web3.Keypair.generate();
  const teeKey = anchor.web3.Keypair.generate();

  const [agentData] = PublicKey.findProgramAddressSync(
    [Buffer.from("agent"), payer.publicKey.toBuffer()],
    program.programId
  );

  it("Airdrop to payer and random user", async () => {
    const tx = await anchor
      .getProvider()
      .connection.requestAirdrop(payer.publicKey, 10000000000);
    await anchor.getProvider().connection.confirmTransaction(tx);

    const tx2 = await anchor
      .getProvider()
      .connection.requestAirdrop(randomUser.publicKey, 10000000000);
    await anchor.getProvider().connection.confirmTransaction(tx2);
  });

  it("Initialize successfully", async () => {
    const tx = await program.methods
      .initializeAgent()
      .accounts({
        signer: payer.publicKey,
      })
      .signers([payer])
      .rpc();
    // Check the authority data
    const data = await program.account.agentData.fetch(agentData);
    assert.ok(data.admin.equals(payer.publicKey));
  });

  it("Reinitialize failed", async () => {
    try {
      const tx = await program.methods
        .initializeAgent()
        .accounts({
          signer: payer.publicKey,
        })
        .signers([payer])
        .rpc();
      assert(false, "should've failed here but didn't ");
    } catch (err) {
      expect(err).to.be.instanceOf(SendTransactionError);
    }
  });

  it("Update TEE key successfully", async () => {
    const tx = await program.methods
      .updateTeeKey({ teeKey: teeKey.publicKey })
      .accounts({
        signer: payer.publicKey,
      })
      .signers([payer])
      .rpc();

    // Check the TEE
    const data = await program.account.agentData.fetch(agentData);
    assert.ok(data.admin.equals(payer.publicKey));
    assert.ok(data.teeKey.equals(teeKey.publicKey));
  });

  it("Update TEE key failed - unauthorized signer", async () => {
    try {
      const tx = await program.methods
        .updateTeeKey({ teeKey: teeKey.publicKey })
        .accounts({
          signer: randomUser.publicKey,
        })
        .signers([randomUser])
        .rpc();
      assert(false, "should've failed here but didn't ");
    } catch (err) {
      expect(err).to.be.instanceOf(AnchorError);
      expect((err as AnchorError).error.errorCode.code).to.equal(
        "AccountNotInitialized"
      );
    }
  });
});
