/** biome-ignore-all lint/style/noNonNullAssertion: <> */
import { expect, test } from "bun:test";
import type { Keypair, PublicKey } from "@solana/web3.js";
import { Transaction, TransactionInstruction } from "@solana/web3.js";
import type {
	FailedTransactionMetadata,
	SimulatedTransactionInfo,
	TransactionMetadata,
} from "litesvm";

import {
	checkSuccess,
	configPDA,
	initBalc,
	svm,
	vaultPDA1,
} from "./litesvm-utils";
import { as9zBn, bigintToBytes, ll } from "./utils";
import {
	adminAddr,
	adminKp,
	systemProgram,
	vaultProgAddr,
} from "./web3jsSetup";

const adminBalc = svm.getBalance(adminAddr);
ll("admin SOL:", adminBalc);
expect(adminBalc).toStrictEqual(initBalc);

let disc = 0; //discriminator
let _payerKp: Keypair;
let authorityKp: Keypair;
let _authority: PublicKey;
let originalOwner: PublicKey;
let amount: bigint;
let _amt: bigint;
let argData: Uint8Array<ArrayBufferLike>;
let blockhash: string;
let ix: TransactionInstruction;
let tx: Transaction;
let simRes: FailedTransactionMetadata | SimulatedTransactionInfo;
let sendRes: FailedTransactionMetadata | TransactionMetadata;

test("InitConfig", () => {
	ll("\n------== InitConfig");
	disc = 12; //discriminator
	ll("vaultPDA1:", vaultPDA1.toBase58());
	ll(`configPDA: ${configPDA}`);
	authorityKp = adminKp;
	originalOwner = authorityKp.publicKey;
	amount = as9zBn(111);
	argData = bigintToBytes(amount);
	//const bytes = [disc, ...argData];
	//ll("bytes:", bytes);

	blockhash = svm.latestBlockhash();
	ix = new TransactionInstruction({
		keys: [
			{ pubkey: authorityKp.publicKey, isSigner: true, isWritable: true },
			{ pubkey: configPDA, isSigner: false, isWritable: true },
			{ pubkey: originalOwner, isSigner: false, isWritable: false },
			{ pubkey: systemProgram, isSigner: false, isWritable: false },
		],
		programId: vaultProgAddr,
		data: Buffer.from([disc, ...argData]),
	});
	tx = new Transaction();
	tx.recentBlockhash = blockhash;
	tx.add(ix); //tx.add(...ixs);
	tx.sign(authorityKp);

	simRes = svm.simulateTransaction(tx);
	sendRes = svm.sendTransaction(tx);
	checkSuccess(simRes, sendRes, vaultProgAddr);
});

test("inputNum to/from Bytes", () => {});

/*Test with Time Travel: https://litesvm.github.io/litesvm/tutorial.html#time-travel
const c = svm.getClock();
svm.setClock(
  new Clock(c.slot, c.epochStartTimestamp, c.epoch, c.leaderScheduleEpoch, BigInt(quarterTime))    );

Test with arbitrary accounts
https://litesvm.github.io/litesvm/tutorial.html#time-travel      

Copying Accounts from a live environment 
https://litesvm.github.io/litesvm/tutorial.html#copying-accounts-from-a-live-environment
*/

ll("LiteSVM3 finished");
