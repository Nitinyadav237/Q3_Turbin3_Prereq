
import { createSolanaClient, createTransaction, generateKeyPairSigner, getExplorerLink, getMinimumBalanceForRentExemption, getSignatureFromTransaction, signTransactionMessageWithSigners } from "gill";
import { loadKeypairSignerFromFile } from "gill/node";
import { getCreateAccountInstruction, getInitializeMintInstruction } from "gill/programs"
import { getMintSize, TOKEN_PROGRAM_ADDRESS } from "gill/programs/token"

async function main() {
    const { rpc, sendAndConfirmTransaction } = createSolanaClient({ urlOrMoniker: "devnet" });
    const signer = await loadKeypairSignerFromFile("./wallet.json");

    const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();
    console.log("Latest blockhash:", latestBlockhash);


    const mint = await generateKeyPairSigner();
    console.log("✅ Mint address:", mint);

    const space = getMintSize()

    const tx = createTransaction({
        feePayer: signer,
        version: "legacy",
        instructions: [
            getCreateAccountInstruction({
                space,
                lamports: getMinimumBalanceForRentExemption(space),
                newAccount: mint,
                payer: signer,
                programAddress: TOKEN_PROGRAM_ADDRESS
            }),
            getInitializeMintInstruction({
                mint: mint.address,
                mintAuthority: signer.address,
                freezeAuthority: signer.address,
                decimals: 9
            }, {
                programAddress: TOKEN_PROGRAM_ADDRESS
            })
        ],
        latestBlockhash


    })

    const signedTx = await signTransactionMessageWithSigners(tx)
    console.log("signedtx", signedTx)

    console.log("Explorer:", getExplorerLink({
        cluster: "devnet",
        transaction: getSignatureFromTransaction(signedTx)
    }))
    await sendAndConfirmTransaction(signedTx)
}

main().catch(console.error);
