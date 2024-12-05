/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  combineCodec,
  getStructDecoder,
  getStructEncoder,
  getU64Decoder,
  getU64Encoder,
  getU8Decoder,
  getU8Encoder,
  transformEncoder,
  type Address,
  type Codec,
  type Decoder,
  type Encoder,
  type IAccountMeta,
  type IAccountSignerMeta,
  type IInstruction,
  type IInstructionWithAccounts,
  type IInstructionWithData,
  type ReadonlySignerAccount,
  type TransactionSigner,
  type WritableAccount,
} from '@solana/web3.js';
import { STAKE_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export const MOVE_LAMPORTS_DISCRIMINATOR = 17;

export function getMoveLamportsDiscriminatorBytes() {
  return getU8Encoder().encode(MOVE_LAMPORTS_DISCRIMINATOR);
}

export type MoveLamportsInstruction<
  TProgram extends string = typeof STAKE_PROGRAM_ADDRESS,
  TAccountSourceStake extends string | IAccountMeta<string> = string,
  TAccountDestinationStake extends string | IAccountMeta<string> = string,
  TAccountStakeAuthority extends string | IAccountMeta<string> = string,
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountSourceStake extends string
        ? WritableAccount<TAccountSourceStake>
        : TAccountSourceStake,
      TAccountDestinationStake extends string
        ? WritableAccount<TAccountDestinationStake>
        : TAccountDestinationStake,
      TAccountStakeAuthority extends string
        ? ReadonlySignerAccount<TAccountStakeAuthority> &
            IAccountSignerMeta<TAccountStakeAuthority>
        : TAccountStakeAuthority,
      ...TRemainingAccounts,
    ]
  >;

export type MoveLamportsInstructionData = {
  discriminator: number;
  args: bigint;
};

export type MoveLamportsInstructionDataArgs = { args: number | bigint };

export function getMoveLamportsInstructionDataEncoder(): Encoder<MoveLamportsInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['args', getU64Encoder()],
    ]),
    (value) => ({ ...value, discriminator: MOVE_LAMPORTS_DISCRIMINATOR })
  );
}

export function getMoveLamportsInstructionDataDecoder(): Decoder<MoveLamportsInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['args', getU64Decoder()],
  ]);
}

export function getMoveLamportsInstructionDataCodec(): Codec<
  MoveLamportsInstructionDataArgs,
  MoveLamportsInstructionData
> {
  return combineCodec(
    getMoveLamportsInstructionDataEncoder(),
    getMoveLamportsInstructionDataDecoder()
  );
}

export type MoveLamportsInput<
  TAccountSourceStake extends string = string,
  TAccountDestinationStake extends string = string,
  TAccountStakeAuthority extends string = string,
> = {
  /** Active or inactive source stake account */
  sourceStake: Address<TAccountSourceStake>;
  /** Mergeable destination stake account */
  destinationStake: Address<TAccountDestinationStake>;
  /** Stake authority */
  stakeAuthority: TransactionSigner<TAccountStakeAuthority>;
  args: MoveLamportsInstructionDataArgs['args'];
};

export function getMoveLamportsInstruction<
  TAccountSourceStake extends string,
  TAccountDestinationStake extends string,
  TAccountStakeAuthority extends string,
  TProgramAddress extends Address = typeof STAKE_PROGRAM_ADDRESS,
>(
  input: MoveLamportsInput<
    TAccountSourceStake,
    TAccountDestinationStake,
    TAccountStakeAuthority
  >,
  config?: { programAddress?: TProgramAddress }
): MoveLamportsInstruction<
  TProgramAddress,
  TAccountSourceStake,
  TAccountDestinationStake,
  TAccountStakeAuthority
> {
  // Program address.
  const programAddress = config?.programAddress ?? STAKE_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    sourceStake: { value: input.sourceStake ?? null, isWritable: true },
    destinationStake: {
      value: input.destinationStake ?? null,
      isWritable: true,
    },
    stakeAuthority: { value: input.stakeAuthority ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Original args.
  const args = { ...input };

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.sourceStake),
      getAccountMeta(accounts.destinationStake),
      getAccountMeta(accounts.stakeAuthority),
    ],
    programAddress,
    data: getMoveLamportsInstructionDataEncoder().encode(
      args as MoveLamportsInstructionDataArgs
    ),
  } as MoveLamportsInstruction<
    TProgramAddress,
    TAccountSourceStake,
    TAccountDestinationStake,
    TAccountStakeAuthority
  >;

  return instruction;
}

export type ParsedMoveLamportsInstruction<
  TProgram extends string = typeof STAKE_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    /** Active or inactive source stake account */
    sourceStake: TAccountMetas[0];
    /** Mergeable destination stake account */
    destinationStake: TAccountMetas[1];
    /** Stake authority */
    stakeAuthority: TAccountMetas[2];
  };
  data: MoveLamportsInstructionData;
};

export function parseMoveLamportsInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedMoveLamportsInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 3) {
    // TODO: Coded error.
    throw new Error('Not enough accounts');
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts![accountIndex]!;
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      sourceStake: getNextAccount(),
      destinationStake: getNextAccount(),
      stakeAuthority: getNextAccount(),
    },
    data: getMoveLamportsInstructionDataDecoder().decode(instruction.data),
  };
}