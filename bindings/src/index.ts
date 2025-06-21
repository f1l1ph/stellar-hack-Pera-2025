import { Buffer } from "buffer";
import { Address } from '@stellar/stellar-sdk';
import {
  AssembledTransaction,
  Client as ContractClient,
  ClientOptions as ContractClientOptions,
  MethodOptions,
  Result,
  Spec as ContractSpec,
} from '@stellar/stellar-sdk/contract';
import type {
  u32,
  i32,
  u64,
  i64,
  u128,
  i128,
  u256,
  i256,
  Option,
  Typepoint,
  Duration,
} from '@stellar/stellar-sdk/contract';
export * from '@stellar/stellar-sdk'
export * as contract from '@stellar/stellar-sdk/contract'
export * as rpc from '@stellar/stellar-sdk/rpc'

if (typeof window !== 'undefined') {
  //@ts-ignore Buffer exists
  window.Buffer = window.Buffer || Buffer;
}




export type DataKey = {tag: "Admin", values: void} | {tag: "PoolCounter", values: void} | {tag: "Pool", values: readonly [u64]} | {tag: "PoolExists", values: readonly [string, string]} | {tag: "PoolTotalLiquidity", values: readonly [u64]} | {tag: "PoolLockedCollateral", values: readonly [u64]} | {tag: "PoolTotalLpShares", values: readonly [u64]} | {tag: "PoolLpShares", values: readonly [u64, string]} | {tag: "OptionCounter", values: void} | {tag: "Option", values: readonly [u64]};


export interface PoolData {
  is_active: boolean;
  name: string;
  pool_id: u64;
  price_feed: string;
  stable_token: string;
  underlying_asset: string;
}

export type OptionType = {tag: "Call", values: void} | {tag: "Put", values: void};


export interface OptionData {
  amount: i128;
  buyer: string;
  collateral: i128;
  expiry: u64;
  is_active: boolean;
  is_exercised: boolean;
  opt_type: OptionType;
  pool_id: u64;
  premium_paid: i128;
  strike: i128;
}

export enum OptionsError {
  NotInitialized = 1,
  AlreadyInitialized = 2,
  Unauthorized = 3,
  InvalidAmount = 4,
  InsufficientLiquidity = 5,
  OptionNotFound = 6,
  OptionNotActive = 7,
  OptionExpired = 8,
  NotOptionOwner = 9,
  NotInTheMoney = 10,
  InsufficientShares = 11,
  PoolNotFound = 12,
  PoolNotActive = 13,
  PoolAlreadyExists = 14,
}

export interface Client {
  /**
   * Construct and simulate a initialize transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Initialize the contract with admin
   */
  initialize: ({admin}: {admin: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a add_liquidity_pool transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Admin function to add a new liquidity pool
   */
  add_liquidity_pool: ({stable_token, underlying_asset, price_feed, name}: {stable_token: string, underlying_asset: string, price_feed: string, name: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<u64>>

  /**
   * Construct and simulate a set_pool_status transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Admin function to toggle pool active status
   */
  set_pool_status: ({pool_id, is_active}: {pool_id: u64, is_active: boolean}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a provide_liquidity transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Provide liquidity to a specific pool
   */
  provide_liquidity: ({pool_id, provider, amount}: {pool_id: u64, provider: string, amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<i128>>

  /**
   * Construct and simulate a withdraw_liquidity transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Withdraw liquidity from a specific pool
   */
  withdraw_liquidity: ({pool_id, provider, share_amount}: {pool_id: u64, provider: string, share_amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<i128>>

  /**
   * Construct and simulate a buy_option transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Buy an option from a specific pool
   */
  buy_option: ({pool_id, buyer, opt_type, strike, expiry, amount}: {pool_id: u64, buyer: string, opt_type: OptionType, strike: i128, expiry: u64, amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<u64>>

  /**
   * Construct and simulate a exercise_option transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Exercise an option (American-style)
   */
  exercise_option: ({option_id}: {option_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<i128>>

  /**
   * Construct and simulate a expire_option transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Expire an option (release collateral)
   */
  expire_option: ({option_id}: {option_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a get_pool_counter transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_pool_counter: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<u64>>

  /**
   * Construct and simulate a get_pool transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_pool: ({pool_id}: {pool_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<PoolData>>

  /**
   * Construct and simulate a get_pool_by_assets transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_pool_by_assets: ({stable_token, underlying_asset}: {stable_token: string, underlying_asset: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<u64>>

  /**
   * Construct and simulate a get_pool_total_liquidity transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_pool_total_liquidity: ({pool_id}: {pool_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<i128>>

  /**
   * Construct and simulate a get_pool_locked_collateral transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_pool_locked_collateral: ({pool_id}: {pool_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<i128>>

  /**
   * Construct and simulate a get_pool_total_lp_shares transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_pool_total_lp_shares: ({pool_id}: {pool_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<i128>>

  /**
   * Construct and simulate a get_pool_lp_shares transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_pool_lp_shares: ({pool_id, provider}: {pool_id: u64, provider: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<i128>>

  /**
   * Construct and simulate a get_option_counter transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_option_counter: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<u64>>

  /**
   * Construct and simulate a get_option transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_option: ({option_id}: {option_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<OptionData>>

  /**
   * Construct and simulate a get_admin transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_admin: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<string>>

  /**
   * Construct and simulate a get_price_from_feed transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_price_from_feed: ({price_feed}: {price_feed: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<i128>>

  /**
   * Construct and simulate a update_pool_price_feed transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  update_pool_price_feed: ({pool_id, new_feed}: {pool_id: u64, new_feed: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a get_all_pools transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_all_pools: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Array<u64>>>

}
export class Client extends ContractClient {
  static async deploy<T = Client>(
    /** Options for initializing a Client as well as for calling a method, with extras specific to deploying. */
    options: MethodOptions &
      Omit<ContractClientOptions, "contractId"> & {
        /** The hash of the Wasm blob, which must already be installed on-chain. */
        wasmHash: Buffer | string;
        /** Salt used to generate the contract's ID. Passed through to {@link Operation.createCustomContract}. Default: random. */
        salt?: Buffer | Uint8Array;
        /** The format used to decode `wasmHash`, if it's provided as a string. */
        format?: "hex" | "base64";
      }
  ): Promise<AssembledTransaction<T>> {
    return ContractClient.deploy(null, options)
  }
  constructor(public readonly options: ContractClientOptions) {
    super(
      new ContractSpec([ "AAAAAgAAAAAAAAAAAAAAB0RhdGFLZXkAAAAACgAAAAAAAAAAAAAABUFkbWluAAAAAAAAAAAAAAAAAAALUG9vbENvdW50ZXIAAAAAAQAAAAAAAAAEUG9vbAAAAAEAAAAGAAAAAQAAAAAAAAAKUG9vbEV4aXN0cwAAAAAAAgAAABMAAAATAAAAAQAAAAAAAAASUG9vbFRvdGFsTGlxdWlkaXR5AAAAAAABAAAABgAAAAEAAAAAAAAAFFBvb2xMb2NrZWRDb2xsYXRlcmFsAAAAAQAAAAYAAAABAAAAAAAAABFQb29sVG90YWxMcFNoYXJlcwAAAAAAAAEAAAAGAAAAAQAAAAAAAAAMUG9vbExwU2hhcmVzAAAAAgAAAAYAAAATAAAAAAAAAAAAAAANT3B0aW9uQ291bnRlcgAAAAAAAAEAAAAAAAAABk9wdGlvbgAAAAAAAQAAAAY=",
        "AAAAAQAAAAAAAAAAAAAACFBvb2xEYXRhAAAABgAAAAAAAAAJaXNfYWN0aXZlAAAAAAAAAQAAAAAAAAAEbmFtZQAAABAAAAAAAAAAB3Bvb2xfaWQAAAAABgAAAAAAAAAKcHJpY2VfZmVlZAAAAAAAEwAAAAAAAAAMc3RhYmxlX3Rva2VuAAAAEwAAAAAAAAAQdW5kZXJseWluZ19hc3NldAAAABM=",
        "AAAAAgAAAAAAAAAAAAAACk9wdGlvblR5cGUAAAAAAAIAAAAAAAAAAAAAAARDYWxsAAAAAAAAAAAAAAADUHV0AA==",
        "AAAAAQAAAAAAAAAAAAAACk9wdGlvbkRhdGEAAAAAAAoAAAAAAAAABmFtb3VudAAAAAAACwAAAAAAAAAFYnV5ZXIAAAAAAAATAAAAAAAAAApjb2xsYXRlcmFsAAAAAAALAAAAAAAAAAZleHBpcnkAAAAAAAYAAAAAAAAACWlzX2FjdGl2ZQAAAAAAAAEAAAAAAAAADGlzX2V4ZXJjaXNlZAAAAAEAAAAAAAAACG9wdF90eXBlAAAH0AAAAApPcHRpb25UeXBlAAAAAAAAAAAAB3Bvb2xfaWQAAAAABgAAAAAAAAAMcHJlbWl1bV9wYWlkAAAACwAAAAAAAAAGc3RyaWtlAAAAAAAL",
        "AAAAAwAAAAAAAAAAAAAADE9wdGlvbnNFcnJvcgAAAA4AAAAAAAAADk5vdEluaXRpYWxpemVkAAAAAAABAAAAAAAAABJBbHJlYWR5SW5pdGlhbGl6ZWQAAAAAAAIAAAAAAAAADFVuYXV0aG9yaXplZAAAAAMAAAAAAAAADUludmFsaWRBbW91bnQAAAAAAAAEAAAAAAAAABVJbnN1ZmZpY2llbnRMaXF1aWRpdHkAAAAAAAAFAAAAAAAAAA5PcHRpb25Ob3RGb3VuZAAAAAAABgAAAAAAAAAPT3B0aW9uTm90QWN0aXZlAAAAAAcAAAAAAAAADU9wdGlvbkV4cGlyZWQAAAAAAAAIAAAAAAAAAA5Ob3RPcHRpb25Pd25lcgAAAAAACQAAAAAAAAANTm90SW5UaGVNb25leQAAAAAAAAoAAAAAAAAAEkluc3VmZmljaWVudFNoYXJlcwAAAAAACwAAAAAAAAAMUG9vbE5vdEZvdW5kAAAADAAAAAAAAAANUG9vbE5vdEFjdGl2ZQAAAAAAAA0AAAAAAAAAEVBvb2xBbHJlYWR5RXhpc3RzAAAAAAAADg==",
        "AAAAAAAAACJJbml0aWFsaXplIHRoZSBjb250cmFjdCB3aXRoIGFkbWluAAAAAAAKaW5pdGlhbGl6ZQAAAAAAAQAAAAAAAAAFYWRtaW4AAAAAAAATAAAAAA==",
        "AAAAAAAAACpBZG1pbiBmdW5jdGlvbiB0byBhZGQgYSBuZXcgbGlxdWlkaXR5IHBvb2wAAAAAABJhZGRfbGlxdWlkaXR5X3Bvb2wAAAAAAAQAAAAAAAAADHN0YWJsZV90b2tlbgAAABMAAAAAAAAAEHVuZGVybHlpbmdfYXNzZXQAAAATAAAAAAAAAApwcmljZV9mZWVkAAAAAAATAAAAAAAAAARuYW1lAAAAEAAAAAEAAAAG",
        "AAAAAAAAACtBZG1pbiBmdW5jdGlvbiB0byB0b2dnbGUgcG9vbCBhY3RpdmUgc3RhdHVzAAAAAA9zZXRfcG9vbF9zdGF0dXMAAAAAAgAAAAAAAAAHcG9vbF9pZAAAAAAGAAAAAAAAAAlpc19hY3RpdmUAAAAAAAABAAAAAA==",
        "AAAAAAAAACRQcm92aWRlIGxpcXVpZGl0eSB0byBhIHNwZWNpZmljIHBvb2wAAAARcHJvdmlkZV9saXF1aWRpdHkAAAAAAAADAAAAAAAAAAdwb29sX2lkAAAAAAYAAAAAAAAACHByb3ZpZGVyAAAAEwAAAAAAAAAGYW1vdW50AAAAAAALAAAAAQAAAAs=",
        "AAAAAAAAACdXaXRoZHJhdyBsaXF1aWRpdHkgZnJvbSBhIHNwZWNpZmljIHBvb2wAAAAAEndpdGhkcmF3X2xpcXVpZGl0eQAAAAAAAwAAAAAAAAAHcG9vbF9pZAAAAAAGAAAAAAAAAAhwcm92aWRlcgAAABMAAAAAAAAADHNoYXJlX2Ftb3VudAAAAAsAAAABAAAACw==",
        "AAAAAAAAACJCdXkgYW4gb3B0aW9uIGZyb20gYSBzcGVjaWZpYyBwb29sAAAAAAAKYnV5X29wdGlvbgAAAAAABgAAAAAAAAAHcG9vbF9pZAAAAAAGAAAAAAAAAAVidXllcgAAAAAAABMAAAAAAAAACG9wdF90eXBlAAAH0AAAAApPcHRpb25UeXBlAAAAAAAAAAAABnN0cmlrZQAAAAAACwAAAAAAAAAGZXhwaXJ5AAAAAAAGAAAAAAAAAAZhbW91bnQAAAAAAAsAAAABAAAABg==",
        "AAAAAAAAACNFeGVyY2lzZSBhbiBvcHRpb24gKEFtZXJpY2FuLXN0eWxlKQAAAAAPZXhlcmNpc2Vfb3B0aW9uAAAAAAEAAAAAAAAACW9wdGlvbl9pZAAAAAAAAAYAAAABAAAACw==",
        "AAAAAAAAACVFeHBpcmUgYW4gb3B0aW9uIChyZWxlYXNlIGNvbGxhdGVyYWwpAAAAAAAADWV4cGlyZV9vcHRpb24AAAAAAAABAAAAAAAAAAlvcHRpb25faWQAAAAAAAAGAAAAAA==",
        "AAAAAAAAAAAAAAAQZ2V0X3Bvb2xfY291bnRlcgAAAAAAAAABAAAABg==",
        "AAAAAAAAAAAAAAAIZ2V0X3Bvb2wAAAABAAAAAAAAAAdwb29sX2lkAAAAAAYAAAABAAAH0AAAAAhQb29sRGF0YQ==",
        "AAAAAAAAAAAAAAASZ2V0X3Bvb2xfYnlfYXNzZXRzAAAAAAACAAAAAAAAAAxzdGFibGVfdG9rZW4AAAATAAAAAAAAABB1bmRlcmx5aW5nX2Fzc2V0AAAAEwAAAAEAAAAG",
        "AAAAAAAAAAAAAAAYZ2V0X3Bvb2xfdG90YWxfbGlxdWlkaXR5AAAAAQAAAAAAAAAHcG9vbF9pZAAAAAAGAAAAAQAAAAs=",
        "AAAAAAAAAAAAAAAaZ2V0X3Bvb2xfbG9ja2VkX2NvbGxhdGVyYWwAAAAAAAEAAAAAAAAAB3Bvb2xfaWQAAAAABgAAAAEAAAAL",
        "AAAAAAAAAAAAAAAYZ2V0X3Bvb2xfdG90YWxfbHBfc2hhcmVzAAAAAQAAAAAAAAAHcG9vbF9pZAAAAAAGAAAAAQAAAAs=",
        "AAAAAAAAAAAAAAASZ2V0X3Bvb2xfbHBfc2hhcmVzAAAAAAACAAAAAAAAAAdwb29sX2lkAAAAAAYAAAAAAAAACHByb3ZpZGVyAAAAEwAAAAEAAAAL",
        "AAAAAAAAAAAAAAASZ2V0X29wdGlvbl9jb3VudGVyAAAAAAAAAAAAAQAAAAY=",
        "AAAAAAAAAAAAAAAKZ2V0X29wdGlvbgAAAAAAAQAAAAAAAAAJb3B0aW9uX2lkAAAAAAAABgAAAAEAAAfQAAAACk9wdGlvbkRhdGEAAA==",
        "AAAAAAAAAAAAAAAJZ2V0X2FkbWluAAAAAAAAAAAAAAEAAAAT",
        "AAAAAAAAAAAAAAATZ2V0X3ByaWNlX2Zyb21fZmVlZAAAAAABAAAAAAAAAApwcmljZV9mZWVkAAAAAAATAAAAAQAAAAs=",
        "AAAAAAAAAAAAAAAWdXBkYXRlX3Bvb2xfcHJpY2VfZmVlZAAAAAAAAgAAAAAAAAAHcG9vbF9pZAAAAAAGAAAAAAAAAAhuZXdfZmVlZAAAABMAAAAA",
        "AAAAAAAAAAAAAAANZ2V0X2FsbF9wb29scwAAAAAAAAAAAAABAAAD6gAAAAY=" ]),
      options
    )
  }
  public readonly fromJSON = {
    initialize: this.txFromJSON<null>,
        add_liquidity_pool: this.txFromJSON<u64>,
        set_pool_status: this.txFromJSON<null>,
        provide_liquidity: this.txFromJSON<i128>,
        withdraw_liquidity: this.txFromJSON<i128>,
        buy_option: this.txFromJSON<u64>,
        exercise_option: this.txFromJSON<i128>,
        expire_option: this.txFromJSON<null>,
        get_pool_counter: this.txFromJSON<u64>,
        get_pool: this.txFromJSON<PoolData>,
        get_pool_by_assets: this.txFromJSON<u64>,
        get_pool_total_liquidity: this.txFromJSON<i128>,
        get_pool_locked_collateral: this.txFromJSON<i128>,
        get_pool_total_lp_shares: this.txFromJSON<i128>,
        get_pool_lp_shares: this.txFromJSON<i128>,
        get_option_counter: this.txFromJSON<u64>,
        get_option: this.txFromJSON<OptionData>,
        get_admin: this.txFromJSON<string>,
        get_price_from_feed: this.txFromJSON<i128>,
        update_pool_price_feed: this.txFromJSON<null>,
        get_all_pools: this.txFromJSON<Array<u64>>
  }
}