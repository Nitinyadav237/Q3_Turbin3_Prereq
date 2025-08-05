import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";

import base58 from "bs58";
import wallet from "../wallet.json" with {type: "json"}

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

const metadataUri="https://gateway.irys.xyz/E15Xq3NNFG5mrUjoPXUZjXg4iRj74wewV8FPiDsm1XfB"
let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata())

const mint = generateSigner(umi);

(async () => {
      let tx = createNft(umi, {
        mint,
        name: "Turbin3 Rock 237",
        symbol: "NY237",
        uri: metadataUri,
        sellerFeeBasisPoints: percentAmount(1),
        isMutable: true,
        collectionDetails: null
    });
    let result = await tx.sendAndConfirm(umi);
    const signature = base58.encode(result.signature);

    console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)

    console.log("Mint Address: ", mint.publicKey);
})();