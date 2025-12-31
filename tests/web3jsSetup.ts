import { Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { LiteSVM } from "litesvm";

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

export const initBalc = BigInt(LAMPORTS_PER_SOL) * BigInt(10);
export const svm = new LiteSVM();
svm.airdrop(ownerAddr, initBalc);
svm.airdrop(adminAddr, initBalc);
svm.airdrop(user1Addr, initBalc);
svm.airdrop(user2Addr, initBalc);
svm.airdrop(user3Addr, initBalc);
svm.airdrop(hackerAddr, initBalc);
