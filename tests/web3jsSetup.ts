import { Keypair, PublicKey } from "@solana/web3.js";

export const ownerKp = new Keypair();
export const adminKp = new Keypair();
export const user1Kp = new Keypair();
export const user2Kp = new Keypair();
export const user3Kp = new Keypair();
export const hackerKp = new Keypair();
export const ownerAddr = ownerKp.publicKey;
export const adminAddr = adminKp.publicKey;
export const user1Addr = user1Kp.publicKey;
export const user2Addr = user2Kp.publicKey;
export const user3Addr = user3Kp.publicKey;
export const hackerAddr = hackerKp.publicKey;

const ll = console.log;

export const vaultProgAddr = new PublicKey(
	"7EKqBVYSCmJbt2T8tGSmwzNKnpL29RqcJcyUr9aEEr6e",
);
ll("vaultProgAddr:", vaultProgAddr.toBase58());

export const systemProgram = new PublicKey("11111111111111111111111111111111");
export const usdcMint = new PublicKey(
	"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
);
export const usdtMint = new PublicKey(
	"Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB",
);
