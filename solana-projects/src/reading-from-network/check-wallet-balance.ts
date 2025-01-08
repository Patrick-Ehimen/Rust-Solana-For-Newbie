import {
  Connection,
  PublicKey,
  clusterApiUrl,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";

// create a connection to the devnet cluster
const connection = new Connection(clusterApiUrl("devnet"));

// specify the public key of the account we want to check
const address = new PublicKey("CenYq6bDRB7p73EjsPEpiYN7uveyPUTdXkDkgUduboaN");

// use the connection to fetch the balance of the account
const balance = await connection.getBalance(address);

// convert the balance from lamports to SOL
const balanceInSol = balance / LAMPORTS_PER_SOL;

// print the result
console.log(
  `The balance of the account at ${address} is ${balanceInSol.toFixed(4)} SOL`
);
console.log(`✅ Finished!`);

// the `toFixed(4)` method formats the number to always have four decimal places,
// so we don't get ugly long numbers in the output.
