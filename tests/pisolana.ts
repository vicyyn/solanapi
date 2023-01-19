import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Pisolana } from "../target/types/pisolana";
import * as wasm from "../pisolana-sdk/pkg";
import { PublicKey } from "@solana/web3.js";

describe("program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Pisolana as Program<Pisolana>;
  let pi_id = Math.floor(Math.random() * 999999999999999 - 1 + 1) + 1;
  const pi = wasm.get_pi_account(BigInt(pi_id));
  const hex_block = wasm.get_hex_block_account(BigInt(pi_id), BigInt(0));

  const pi_pubkey = new PublicKey(pi[0]);
  const pi_bump = pi[1];

  let hex_block_pubkey = new PublicKey(hex_block[0]);
  const hex_block_bump = hex_block[1];

  it("Initialize Pi", async () => {
    const tx = await program.methods
      .initializePi(new anchor.BN(pi_id), pi_bump)
      .accounts({ pi: pi_pubkey })
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it("Initialize Hex Block", async () => {
    const tx = await program.methods
      .initializeHexBlock(hex_block_bump)
      .accounts({ pi: pi_pubkey, hexBlock: hex_block_pubkey })
      .rpc();

    console.log("Your transaction signature", tx);
  });

  for (let i = 0; i < 10; i++) {
    it("Calculate Pi", async () => {
      const pi_account = await program.account.pi.fetch(pi_pubkey);
      const hex_block_account = await program.account.hexBlock.fetch(
        hex_block_pubkey
      );

      console.log(
        i,
        hex_block_account.hex,
        pi_account.currentPiIteration,
        pi_account.currentHexBlock
      );

      if (pi_account.currentHexBlock > hex_block_account.currentHexBlock) {
        hex_block_pubkey = new PublicKey(
          wasm.get_hex_block_account(
            BigInt(pi_id),
            BigInt(pi_account.currentHexBlock.toNumber())
          )[0]
        );
        const tx = await program.methods
          .initializeHexBlock(hex_block_bump)
          .accounts({ pi: pi_pubkey, hexBlock: hex_block_pubkey })
          .rpc();
        console.log("Your transaction signature", tx);
      }

      const tx = await program.methods
        .calculatePi(10)
        .accounts({ pi: pi_pubkey, hexBlock: hex_block_pubkey })
        .rpc();

      console.log("Your transaction signature", tx);
    });
  }
});
