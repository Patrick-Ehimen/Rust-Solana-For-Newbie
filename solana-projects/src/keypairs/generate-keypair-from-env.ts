import "dotenv/config";
import { getKeypairFromEnvironment } from "@solana-developers/helpers";

// Use the `getKeypairFromEnvironment` function from the `@solana-developers/helpers` package to load our secret key securely from an environment variable.
const keypair = getKeypairFromEnvironment("SECRET_KEY");

// Print a success message to the console, to let the user know that the script has finished successfully.
console.log(
  `✅ Finished! We've loaded our secret key securely, using an env file!`
);
