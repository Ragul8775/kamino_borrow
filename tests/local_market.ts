import * as anchor from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";
import {
  KaminoMarket,
  LendingMarket,
  DEFAULT_RECENT_SLOT_DURATION_MS,
  lendingMarketAuthPda,
  initLendingMarket,
  InitLendingMarketArgs,
  InitLendingMarketAccounts,

} from "@kamino-finance/klend-sdk";
import { assert } from "chai";

// Set up Anchor provider
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

// Establish connection to local validator
const connection = new Connection("http://127.0.0.1:8899", "finalized");  // use "finalized" to ensure full confirmation
const PROGRAM_ID = new PublicKey("FbUYVvxQFmGLL5bRMiJtHWTdeiw24akD5D5c7wpUesCP");

// Keypair of the payer (Admin)
const payer = Keypair.generate();

async function createLendingMarket() {
  const args: InitLendingMarketArgs = {
    quoteCurrency: Array(32).fill(0),  // Setting up the currency, can be USDC or another token
  };

  const lendingMarketAccount = Keypair.generate();
  const size = LendingMarket.layout.span + 8;  // Calculate the size of the account

  const [lendingMarketAuthority] = lendingMarketAuthPda(lendingMarketAccount.publicKey, PROGRAM_ID);

  const createMarketIx = SystemProgram.createAccount({
    fromPubkey: payer.publicKey,
    newAccountPubkey: lendingMarketAccount.publicKey,
    lamports: await connection.getMinimumBalanceForRentExemption(size),
    space: size,
    programId: PROGRAM_ID,
  });

  const accounts: InitLendingMarketAccounts = {
    lendingMarketOwner: payer.publicKey,
    lendingMarket: lendingMarketAccount.publicKey,
    lendingMarketAuthority: lendingMarketAuthority,
    systemProgram: SystemProgram.programId,
    rent: anchor.web3.SYSVAR_RENT_PUBKEY,
  };

  const ix = initLendingMarket(args, accounts);

  const { blockhash } = await connection.getLatestBlockhash();  
  const tx = new Transaction({
    recentBlockhash: blockhash,
    feePayer: payer.publicKey,
  }).add(createMarketIx, ix);



  console.log("Lending Market Created: ", lendingMarketAccount.publicKey.toBase58());


  return lendingMarketAccount.publicKey;
}

// Mocha test case
describe("kamino_borrow", () => {
  it("should create a lending market", async function () {
    this.timeout(60000); 
    
    const airdropSignature = await connection.requestAirdrop(
      payer.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(airdropSignature, "finalized");

    // Create the lending market
    const lendingMarketAddress = await createLendingMarket();
    console.log("Newly created Lending Market Address: ", lendingMarketAddress.toBase58());


    assert.isTrue(lendingMarketAddress instanceof PublicKey, "Lending market address should be a PublicKey");
  });
});
