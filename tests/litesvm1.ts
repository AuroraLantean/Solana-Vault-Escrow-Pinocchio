/** biome-ignore-all lint/style/noNonNullAssertion: <> */
import { expect, test } from "bun:test";

//Tutorial: <https://litesvm.github.io/litesvm/tutorial.html>
import {
	//	ACCOUNT_SIZE,	TOKEN_PROGRAM_ID,
	AccountLayout,
	getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import {
	Keypair,
	LAMPORTS_PER_SOL,
	SystemProgram,
	Transaction,
	TransactionInstruction,
} from "@solana/web3.js";
import {
	checkSuccess,
	findPdaV1,
	helloworldProgram,
	ll,
	makeMint,
	systemProgram,
	usdcMint,
	vaultProgram,
} from "./litesvm-utils";
import { as9zBn, bigintToBytes, bytesToBigint } from "./utils";
import {
	adminAddr,
	adminKp,
	initBalc,
	ownerAddr,
	svm,
	user1Addr,
	user1Kp,
} from "./web3jsSetup";

const adminBalc = svm.getBalance(adminAddr);
ll("admin SOL:", adminBalc);
expect(adminBalc).toStrictEqual(initBalc);

const vaultPdaBump = findPdaV1(ownerAddr, "vault", "VaultPDA");
const vaultPdaBump1 = findPdaV1(user1Addr, "vault", "VaultPDA1");
const _vaultPDA = vaultPdaBump.pda;
const vaultPDA1 = vaultPdaBump1.pda;

test("transfer SOL", () => {
	const blockhash = svm.latestBlockhash();
	const transferLamports = 1_000_000n;
	const ixs = [
		SystemProgram.transfer({
			fromPubkey: adminKp.publicKey,
			toPubkey: user1Addr,
			lamports: transferLamports,
		}),
	];
	const tx = new Transaction();
	tx.recentBlockhash = blockhash;
	tx.add(...ixs);
	tx.sign(adminKp);
	svm.sendTransaction(tx);
	const balanceAfter = svm.getBalance(user1Addr);
	expect(balanceAfter).toStrictEqual(transferLamports + initBalc);
});

test("hello world", () => {
	const [programId, greetedPubkey] = helloworldProgram(svm);

	const payer = new Keypair();
	const amtInit = BigInt(LAMPORTS_PER_SOL);
	svm.airdrop(payer.publicKey, amtInit);
	const amt = svm.getBalance(greetedPubkey);
	ll("payer SOL balc:", amt);
	expect(amt).toStrictEqual(amtInit);

	const blockhash = svm.latestBlockhash();

	const greetedAccountBefore = svm.getAccount(greetedPubkey);
	expect(greetedAccountBefore).not.toBeNull();
	expect(greetedAccountBefore?.data).toStrictEqual(
		new Uint8Array([0, 0, 0, 0]),
	);

	const ix = new TransactionInstruction({
		keys: [{ pubkey: greetedPubkey, isSigner: false, isWritable: true }],
		programId,
		data: Buffer.from([0]),
	});
	const tx = new Transaction();
	tx.recentBlockhash = blockhash;
	tx.add(ix); //tx.add(...ixs);
	tx.sign(payer);
	svm.sendTransaction(tx);

	const greetedAccountAfter = svm.getAccount(greetedPubkey);
	expect(greetedAccountAfter).not.toBeNull();
	expect(greetedAccountAfter?.data).toStrictEqual(new Uint8Array([1, 0, 0, 0]));
});

test("User1 Deposits SOL to vault1", () => {
	ll("\n------== User1 Deposits SOL to vault1");
	const disc = 0; //discriminator
	ll("vaultPDA1:", vaultPDA1.toBase58());
	const payer = user1Kp;
	const amtlam = as9zBn(1.23);
	//ll(toLam(amtSol));1230000000n

	const [programId] = vaultProgram(svm);
	ll("programId:", programId.toBase58());

	const argData = bigintToBytes(amtlam);
	//const bytes = [disc, ...argData];
	//ll("bytes:", bytes);

	const blockhash = svm.latestBlockhash();
	const ix = new TransactionInstruction({
		keys: [
			{ pubkey: payer.publicKey, isSigner: true, isWritable: true },
			{ pubkey: vaultPDA1, isSigner: false, isWritable: true },
			{ pubkey: systemProgram, isSigner: false, isWritable: false },
		],
		programId,
		data: Buffer.from([disc, ...argData]),
	});
	const tx = new Transaction();
	tx.recentBlockhash = blockhash;
	tx.add(ix); //tx.add(...ixs);
	tx.sign(payer);

	const simRes = svm.simulateTransaction(tx);
	ll("simRes meta logs:", simRes.meta().logs());
	//ll("simRes meta prettylogs:", simRes.meta().prettyLogs());
	ll("simRes meta returnData:", simRes.meta().returnData().toString()); //simRes.err(),
	/** simRes.meta():
  computeUnitsConsumed: [class computeUnitsConsumed],
  innerInstructions: [class innerInstructions],
  logs: [class logs],
  prettyLogs: [class prettyLogs],
  returnData: [class returnData],
  signature: [class signature],
  toString: [class toString], */

	const sendRes = svm.sendTransaction(tx);
	ll("\nsendRes:", sendRes.toString()); //sendRes.err(),sendRes.meta()
	//ll("sendRes:", sendRes);
	//ll("sendRes.logs():", sendRes.logs());

	checkSuccess(simRes, sendRes, programId, 15);
	ll("after simulation");

	const lamports2a = svm.getBalance(vaultPDA1);
	ll("lamports2a:", lamports2a);
	//expect(amtAdmin).toStrictEqual(initBalc);
});

test("inputNum to/from Bytes", () => {
	ll("\n------== inputNum to/from Bytes");
	const amountNum = as9zBn(1.23);
	const argData64 = bigintToBytes(amountNum);
	const _amtOut64 = bytesToBigint(argData64);

	const time1 = 1766946349;
	const argData32 = bigintToBytes(time1, 32);
	const _amtOut32 = bytesToBigint(argData32);

	const u8Num = 37;
	const argDataU8 = bigintToBytes(u8Num, 8);
	const _amtOut8 = bytesToBigint(argDataU8);
});

test("infinite usdc mint", () => {
	const adminUsdcAta = getAssociatedTokenAddressSync(usdcMint, adminAddr, true);
	const amt = 1_000_000_000_000n;
	const rawAccount = makeMint(svm, usdcMint, adminAddr, adminUsdcAta, amt);

	expect(rawAccount).not.toBeNull();
	const rawAccountData = rawAccount?.data;
	const decoded = AccountLayout.decode(rawAccountData!);
	expect(decoded.amount).toStrictEqual(amt);
});

/*Test with Time Travel: https://litesvm.github.io/litesvm/tutorial.html#time-travel
const c = svm.getClock();
svm.setClock(
  new Clock(c.slot, c.epochStartTimestamp, c.epoch, c.leaderScheduleEpoch, BigInt(quarterTime))    );

Test with arbitrary accounts
https://litesvm.github.io/litesvm/tutorial.html#time-travel      

Copying Accounts from a live environment 
https://litesvm.github.io/litesvm/tutorial.html#copying-accounts-from-a-live-environment
*/

ll("LiteSVM finished");
