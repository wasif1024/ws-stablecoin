import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PythSolanaReceiver } from "@pythnetwork/pyth-solana-receiver";
import { WsStablecoin } from "../target/types/ws_stablecoin";


describe("ws_stablecoin", () => {
  // Configure the client to use the local cluster.
  const provider=anchor.AnchorProvider.env();
  const connection=provider.connection;
  const wallet=provider.wallet as anchor.Wallet;
  anchor.setProvider(provider);
  const program = anchor.workspace.wsStablecoin as Program<WsStablecoin>;
  const pythsolanaReceiver=new PythSolanaReceiver({connection,wallet});
const SOL_PRICE_FEED_ID="0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
const solUsdPriceFeedAccount=pythsolanaReceiver.getPriceFeedAccountAddress(0,SOL_PRICE_FEED_ID);
const [collatoralAccount]=anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("collateral"),wallet.publicKey.toBuffer()],program.programId);

  

  it("initialized!", async () => {
    // Add your test here.
    const tx=await program.methods.initializeConfig().accounts({}).rpc({commitment:"confirmed",skipPreflight:true});
    console.log("Transaction signature", tx);
  });
  it("Deposit Collateral and Mint Token", async () => {
    const amountCollateral=new anchor.BN(1_000_000_000);
    const amountToMint=new anchor.BN(1_000_000_000);
    const tx=await program.methods.depositCollateralAndMintToken(amountCollateral,amountToMint).accounts({
      priceUpdate:solUsdPriceFeedAccount,
    }).rpc({commitment:"confirmed",skipPreflight:true});
    console.log("Deposit Collateral and Mint Token Transaction signature", tx);
  });
  it("Redeem Collateral and Burn Tokens", async () => {
    const amountCollateral=new anchor.BN(500_000_000);
    const amountToBurn=new anchor.BN(500_000_000);
    const tx=await program.methods.redeemCollateralAndBurnTokens(amountCollateral,amountToBurn).accounts({
      priceUpdate:solUsdPriceFeedAccount,
    }).rpc({commitment:"confirmed",skipPreflight:true});
    console.log("Redeem Collateral and Burn Tokens Transaction signature", tx);
  });
  it("update config", async () => {
    const minHealthFactor=new anchor.BN(100);
    const tx=await program.methods.updateConfig(minHealthFactor).accounts({}).rpc({commitment:"confirmed",skipPreflight:true});
    console.log("Update Config Transaction signature", tx);
  });
  it("liquidate", async () => {
    const amountToBurn=new anchor.BN(500_000_000);
    const tx=await program.methods.liquidate(amountToBurn).accounts({collateralAccount:collatoralAccount,priceUpdate:solUsdPriceFeedAccount,
    }).rpc({commitment:"confirmed",skipPreflight:true});
    console.log("Liquidate Transaction signature", tx);
  });
  it("update config", async () => {
    const minHealthFactor=new anchor.BN(1);
    const tx=await program.methods.updateConfig(minHealthFactor).accounts({}).rpc({commitment:"confirmed",skipPreflight:true});
    console.log("Update Config Transaction signature", tx);
  });
});
