/** biome-ignore-all lint/style/noNonNullAssertion: <> */
import { expect, test } from "bun:test";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import type { Keypair, PublicKey } from "@solana/web3.js";
import { Status, solanaKitDecodeConfigDev } from "./decoder";
import {
	acctExists,
	configBump,
	configPDA,
	initConfig,
	initSolBalc,
	oraclesRead,
	setMint,
	svm,
	vault1,
	vaultO,
} from "./litesvm-utils";
import { ll } from "./utils";
import {
	admin,
	owner,
	ownerKp,
	pyusdMint,
	usdcMint,
	usdgMint,
	usdtMint,
	user1,
	user1Kp,
	vaultProgAddr,
} from "./web3jsSetup";

const adminBalc = svm.getBalance(admin);
ll("admin SOL:", adminBalc);
expect(adminBalc).toStrictEqual(initSolBalc);

let signerKp: Keypair;
let mints: PublicKey[];
let progOwner: PublicKey;
let progAdmin: PublicKey;
let oracleProg: PublicKey;
let tokenMint: PublicKey;
let tokenProg: PublicKey;
let oracleVendor = 0;
let numU32 = 0;
let numU64 = 0n;
let fee: bigint;
let isAuthorized = false;
let status: Status;
let str: string;

test("Set Mints", () => {
	ll("\n------== Set Mints");
	setMint(usdcMint);
	acctExists(usdcMint);
	setMint(usdtMint);
	acctExists(usdtMint);
	setMint(pyusdMint);
	acctExists(pyusdMint);
	setMint(usdgMint);
	acctExists(usdgMint);
});
test("InitConfig", () => {
	ll("\n------== InitConfig");
	ll("vault1:", vault1.toBase58());
	ll(`configPDA: ${configPDA}`);
	signerKp = ownerKp;
	mints = [usdcMint, usdtMint, pyusdMint, usdgMint];
	progOwner = owner;
	progAdmin = user1;
	fee = 111000000n;
	isAuthorized = true;
	status = Status.Active;
	str = "MoonDog to the Moon!";

	ll("progOwner:", progOwner.toBase58(), progOwner.toBytes());
	ll("progAdmin:", progAdmin.toBase58(), progAdmin.toBytes());
	initConfig(
		signerKp,
		mints,
		progOwner,
		progAdmin,
		isAuthorized,
		status,
		fee,
		str,
	);

	const pdaRaw = svm.getAccount(configPDA);
	expect(pdaRaw).not.toBeNull();
	const rawAccountData = pdaRaw?.data;
	ll("rawAccountData:", rawAccountData);
	expect(pdaRaw?.owner).toEqual(vaultProgAddr);

	const decoded = solanaKitDecodeConfigDev(rawAccountData);
	expect(decoded.mint0).toEqual(mints[0]!);
	expect(decoded.mint1).toEqual(mints[1]!);
	expect(decoded.mint2).toEqual(mints[2]!);
	expect(decoded.mint3).toEqual(mints[3]!);
	expect(decoded.vault).toEqual(vaultO);
	expect(decoded.progOwner).toEqual(progOwner);
	expect(decoded.admin).toEqual(progAdmin);
	expect(decoded.str).toEqual(str);
	expect(decoded.fee).toEqual(fee);
	expect(decoded.solBalance).toEqual(0n);
	expect(decoded.tokenBalance).toEqual(0n);
	ll("updatedAt:", decoded.updatedAt);
	expect(decoded.isAuthorized).toEqual(isAuthorized);
	expect(decoded.status).toEqual(status);
	expect(decoded.bump).toEqual(configBump);
});
test("OraclesRead", () => {
	ll("\n------== OraclesRead");
	ll("vault1:", vault1.toBase58());
	ll(`configPDA: ${configPDA}`);
	signerKp = user1Kp;
	oracleProg = usdcMint;
	tokenMint = usdcMint;
	tokenProg = TOKEN_PROGRAM_ID; //TOKEN_2022_PROGRAM_ID;
	oracleVendor = 0;
	numU32 = 12;
	numU64 = 1100n;

	ll("oracleProg:", oracleProg.toBase58());
	ll("tokenMint:", tokenMint.toBase58());
	ll("tokenProg:", tokenProg.toBase58());
	ll("oracleVendor:", oracleVendor);
	oraclesRead(
		signerKp,
		configPDA,
		oracleProg,
		tokenMint,
		tokenProg,
		oracleVendor,
		numU32,
		numU64,
	);
});
