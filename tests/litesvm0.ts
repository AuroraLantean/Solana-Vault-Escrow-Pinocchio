/** biome-ignore-all lint/style/noNonNullAssertion: <> */
import { expect, test } from "bun:test";
//Tutorial: <https://litesvm.github.io/litesvm/tutorial.html>
import type { Keypair } from "@solana/web3.js";
import {
	acctIsNull,
	depositSol,
	initSolBalc,
	sendSol,
	svm,
	vault1,
	vaultAta1,
	vaultO,
	withdrawSol,
} from "./litesvm-utils";
import { as9zBn, ll } from "./utils";
import {
	admin,
	adminKp,
	hackerKp,
	ownerKp,
	user1,
	user1Kp,
} from "./web3jsSetup";

let signerKp: Keypair;
let amount: bigint;
let amtDeposit: bigint;
let amtWithdraw: bigint;
let balcAf: bigint | null;
const vaultRent = 1002240n; //from Rust
//const initUsdcBalc = bigintAmt(1000, 6);

const balcBf = svm.getBalance(admin);
ll("admin SOL:", balcBf);
expect(balcBf).toStrictEqual(initSolBalc);

test("initial conditions", () => {
	acctIsNull(vaultAta1);
});
test("transfer SOL", () => {
	amount = as9zBn(0.001);
	sendSol(user1, amount, adminKp);
	balcAf = svm.getBalance(user1);
	expect(balcAf).toStrictEqual(amount + initSolBalc);
});

test("Owner Deposits SOL to VaultPDA", () => {
	ll("\n------== Owner Deposits SOL to VaultPDA");
	ll("vaultO:", vaultO.toBase58());
	signerKp = ownerKp;
	amtDeposit = as9zBn(0.46);

	depositSol(vaultO, amtDeposit, signerKp);
	balcAf = svm.getBalance(vaultO);
	ll("vaultO SOL:", balcAf);
	expect(balcAf).toStrictEqual(vaultRent + amtDeposit);
});

test("User1 Deposits SOL to vault1", () => {
	ll("\n------== User1 Deposits SOL to vault1");
	ll("vault1:", vault1.toBase58());
	signerKp = user1Kp;
	amtDeposit = as9zBn(1.23); //1230000000n

	depositSol(vault1, amtDeposit, signerKp);
	balcAf = svm.getBalance(vault1);
	ll("vault1 SOL:", balcAf);
	expect(balcAf).toStrictEqual(vaultRent + amtDeposit);
});

test("User1 Withdraws SOL from vault1", () => {
	ll("\n------== User1 Withdraws SOL from vault1");
	ll("vault1:", vault1.toBase58());
	signerKp = user1Kp;
	amtWithdraw = as9zBn(0.48); //480000000n

	withdrawSol(vault1, amtWithdraw, signerKp);
	balcAf = svm.getBalance(vault1);
	ll("vault1 SOL:", balcAf);
	expect(balcAf).toStrictEqual(vaultRent + amtDeposit - amtWithdraw);
});
//test.failing
test("hacker cannot withdraw SOL from  vault1", () => {
	ll("\n------== Hacker cannot withdraw SOL from vault1");
	signerKp = hackerKp;
	amtWithdraw = as9zBn(0.48); //480000000n
	withdrawSol(vault1, amtWithdraw, signerKp, "0x35");
});
