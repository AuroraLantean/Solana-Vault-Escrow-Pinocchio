import { expect } from "bun:test";
import {
	ACCOUNT_SIZE,
	AccountLayout,
	TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import type { Keypair } from "@solana/web3.js";
import {
	LAMPORTS_PER_SOL,
	PublicKey,
	SystemProgram,
	Transaction,
} from "@solana/web3.js";
import type {
	FailedTransactionMetadata,
	SimulatedTransactionInfo,
} from "litesvm";
import { ComputeBudget, type LiteSVM, TransactionMetadata } from "litesvm";

export const vaultProgAddr = new PublicKey(
	"7EKqBVYSCmJbt2T8tGSmwzNKnpL29RqcJcyUr9aEEr6e",
);
export const systemProgram = new PublicKey("11111111111111111111111111111111");
export const usdcMint = new PublicKey(
	"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
);
export const usdtMint = new PublicKey(
	"Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB",
);
export const ll = console.log;
ll("vaultProgAddr:", vaultProgAddr.toBase58());

export function getRawAccount(svm: LiteSVM, address: PublicKey) {
	const rawAccount = svm.getAccount(address);
	return rawAccount;
}

export const findPdaV1 = (
	userAddr: PublicKey,
	seedStr: string,
	pdaName: string,
	progAddr = vaultProgAddr,
) => {
	const [pda, bump] = PublicKey.findProgramAddressSync(
		[Buffer.from(seedStr), userAddr.toBuffer()],
		progAddr,
	);
	ll(`${pdaName} pda: ${pda.toBase58()}, bump: ${bump}`);
	return { pda, bump };
};

export type ConfigT = {
	owner: PublicKey;
	deadline: number;
	deposit: bigint;
};
export const getConfigAcct = (
	programId: PublicKey,
	pdaName: string,
): PublicKey => {
	const [configPbk, _configBump] = PublicKey.findProgramAddressSync(
		[Buffer.from("proj_config")],
		programId,
	);
	ll(pdaName, ":", configPbk.toBase58());
	return configPbk;
};

export const makeAccount = (
	svm: LiteSVM,
	payer: Keypair,
	acctKp: Keypair,
	programId: PublicKey,
) => {
	const ixs = [
		SystemProgram.createAccount({
			fromPubkey: payer.publicKey,
			newAccountPubkey: acctKp.publicKey,
			lamports: Number(svm.minimumBalanceForRentExemption(BigInt(4))),
			space: 4,
			programId: programId,
		}),
	];
	const tx = new Transaction();
	const blockhash = svm.latestBlockhash();
	tx.recentBlockhash = blockhash;
	tx.add(...ixs);
	tx.sign(payer);
	svm.sendTransaction(tx);
};

//-------------==

//-------------== USDC or USDT
export const makeMint = (
	svm: LiteSVM,
	mint: PublicKey,
	owner: PublicKey,
	ata: PublicKey,
	tokenAmount: bigint,
) => {
	const tokenAccData = Buffer.alloc(ACCOUNT_SIZE);
	AccountLayout.encode(
		{
			mint,
			owner,
			//decimal: 6,
			amount: tokenAmount,
			delegateOption: 0,
			delegate: PublicKey.default,
			delegatedAmount: 0n,
			state: 1,
			isNativeOption: 0,
			isNative: 0n,
			closeAuthorityOption: 0,
			closeAuthority: PublicKey.default,
		},
		tokenAccData,
	);
	svm.setAccount(ata, {
		lamports: 1_000_000_000,
		data: tokenAccData,
		owner: TOKEN_PROGRAM_ID,
		executable: false,
	});
	const rawAccount = svm.getAccount(ata);
	return rawAccount;
};

//---------------== Deployment
export const vaultProgram = (
	svm: LiteSVM,
	computeMaxUnits?: bigint,
): [PublicKey] => {
	const programId = vaultProgAddr;

	if (computeMaxUnits) {
		const computeBudget = new ComputeBudget();
		computeBudget.computeUnitLimit = computeMaxUnits;
		svm = svm.withComputeBudget(computeBudget);
	}
	const programPath = "target/deploy/pinocchio_vault.so";
	svm.addProgramFromFile(programId, programPath);
	return [programId];
};

export function helloworldProgram(
	svm: LiteSVM,
	computeMaxUnits?: bigint,
): [PublicKey, PublicKey] {
	const programId = PublicKey.unique();
	const greetedPubkey = PublicKey.unique();
	//let svm = new LiteSVM();

	if (computeMaxUnits) {
		const computeBudget = new ComputeBudget();
		computeBudget.computeUnitLimit = computeMaxUnits;
		svm = svm.withComputeBudget(computeBudget);
	}
	svm.setAccount(greetedPubkey, {
		executable: false,
		owner: programId,
		lamports: LAMPORTS_PER_SOL,
		data: new Uint8Array([0, 0, 0, 0]),
	});
	const programPath = "program_bytes/counter.so";
	svm.addProgramFromFile(programId, programPath);
	return [programId, greetedPubkey];
}

export const checkSuccess = (
	simRes: FailedTransactionMetadata | SimulatedTransactionInfo,
	sendRes: TransactionMetadata | FailedTransactionMetadata,
	programId: PublicKey,
	logIndex: number,
) => {
	if (sendRes instanceof TransactionMetadata) {
		expect(simRes.meta().logs()).toStrictEqual(sendRes.logs());
		expect(sendRes.logs()[logIndex]).toStrictEqual(
			`Program ${programId} success`,
		);
	} else {
		throw new Error("Unexpected tx failure");
	}
};
