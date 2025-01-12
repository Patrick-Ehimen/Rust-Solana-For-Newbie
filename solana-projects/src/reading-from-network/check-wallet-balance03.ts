import {
  Connection,
  LAMPORTS_PER_SOL,
  PublicKey,
  clusterApiUrl,
} from "@solana/web3.js";
import { getDomainKeySync, NameRegistryState } from "@bonfida/spl-name-service";

async function checkWalletBlance(publicKeyString: String) {
  // The public key should be 44 characters long, which is the length of a
  // base58 encoded public key.
  if (publicKeyString.length !== 44 && !publicKeyString.endsWith(".sol")) {
    console.error(
      "Error: Wallet address must be exactly 44 characters long or a valid SNS domain."
    );
    return;
  }

  // Validate the public key
  try {
    const publicKey = new PublicKey(publicKeyString);
    const connection = new Connection(clusterApiUrl("mainnet-beta"));

    // const connection = new Connection(
    //   "https://api.devnet.solana.com",
    //   "confirmed"
    // );

    const balanceInLamports = await connection.getBalance(publicKey);
    const balanceInSOL = balanceInLamports / LAMPORTS_PER_SOL;

    console.log(
      `✅ Finished! The balance for the wallet at address ${publicKey} is ${balanceInSOL} SOL!`
    );
  } catch (error) {
    console.error("Invalid wallet address provided:", publicKeyString);
    return; // Exit the function early
  }
}

const suppliedPublicKey = process.argv[2];

if (!suppliedPublicKey) {
  throw new Error("Provide a valid public key to check the balance of!");
}

checkWalletBlance(suppliedPublicKey);
