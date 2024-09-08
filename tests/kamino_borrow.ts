import * as anchor from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { BN } from "@coral-xyz/anchor";
import {
  DEFAULT_RECENT_SLOT_DURATION_MS,
  KaminoAction,
  KaminoMarket,
  PROGRAM_ID,
  VanillaObligation,
} from "@kamino-finance/klend-sdk";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const connection = new Connection("http://127.0.0.1:8899", "confirmed");

// Local lending market and program addresses
const LOCAL__LENDING_MARKET = new PublicKey("9jVRjgPrGzLwzXpFR3iWCfh5DgMGj3KBtokkfR3GnWA8");
const LOCAL_DEPLOY_ID = new PublicKey("FbUYVvxQFmGLL5bRMiJtHWTdeiw24akD5D5c7wpUesCP");

describe("kamino_borrow", function () {
  this.timeout(60000);  
  
  let payer = Keypair.generate();
  const deposit_amount = new BN(1 * 10 ** 9);
  const half_deposit_amount = deposit_amount.div(new BN(2));

  it("deposit", async function () {
    try {
      // Load the lending market
      const market = await KaminoMarket.load(
        connection,
        LOCAL__LENDING_MARKET,
        DEFAULT_RECENT_SLOT_DURATION_MS,
        LOCAL_DEPLOY_ID
      );
      
      if (!market) {
        throw new Error("Failed to load the market. Make sure the market is initialized correctly.");
      }

      // Load the reserves for the market
      await market.loadReserves();
      console.log("Loaded Reserves: ", market.reserves);

      // Check if reserves exist and fetch USDC reserve
      const usdReserve = market.getReserveBySymbol("USDC");
      if (usdReserve) {
        console.log("USDC Reserve Mint Supply: ", usdReserve.stats.mintTotalSupply.toString());
      } else {
        console.log("No USDC reserve found in the market.");
      }
    } catch (error) {
      console.error("Error:", error);
    }
  });
});
