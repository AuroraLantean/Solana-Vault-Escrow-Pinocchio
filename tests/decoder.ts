import type { Address, Decoder } from "@solana/kit";
import {
	getAddressDecoder,
	getArrayDecoder,
	getBooleanDecoder,
	getEnumDecoder,
	getStructDecoder,
	getU8Decoder,
	getU64Decoder,
} from "@solana/kit";

//converted from Rust code. XyzAcct, xyzAcctDecoder, DecodedXyzAcct should all match in field order and types!
export type ConfigAcct = {
	progOwner: Address;
	admin: Address;
	strU8array: number[];
	fee: bigint;
	solBalance: bigint;
	tokenBalance: bigint;
	isAuthorized: boolean;
	status: number;
	//status: Status;
	bump: number;
};
enum Status {
	Waiting,
	Active,
	Expired,
	Paused,
	Canceled,
}
export const configAcctDecoder: Decoder<ConfigAcct> = getStructDecoder([
	//["discriminator", fixDecoderSize(getBytesDecoder(), 4)],
	["progOwner", getAddressDecoder()],
	["admin", getAddressDecoder()],
	["strU8array", getArrayDecoder(getU8Decoder(), { size: 32 })],
	["fee", getU64Decoder()],
	["solBalance", getU64Decoder()],
	["tokenBalance", getU64Decoder()],
	["isAuthorized", getBooleanDecoder()],
	["status", getEnumDecoder(Status)],
	["bump", getU8Decoder()],
	//["padding", getArrayDecoder(getU64Decoder(), { size: 3 })],
]);

export type DecodedConfigAcct = {
	executable: boolean;
	lamports: bigint;
	programAddress: string;
	space: bigint;
	address: string;
	data: {
		progOwner: string;
		admin: string;
		strU8array: number[];
		fee: bigint;
		solBalance: bigint;
		tokenBalance: bigint;
		isAuthorized: boolean;
		status: number; //TODO: decode u8 to Enum
		bump: number;
	};
	exists: boolean;
};
