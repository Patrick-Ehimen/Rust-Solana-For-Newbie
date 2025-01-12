import {
  Connection,
  Transaction,
  SystemProgram,
  sendAndConfirmTransaction,
  PublicKey,
} from "@solana/web3.js";
import "dotenv/config";
import { getKeypairFromEnvironment } from "@solana-developers/helpers";

// Get the public key of the recipient from the command line arguments
const suppliedToPubkey = process.argv[2] || null;

// If no public key is provided, print an error message and exit
if (!suppliedToPubkey) {
  console.log(`Please provide a public key to send to`);
  process.exit(1);
}

// Load the keypair of the sender from the environment variable
const senderKeypair = getKeypairFromEnvironment("SECRET_KEY_TWO");

console.log(`suppliedToPubkey: ${suppliedToPubkey}`);

// Create a new PublicKey object from the recipient's public key
const toPubkey = new PublicKey(suppliedToPubkey);

// Connect to the Solana devnet cluster
const connection = new Connection("https://api.devnet.solana.com", "confirmed");

console.log(
  `✅ Loaded our own keypair, the destination public key, and connected to Solana`
);

// Create a new Transaction object
const transaction = new Transaction();

// Set the amount of SOL to send in lamports
const LAMPORTS_TO_SEND = 300000;

// Create a transfer instruction to send the SOL
const sendSolInstruction = SystemProgram.transfer({
  fromPubkey: senderKeypair.publicKey,
  toPubkey,
  lamports: LAMPORTS_TO_SEND,
});

// Add the instruction to the transaction
transaction.add(sendSolInstruction);

// Sign and send the transaction
const signature = await sendAndConfirmTransaction(connection, transaction, [
  senderKeypair,
]);

console.log(
  `💸 Finished! Sent ${LAMPORTS_TO_SEND} to the address ${toPubkey}. `
);
console.log(`Transaction signature is ${signature}!`);
