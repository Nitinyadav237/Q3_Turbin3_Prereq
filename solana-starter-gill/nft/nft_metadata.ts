import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
import fs, { ReadStream } from 'fs';
import wallet from "../wallet.json" with {type: "json"}

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

// umi.use(irysUploader({ address: "https://devnet.irys.xyz/", }));
umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        const imagePath = " https://gateway.irys.xyz/CU9P47qnbZSpiEMwzmz8B6uhwG7XkTYEBS4x2RjizX9"
        const metadata = {
            name: "Turbin3 Rock 237",
            symbol: "NY237",
            description: "Turbin Everywhere",
            image: imagePath,
            attributes: [
                { trait_type: 'generated', value: 'with excitement!' }
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: imagePath,
                    },
                ],
                category: "image"
            },
            creators: [{
                address: signer.publicKey.toString(),
                share: 100
            }]
        };
        console.log(imagePath,metadata,"imafsa")
        const myUri = await umi.uploader.uploadJson(metadata);
        console.log("Your metadata URI: ", myUri);
    }
    catch (error) {
        console.log("Oops.. Something went wrong", error);
    }
})();