import { Keypair } from "@solana/web3.js";
import fs from "fs"
// Generate a new keypair
let kp = Keypair.generate()
console.log(`You've generated a new Solana wallet: ${kp.publicKey.toBase58()}`)
console.log(`[${kp.secretKey}]`)
// Save to file
fs.writeFileSync("../dev-wallet.json", JSON.stringify(Array.from(kp.secretKey)));