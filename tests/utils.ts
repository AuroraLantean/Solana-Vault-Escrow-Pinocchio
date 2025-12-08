import assert from "node:assert";
import {
	appendTransactionMessageInstruction,
	assertIsTransactionWithBlockhashLifetime,
	createTransactionMessage,
	getSignatureFromTransaction,
	pipe,
	sendAndConfirmTransactionFactory,
	setTransactionMessageFeePayer,
	setTransactionMessageLifetimeUsingBlockhash,
	signTransactionMessageWithSigners,
} from "@solana/kit";
export const ll = console.log;

//https://www.solanakit.com/docs/getting-started/send-transaction#confirmation-strategies
export const sendTxn = async (
	methodIx: any,
	signerKp: any,
	rpc: any,
	rpcSubscriptions: any,
	shouldSucceed = true,
	isVerbose = false,
) => {
	ll("sendTxn() ...");
	const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();
	if (isVerbose) ll("latestBlockhash:", latestBlockhash);
	const txnMesg = pipe(
		createTransactionMessage({ version: 0 }),
		(tx) => setTransactionMessageFeePayer(signerKp.address, tx),
		(tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
		(tx) => appendTransactionMessageInstruction(methodIx, tx),
	);
	// Sign and send transaction
	const signedTransaction = await signTransactionMessageWithSigners(txnMesg);
	assertIsTransactionWithBlockhashLifetime(signedTransaction);

	const sendAndConfirmTransaction = sendAndConfirmTransactionFactory({
		rpc,
		rpcSubscriptions,
	});

	//lastValidBlockHeight
	if (shouldSucceed) {
		await sendAndConfirmTransaction(signedTransaction, {
			commitment: "confirmed",
		}); //"confirmRecentTransaction" | "rpc" | "transaction"

		const signature = getSignatureFromTransaction(signedTransaction);
		ll("Transaction signature:", signature);
	} else {
		await assert.rejects(
			sendAndConfirmTransaction(signedTransaction, {
				commitment: "confirmed",
			}),
			{
				message: "Transaction simulation failed",
			},
		);
	}
};
