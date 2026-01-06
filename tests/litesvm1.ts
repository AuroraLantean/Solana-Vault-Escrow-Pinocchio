/** biome-ignore-all lint/style/noNonNullAssertion: <> */
import { expect, test } from "bun:test";

//Tutorial: <https://litesvm.github.io/litesvm/tutorial.html>
import {
	//	ACCOUNT_SIZE,	TOKEN_PROGRAM_ID,
	AccountLayout,
} from "@solana/spl-token";
import {
	Connection,
	type Keypair,
	SystemProgram,
	Transaction,
} from "@solana/web3.js";
import {
	depositSol,
	initBalc,
	makeMint,
	svm,
	vaultPDA,
	vaultPDA1,
} from "./litesvm-utils";
import { as9zBn, bigintToBytes, bytesToBigint, ll } from "./utils";
import {
	adminAddr,
	adminKp,
	ownerKp,
	usdcMint,
	user1Addr,
	user1Kp,
} from "./web3jsSetup";

//let disc = 0; //discriminator
let payerKp: Keypair;
let amount: bigint;
let amt: bigint;
let argData: Uint8Array<ArrayBufferLike>;
let blockhash: string;
let tx: Transaction;

const adminBalc = svm.getBalance(adminAddr);
ll("admin SOL:", adminBalc);
expect(adminBalc).toStrictEqual(initBalc);

test("transfer SOL", () => {
	blockhash = svm.latestBlockhash();
	amount = as9zBn(0.001);
	const ixs = [
		SystemProgram.transfer({
			fromPubkey: adminKp.publicKey,
			toPubkey: user1Addr,
			lamports: amount,
		}),
	];
	tx = new Transaction();
	tx.recentBlockhash = blockhash;
	tx.add(...ixs);
	tx.sign(adminKp);
	svm.sendTransaction(tx);
	const balanceAfter = svm.getBalance(user1Addr);
	expect(balanceAfter).toStrictEqual(amount + initBalc);
});

test("Owner Deposits SOL to VaultPDA", () => {
	ll("\n------== Owner Deposits SOL to VaultPDA");
	ll("vaultPDA:", vaultPDA.toBase58());
	payerKp = ownerKp;
	amount = as9zBn(0.46);
	argData = bigintToBytes(amount);
	depositSol(svm, vaultPDA, argData, payerKp);
});

test("User1 Deposits SOL to vault1", () => {
	ll("\n------== User1 Deposits SOL to vault1");
	ll("vaultPDA1:", vaultPDA1.toBase58());
	payerKp = user1Kp;
	amount = as9zBn(1.23); //1230000000n
	argData = bigintToBytes(amount);
	depositSol(svm, vaultPDA1, argData, payerKp);
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

test("mint usdc(set arbitrary account data)", () => {
	amt = 1_000_000_000_000n;
	const { rawAccount, ata: _adminUsdcAta } = makeMint(
		svm,
		usdcMint,
		adminAddr,
		amt,
	);
	expect(rawAccount).not.toBeNull();
	const rawAccountData = rawAccount?.data;
	const decoded = AccountLayout.decode(rawAccountData!);
	expect(decoded.amount).toStrictEqual(amt);
});

test.skip("copy accounts from devnet", async () => {
	const connection = new Connection("https://api.devnet.solana.com");
	const accountInfo = await connection.getAccountInfo(usdcMint);
	// the rent epoch goes above 2**53 which breaks web3.js, so just set it to 0;
	if (!accountInfo) throw new Error("accountInfo is null");
	accountInfo.rentEpoch = 0;
	svm.setAccount(usdcMint, accountInfo);
	const rawAccount = svm.getAccount(usdcMint);
	expect(rawAccount).not.toBeNull();
});
