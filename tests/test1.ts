import assert from "node:assert";
import { before, describe, it } from "node:test";
//import { test, expect, mock } from "bun:test";
import {
	airdropFactory,
	appendTransactionMessageInstruction,
	assertIsTransactionWithBlockhashLifetime,
	createSolanaRpc,
	createSolanaRpcSubscriptions,
	createTransactionMessage,
	generateKeyPairSigner,
	getAddressEncoder,
	getProgramDerivedAddress,
	getSignatureFromTransaction,
	getUtf8Encoder,
	lamports,
	pipe,
	sendAndConfirmTransactionFactory,
	setTransactionMessageFeePayer,
	setTransactionMessageLifetimeUsingBlockhash,
	signTransactionMessageWithSigners,
} from "@solana/kit";
import { SYSTEM_PROGRAM_ADDRESS } from "@solana-program/system";
import * as vault from "../clients/js/src/generated/index";

const LAMPORTS_PER_SOL = BigInt(1_000_000_000);
const vaultProgAddr = vault.PINOCCHIO_VAULT_PROGRAM_ADDRESS;

//BunJs Tests: https://bun.com/docs/test/writing-tests
describe("Vault Program", () => {
	let rpc: any;
	let rpcSubscriptions: any;
	let signer: any;
	let vaultRent: bigint;
	let vaultPDA: any;

	const ACCOUNT_DISCRIMINATOR_SIZE = 8; // same as Anchor/Rust
	const U64_SIZE = 8; // u64 is 8 bytes
	const VAULT_SIZE = ACCOUNT_DISCRIMINATOR_SIZE + U64_SIZE; // 16
	const DEPOSIT_AMOUNT = BigInt(100000000);
	const ll = console.log;

	//https://bun.com/docs/test: beforeAll, beforeEach
	before(async () => {
		// Establish connection to Solana cluster
		const httpProvider = "http://127.0.0.1:8899";
		const wssProvider = "ws://127.0.0.1:8900";
		rpc = createSolanaRpc(httpProvider);
		rpcSubscriptions = createSolanaRpcSubscriptions(wssProvider);
		ll(`✅ - Established connection to ${httpProvider}`);

		const { value } = await rpc
			.getAccountInfo(vaultProgAddr, { encoding: "base64" })
			.send();
		if (!value || !value?.data) {
			throw new Error(
				`Program account does not exist: ${vaultProgAddr.toString()}`,
			);
		}
		ll("✅ - program exits!");
		/*const base64Encoder = getBase64Encoder();
    let bytes = base64Encoder.encode(value.data[0]);
    const decoded = ammConfigDecoder.decode(bytes);
    ll(decoded);*/

		//https://www.solanakit.com/docs/getting-started/signers
		// Generate signers
		signer = await generateKeyPairSigner();
		//import secret from './my-keypair.json';
		//const user2 = await createKeyPairSignerFromBytes(new Uint8Array(secret));
		const signerAddress = await signer.address;
		ll(`✅ - New signer address: ${signerAddress}`);

		// Airdrop SOL to signer
		const airdrop = airdropFactory({ rpc, rpcSubscriptions });
		await airdrop({
			commitment: "confirmed",
			lamports: lamports(LAMPORTS_PER_SOL),
			recipientAddress: signerAddress,
		});
		ll(`✅ - Airdropped SOL to Signer: ${signerAddress}`);

		// get vault rent
		vaultRent = await rpc.getMinimumBalanceForRentExemption(VAULT_SIZE).send();

		// Get vault PDA
		const seedSigner = getAddressEncoder().encode(await signer.address);
		const seedTag = getUtf8Encoder().encode("vault");

		ll("vaultProgAddr:", vaultProgAddr);
		vaultPDA = await getProgramDerivedAddress({
			programAddress: vaultProgAddr,
			seeds: [seedTag, seedSigner],
		});
		ll(`✅ - Vault PDA: ${vaultPDA[0]}`);
	});

	//------------------==
	it("can deposit to vault", async () => {
		ll("here deposit-01");
		//  Deposit transaction using generated client
		const depositIx = vault.getDepositInstruction(
			{
				owner: signer,
				vault: vaultPDA[0],
				program: vaultProgAddr,
				systemProgram: SYSTEM_PROGRAM_ADDRESS,
				amount: lamports(DEPOSIT_AMOUNT),
			},
			{
				programAddress: vaultProgAddr,
			},
		);

		ll("here deposit-02");
		const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

		ll("here deposit-03");
		const txnMesg = pipe(
			createTransactionMessage({ version: 0 }),
			(tx) => setTransactionMessageFeePayer(signer.address, tx),
			(tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
			(tx) => appendTransactionMessageInstruction(depositIx, tx),
		);

		ll("here deposit-04");
		//https://www.solanakit.com/docs/getting-started/send-transaction#confirmation-strategies
		// Sign and send transaction
		const signedTransaction = await signTransactionMessageWithSigners(txnMesg);

		assertIsTransactionWithBlockhashLifetime(signedTransaction);

		const sendAndConfirmTransaction = sendAndConfirmTransactionFactory({
			rpc,
			rpcSubscriptions,
		});
		//lastValidBlockHeight
		ll("here deposit-05");
		await sendAndConfirmTransaction(signedTransaction, {
			commitment: "confirmed",
		}); //"confirmRecentTransaction" | "rpc" | "transaction"

		ll("here deposit-06");
		const signature = getSignatureFromTransaction(signedTransaction);
		ll("Transaction signature:", signature);

		ll("here deposit-07");
		const { value } = await rpc.getBalance(vaultPDA[0].toString()).send();
		assert.equal(DEPOSIT_AMOUNT, Number(value) - Number(vaultRent));
	}); //can deposit to vault
	/* BunJs
  test.serial("first test", ()=>{...})
  expect(true).toBe(true);
  expect(1 + 1).toBe(2);
  expect(sharedState).toBe(1);
 */
	//------------------==
	it("can withdraw from vault", async () => {
		const withdrawIx = vault.getWithdrawInstruction({
			owner: signer,
			vault: vaultPDA[0],
			program: vaultProgAddr,
		});

		const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();
		const txnMesg = pipe(
			createTransactionMessage({ version: 0 }),
			(tx) => setTransactionMessageFeePayer(signer.address, tx),
			(tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
			(tx) => appendTransactionMessageInstruction(withdrawIx, tx),
		);

		const signedTransaction = await signTransactionMessageWithSigners(txnMesg);
		assertIsTransactionWithBlockhashLifetime(signedTransaction);

		const sendAndConfirmTransaction = sendAndConfirmTransactionFactory({
			rpc,
			rpcSubscriptions,
		});

		await sendAndConfirmTransaction(signedTransaction, {
			commitment: "confirmed",
		});

		const signature = getSignatureFromTransaction(signedTransaction);
		ll("Transaction signature:", signature);

		const { value } = await rpc.getBalance(vaultPDA[0].toString()).send();
		assert.equal(Number(vaultRent), value);
	}); //can withdraw from vault

	//------------------==
	//test.failing("fail test",)_=>{...})
	it("doesn't allow other users to withdraw from the vault", async () => {
		// signer that DOES NOT own the vault
		const otherSigner = await generateKeyPairSigner();

		const withdrawIx = vault.getWithdrawInstruction({
			owner: otherSigner,
			vault: vaultPDA[0],
			program: vaultProgAddr,
		});

		const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();
		const tx = pipe(
			createTransactionMessage({ version: 0 }),
			(tx) => setTransactionMessageFeePayer(otherSigner.address, tx),
			(tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
			(tx) => appendTransactionMessageInstruction(withdrawIx, tx),
		);

		const signedTransaction = await signTransactionMessageWithSigners(tx);
		assertIsTransactionWithBlockhashLifetime(signedTransaction);

		const sendAndConfirmTransaction = sendAndConfirmTransactionFactory({
			rpc,
			rpcSubscriptions,
		});

		await assert.rejects(
			sendAndConfirmTransaction(signedTransaction, {
				commitment: "confirmed",
			}),
			{
				message: "Transaction simulation failed",
			},
		);
	});
});
//if error: Attempt to load a program that does not exist. You have to deploy the program first before running this test!
