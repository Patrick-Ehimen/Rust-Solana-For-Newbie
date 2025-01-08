import { Connection, clusterApiUrl } from "@solana/web3.js";

// create a connection to the devnet cluster
const connection = new Connection(clusterApiUrl("devnet"));

// print a success message to the console, to let the user know that the connection was successful
console.log(`✅ Connected!`);
