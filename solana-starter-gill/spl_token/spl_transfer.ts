
import { address, createSolanaClient, createTransaction, getExplorerLink, getSignatureFromTransaction, signTransactionMessageWithSigners } from "gill";
import { loadKeypairSignerFromFile } from "gill/node";
import { getAssociatedTokenAccountAddress, getCreateAssociatedTokenIdempotentInstruction, getTransferInstruction } from "gill/programs"
import { getMintSize, TOKEN_PROGRAM_ADDRESS } from "gill/programs/token"

async function main() {
    const { rpc, sendAndConfirmTransaction } = createSolanaClient({ urlOrMoniker: "devnet" });
    const signer = await loadKeypairSignerFromFile("./wallet.json");

    const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();
    console.log("Latest blockhash:", latestBlockhash);


    const mint = address("2yAHTmxUGhcfw5PAMNzhLitkTdPEb15QTyyDeXPnMYY6");
    const ownerAddress = signer.address
    const destinationAddress = address("4XVnTC3qLZnN1MSDCQ7QXHLow9yeLsabGFSCzpQCs7Q2")

    const sourceAta = await getAssociatedTokenAccountAddress(mint, ownerAddress, TOKEN_PROGRAM_ADDRESS)
    const destinationAta = await getAssociatedTokenAccountAddress(mint, destinationAddress, TOKEN_PROGRAM_ADDRESS)

    const space = getMintSize()
    console.log("✅ Mint address:", mint, ownerAddress, sourceAta, space);

    const tx = createTransaction({
        feePayer: signer,
        version: "legacy",
        instructions: [
            getCreateAssociatedTokenIdempotentInstruction({
                mint,
                owner: destinationAddress,
                payer: signer,
                tokenProgram: TOKEN_PROGRAM_ADDRESS,
                ata: destinationAta,
            }),
            getTransferInstruction({
                source: sourceAta,
                authority: signer,
                destination: destinationAta,
                amount: 2 * 1_000_000_000,
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
