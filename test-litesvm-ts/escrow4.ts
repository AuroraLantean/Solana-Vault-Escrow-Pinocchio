/** biome-ignore-all lint/style/noNonNullAssertion: <> */
import { expect, test } from "bun:test";
import type { Keypair, PublicKey } from "@solana/web3.js";
import type { AccountInfoBytes } from "litesvm";
import {
	Status,
	solanaKitDecodeConfigDev,
	solanaKitDecodeEscrowDev,
} from "./decoder";
import {
	acctExists,
	acctIsNull,
	ataBalCk,
	ataBalc,
	cancelTokEscrow,
	configBump,
	configPDA,
	findEscrow,
	getAta,
	initConfig,
	initSolBalc,
	lgcInitAta,
	lgcInitMint,
	lgcMintToken,
	makeTokEscrow,
	type PdaOut,
	setAtaCheck,
	setMint,
	svm,
	takeTokEscrow,
	vault1,
	vaultO,
	withdrawTokEscrow,
} from "./litesvm-utils";
import { bigintAmt, ll, zero } from "./utils";
import {
	admin,
	adminKp,
	dgcAuthorityKp,
	dragonCoin,
	dragonCoinKp,
	owner,
	ownerKp,
	pyusdMint,
	usdcMint,
	usdgMint,
	usdtMint,
	user1,
	user1Kp,
	user2,
	user2Kp,
	vaultProgAddr,
} from "./web3jsSetup";

let signerKp: Keypair;
let mints: PublicKey[];
let signer: PublicKey;
let progOwner: PublicKey;
let progAdmin: PublicKey;
let fee: bigint;
let prevBalcX: bigint;
let isAuthorized = false;
let status: Status;
let str: string;
let mintKp: Keypair;
let mintAuthorityKp: Keypair;
let mint: PublicKey;
let mintAuthority: PublicKey;
let user: PublicKey;
let ata: PublicKey;

let escrowOut: PdaOut;
let escrowPDA: PublicKey;
let mintX: PublicKey;
let mintY: PublicKey;
let makerAtaX: PublicKey;
let makerAtaY: PublicKey;
let takerAtaX: PublicKey;
let takerAtaY: PublicKey;
let escrowAtaX: PublicKey;
let escrowAtaY: PublicKey;
let escrowU1_1: PublicKey;
let _escrowU2_2: PublicKey;
let rawAccount: AccountInfoBytes | null;
let _amtvBalcX: bigint;
let prevBalcY: bigint;
let decimalX: number;
let decimalY: number;
let amountX: bigint;
let amountY: bigint;
let id: bigint;
const decDgc = 9;

let decimals: number;
let amt: bigint;
const initDgcBalc = bigintAmt(9000, decDgc);
const _initUsdcBalc = bigintAmt(1000, 6);

const adminBalc = svm.getBalance(admin);
ll("admin SOL:", adminBalc);
expect(adminBalc).toStrictEqual(initSolBalc);

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
		mints,
		progOwner,
		progAdmin,
		isAuthorized,
		status,
		fee,
		str,
		signerKp,
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

test("Set USDT Mint and ATAs", () => {
	ll("\n------== Set USDT Mint and ATAs");
	setMint(usdcMint);
	acctExists(usdcMint);
	setMint(usdtMint);
	acctExists(usdtMint);
	setMint(pyusdMint);
	acctExists(pyusdMint);
	setMint(usdgMint);
	acctExists(usdgMint);
	const initUsdcBalc = bigintAmt(1000, 6);
	setAtaCheck(usdcMint, user1, initUsdcBalc, "User1 USDC");
});

test("Make DragonCoin Mint, ATA, Tokens", () => {
	ll("\n------== Make DragonCoin Mint, ATA, Tokens");
	signerKp = adminKp;
	mintKp = dragonCoinKp;
	mintAuthorityKp = dgcAuthorityKp;
	decimals = decDgc;
	amt = initDgcBalc;

	signer = signerKp.publicKey;
	mint = mintKp.publicKey;
	mintAuthority = mintAuthorityKp.publicKey;
	ll("signer", signerKp.publicKey.toBase58());
	ll("mint", mint.toBase58());
	//TODO: Codama to defined optional account
	acctIsNull(mint);
	lgcInitMint(signerKp, mintKp, mintAuthority, mintAuthority, decimals);
	acctExists(mint);

	user = admin;
	ata = getAta(mint, user);
	lgcInitAta(signerKp, user, mint, ata);
	acctExists(ata);
	lgcMintToken(mintAuthorityKp, user, mint, ata, decimals, amt);
	ataBalCk(ata, amt, "admin", decDgc);
	ll("can mint to admin with ATA");

	user = user2;
	ata = getAta(mint, user);
	acctIsNull(ata);
	lgcMintToken(mintAuthorityKp, user, mint, ata, decimals, amt);
	ataBalCk(ata, amt, "user2", decDgc);
	ll("can mint to user2 without ATA");
});

test("Make Token Escrow", () => {
	ll("\n------== Make Token Escrow");
	signerKp = user1Kp;
	mintX = usdcMint;
	mintY = dragonCoin;
	decimalX = 6;
	decimalY = decDgc;
	amountX = bigintAmt(326, decimalX);
	amountY = bigintAmt(2100, decimalY);
	id = BigInt(1);
	signer = signerKp.publicKey;
	escrowOut = findEscrow(signer, id);
	escrowU1_1 = escrowOut.pda;
	escrowPDA = escrowU1_1;

	escrowAtaX = getAta(mintX, escrowPDA);
	makerAtaX = getAta(mintX, signer);
	prevBalcX = ataBalc(makerAtaX, "makerAtaX");
	makeTokEscrow(
		signerKp,
		makerAtaX,
		escrowAtaX,
		mintX,
		mintY,
		escrowPDA,
		configPDA,
		decimalX,
		amountX,
		decimalY,
		amountY,
		id,
	);
	const pdaRaw = svm.getAccount(escrowPDA);
	expect(pdaRaw).not.toBeNull();
	const rawAccountData = pdaRaw?.data;
	ll("rawAccountData:", rawAccountData);

	const decoded = solanaKitDecodeEscrowDev(rawAccountData);
	expect(decoded.maker).toEqual(signer);
	expect(decoded.mintX).toEqual(mintX);
	expect(decoded.mintY).toEqual(mintY);
	expect(decoded.amountY).toEqual(amountY);
	expect(decoded.amountX).toEqual(amountX);
	expect(decoded.id).toEqual(id);
	expect(decoded.decimalX).toEqual(decimalX);
	expect(decoded.decimalY).toEqual(decimalY);
	expect(decoded.bump).toEqual(escrowOut.bump);
	ataBalCk(escrowAtaX, amountX, "Escrow");
	ataBalCk(makerAtaX, prevBalcX - amountX, "user1 ");
});
test("Take Token Escrow", () => {
	ll("\n------== Take Token Escrow");
	signerKp = user2Kp;
	//args below should be taken from EscrowPDA
	mintX = usdcMint;
	mintY = dragonCoin;
	decimalX = 6;
	decimalY = decDgc;
	amountX = bigintAmt(326, decimalX);
	amountY = bigintAmt(2100, decimalY);
	id = BigInt(1);
	escrowPDA = escrowU1_1;

	signer = signerKp.publicKey;
	takerAtaX = getAta(mintX, signer);
	escrowAtaX = getAta(mintX, escrowPDA);

	takerAtaY = getAta(mintY, signer);
	escrowAtaY = getAta(mintY, escrowPDA);
	prevBalcX = ataBalc(takerAtaX, "takerAtaX");
	takeTokEscrow(
		signerKp,
		takerAtaX,
		takerAtaY,
		escrowAtaX,
		escrowAtaY,
		mintX,
		mintY,
		escrowPDA,
		configPDA,
		decimalX,
		amountX,
		decimalY,
		amountY,
		id,
	);
	const pdaRaw = svm.getAccount(escrowPDA);
	expect(pdaRaw).not.toBeNull();
	const rawAccountData = pdaRaw?.data;
	ll("rawAccountData:", rawAccountData);

	const _decoded = solanaKitDecodeEscrowDev(rawAccountData);
	ataBalCk(escrowAtaX, zero, "Escrow X");
	ataBalCk(escrowAtaY, amountY, "Escrow Y");
	ataBalCk(takerAtaX, prevBalcX + amountX, "Taker X");
});
test("Withdraw TokenY on Escrow", () => {
	ll("\n------== Withdraw TokenY on Escrow");
	signerKp = user1Kp;
	//args below should be taken from EscrowPDA
	mintX = usdcMint;
	mintY = dragonCoin;
	escrowPDA = escrowU1_1;
	//amountY = amountY... also from EscrowPDA

	signer = signerKp.publicKey;
	makerAtaX = getAta(mintX, signer);
	escrowAtaX = getAta(mintX, escrowPDA);
	makerAtaY = getAta(mintY, signer);
	escrowAtaY = getAta(mintY, escrowPDA);
	prevBalcX = ataBalc(makerAtaX, "makerAtaX");
	prevBalcY = ataBalc(makerAtaY, "makerAtaY");

	withdrawTokEscrow(
		signerKp,
		makerAtaX,
		makerAtaY,
		escrowAtaX,
		escrowAtaY,
		mintX,
		mintY,
		escrowPDA,
		configPDA,
	);
	ataBalCk(escrowAtaX, zero, "Escrow X");
	ataBalCk(escrowAtaY, zero, "Escrow Y");
	ataBalCk(makerAtaX, prevBalcX, "user1 X");
	ataBalCk(makerAtaY, prevBalcY + amountY, "user1 Y");
	rawAccount = svm.getAccount(escrowAtaX);
	expect(rawAccount).toBeNull();
	rawAccount = svm.getAccount(escrowAtaY);
	expect(rawAccount).toBeNull();
	rawAccount = svm.getAccount(escrowPDA);
	expect(rawAccount).toBeNull();
});

test("Make & Cancel Token Escrow", () => {
	ll("\n------== Make & Cancel Token Escrow");
	signerKp = user1Kp;
	mintX = usdcMint;
	mintY = dragonCoin;
	decimalX = 6;
	decimalY = decDgc;
	amountX = bigintAmt(135, decimalX);
	amountY = bigintAmt(2700, decimalY);
	id = BigInt(1);
	signer = signerKp.publicKey;
	escrowOut = findEscrow(signer, id);
	escrowU1_1 = escrowOut.pda;
	escrowPDA = escrowU1_1;

	escrowAtaX = getAta(mintX, escrowPDA);
	makerAtaX = getAta(mintX, signer);
	prevBalcX = ataBalc(makerAtaX, "makerAtaX");
	makeTokEscrow(
		signerKp,
		makerAtaX,
		escrowAtaX,
		mintX,
		mintY,
		escrowPDA,
		configPDA,
		decimalX,
		amountX,
		decimalY,
		amountY,
		id,
	);
	ataBalCk(escrowAtaX, amountX, "Escrow");
	ataBalCk(makerAtaX, prevBalcX - amountX, "user1 ");

	cancelTokEscrow(
		signerKp,
		makerAtaX,
		makerAtaY,
		escrowAtaX,
		escrowAtaY,
		mintX,
		mintY,
		escrowPDA,
		configPDA,
	);
	ataBalCk(escrowAtaX, zero, "Escrow");
	ataBalCk(makerAtaX, prevBalcX, "user1 ");
	rawAccount = svm.getAccount(escrowAtaX);
	expect(rawAccount).toBeNull();
	rawAccount = svm.getAccount(escrowAtaY);
	expect(rawAccount).toBeNull();
	rawAccount = svm.getAccount(escrowPDA);
	expect(rawAccount).toBeNull();
});
