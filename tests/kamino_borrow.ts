import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";

import { BN } from "@coral-xyz/anchor";
import {
  DEFAULT_RECENT_SLOT_DURATION_MS,
  KaminoAction,
  KaminoMarket,
  PROGRAM_ID,
  VanillaObligation,
} from "@kamino-finance/klend-sdk";
import { assert } from "chai";
import { KaminoBorrow } from "../target/types/kamino_borrow";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const connection = new Connection(
  "https://mainnet.helius-rpc.com/?api-key=91acf6dc-f1f0-4db8-9763-aff8b775fa6a"
);

const MAINNET_LENDING_MARKET = new PublicKey(
  "7u3HeHxYDLhnCoErrtycNokbQYbWGzLs6JSDqGAv5PfF"
);
const KAMINO_PROGRAM = new PublicKey(
  "KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD"
);

describe("kamino_borrow", () => {
  let payer = Keypair.generate();
  const deposit_amount = new BN(1 * 10 ** 9);
  const half_deposit_amount = deposit_amount.div(new BN(2));

  const program = anchor.workspace.KaminoBorrow as anchor.Program<KaminoBorrow>;
  it("deposit", async () => {
    try {
      const kaminoMarket = await KaminoMarket.load(
        connection,
        MAINNET_LENDING_MARKET,
        DEFAULT_RECENT_SLOT_DURATION_MS,
        PROGRAM_ID,
        true
      );

      const kaminoAction = await KaminoAction.buildDepositTxns(
        kaminoMarket,
        deposit_amount,
        new PublicKey("So11111111111111111111111111111111111111112"),
        payer.publicKey,
        new VanillaObligation(PROGRAM_ID),
        1_000_000,
        true
      );

      const borrowAction = await KaminoAction.buildBorrowTxns(
        kaminoMarket!,
        half_deposit_amount,
        new PublicKey("So11111111111111111111111111111111111111112"),
        payer.publicKey,
        new VanillaObligation(PROGRAM_ID),
        1000000,
        true,
        undefined,
        true,
        PublicKey.default
      );
      if (!borrowAction) {
        throw new Error("Borrow Action not generated properly");
      }

      console.log("Borrow Instructions:", borrowAction.lendingIxs);

      // Combine all instructions
      const allInstructions = [
        ...borrowAction.setupIxs,
        ...borrowAction.lendingIxs,
        ...borrowAction.cleanupIxs,
      ];

      const allAccountMetas = allInstructions.flatMap((ix) => ix.keys);
      const ixDatas: Buffer[] = allInstructions.map((ix) => ix.data);

      console.log("All Account Metas:", allAccountMetas);
    } catch (error) {
      console.error("Error:", error);
    }
  });
});
