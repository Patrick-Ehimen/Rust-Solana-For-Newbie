import { Connection, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { keypair } from "../keypairs/generate-keypair";

// const publicKey = new PublicKey(keypair.publicKey.toBase58());
const publicKey = new PublicKey("AXjvtHF6siGu56EpKM1R9rVQJkVAxRt3sRHsB75xUbGY");
// const publicKey = new PublicKey("7sD5Yi3U4BqQAbiqYyiL8Bw5gV2zoAZED2xavRBrTVNt");

console.log("publickey", publicKey);

const connection = new Connection("https://api.devnet.solana.com", "confirmed");

try {
  const balanceInLamports = await connection.getBalance(publicKey);
  const balanceInSOL = balanceInLamports / LAMPORTS_PER_SOL;

  console.log(
    `💰 Finished! The balance for the wallet at address ${publicKey} is ${balanceInSOL}!`
  );
} catch (error) {
  console.error(
    `Error fetching balance for wallet ${publicKey}:`,
    error.message
  );
}
