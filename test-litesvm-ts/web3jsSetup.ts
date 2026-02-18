import { Keypair, PublicKey } from "@solana/web3.js";
import jsonPythBTC from "../pricefeeds/pythBTC.json";
import jsonPythETH from "../pricefeeds/pythETH.json";
import jsonPythSOL from "../pricefeeds/pythSOL.json";
import type { SolanaAccount } from "./utils";

const ll = console.log;
ll("\n------== web3jsSetup");
export const ownerKp = new Keypair();
export const adminKp = new Keypair();
export const user1Kp = new Keypair();
export const user2Kp = new Keypair();
export const user3Kp = new Keypair();
export const hackerKp = new Keypair();
export const dgcAuthorityKp = new Keypair();
export const dragonCoinKp = new Keypair();
export const owner = ownerKp.publicKey;
export const admin = adminKp.publicKey;
export const user1 = user1Kp.publicKey;
export const user2 = user2Kp.publicKey;
export const user3 = user3Kp.publicKey;
export const hacker = hackerKp.publicKey;
export const dgcAuthority = dgcAuthorityKp.publicKey;
export const dragonCoin = dragonCoinKp.publicKey;

export const vaultProgAddr = new PublicKey(
	"A9TPi1RSW5apQcZch9CUz5EnuyfSF773zxndJowMrcK3",
);
ll("vaultProgAddr:", vaultProgAddr.toBase58());
export const futureOptionAddr = new PublicKey(
	"CgZEcSRPh1Ay1EYR4VJPTJRYcRkTDjjZhBAjZ5M8keGp",
);
ll("futureOptionAddr:", futureOptionAddr.toBase58());

export const SYSTEM_PROGRAM = new PublicKey("11111111111111111111111111111111"); //default
export const RentSysvar = new PublicKey(
	"SysvarRent111111111111111111111111111111111",
); //RENT_ID
export const usdcMint = new PublicKey(
	"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
); //decimals = 6
export const usdtMint = new PublicKey(
	"Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB",
); //decimals = 6
export const pyusdMint = new PublicKey(
	"2b1kV6DkPAnxd5ixfnxCpjxmKwqjjaYmCZfHsFu24GXo",
); //token2022, decimals = 6... https://docs.paxos.com/guides/stablecoin/pyusd/mainnet
export const usdgMint = new PublicKey(
	"2u1tszSeqZ3qBWF3uNGPFc8TzMk2tdiwknnRMWGWjGWH",
); //token2022, decimals = 6... https://docs.paxos.com/guides/stablecoin/usdg/mainnet
export const ATokenGPvbd = new PublicKey(
	"ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
);
export const decUsdx = 6;

//-------------== PriceFeed
export type PriceFeed = {
	vendor: number;
	feedId: string;
	addr: PublicKey;
	json: SolanaAccount;
};
//https://docs.pyth.network/price-feeds/core/price-feeds/price-feed-ids
export const pythPricefeedBTCUSD: PriceFeed = {
	vendor: 0,
	feedId: "0xe62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43",
	addr: new PublicKey("4cSM2e6rvbGQUFiJbqytoVMi5GgghSMr8LwVrT9VPSPo"),
	json: jsonPythBTC,
};
export const pythPricefeedETHUSD: PriceFeed = {
	vendor: 0,
	feedId: "0xff61491a931112ddf1bd8147cd1b641375f79f5825126d665480874634fd0ace",
	addr: new PublicKey("42amVS4KgzR9rA28tkVYqVXjq9Qa8dcZQMbH5EYFX6XC"),
	json: jsonPythETH,
};
export const pythPricefeedSOLUSD: PriceFeed = {
	vendor: 0,
	feedId: "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d",
	addr: new PublicKey("7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE"),
	json: jsonPythSOL,
};
export const makeFakePricefeed = (target: PublicKey): PriceFeed => {
	return {
		vendor: 255,
		feedId:
			"0xe62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43",
		addr: target,
		json: jsonPythBTC,
	};
};
