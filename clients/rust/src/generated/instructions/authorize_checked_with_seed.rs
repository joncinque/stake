//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use crate::generated::types::StakeAuthorize;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::pubkey::Pubkey;

/// Accounts.
pub struct AuthorizeCheckedWithSeed {
    /// The stake account to be updated
    pub stake: solana_program::pubkey::Pubkey,
    /// Base account of stake's authority to be updated
    pub authority_base: solana_program::pubkey::Pubkey,
    /// Clock sysvar
    pub clock: solana_program::pubkey::Pubkey,
    /// stake's new stake or withdraw authority to change to. If stake Lockup is active, the signing lockup authority must follow if updating withdrawer.
    pub new_authority: solana_program::pubkey::Pubkey,
}

impl AuthorizeCheckedWithSeed {
    pub fn instruction(
        &self,
        args: AuthorizeCheckedWithSeedInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: AuthorizeCheckedWithSeedInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.stake, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.authority_base,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.clock, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.new_authority,
            true,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = AuthorizeCheckedWithSeedInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::STAKE_PROGRAM_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct AuthorizeCheckedWithSeedInstructionData {
    discriminator: [u8; 8],
}

impl AuthorizeCheckedWithSeedInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [14, 230, 154, 165, 225, 209, 194, 210],
        }
    }
}

impl Default for AuthorizeCheckedWithSeedInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AuthorizeCheckedWithSeedInstructionArgs {
    pub stake_authorize: StakeAuthorize,
    pub authority_seed: String,
    pub authority_owner: Pubkey,
}

/// Instruction builder for `AuthorizeCheckedWithSeed`.
///
/// ### Accounts:
///
///   0. `[writable]` stake
///   1. `[signer]` authority_base
///   2. `[]` clock
///   3. `[signer]` new_authority
#[derive(Clone, Debug, Default)]
pub struct AuthorizeCheckedWithSeedBuilder {
    stake: Option<solana_program::pubkey::Pubkey>,
    authority_base: Option<solana_program::pubkey::Pubkey>,
    clock: Option<solana_program::pubkey::Pubkey>,
    new_authority: Option<solana_program::pubkey::Pubkey>,
    stake_authorize: Option<StakeAuthorize>,
    authority_seed: Option<String>,
    authority_owner: Option<Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl AuthorizeCheckedWithSeedBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// The stake account to be updated
    #[inline(always)]
    pub fn stake(&mut self, stake: solana_program::pubkey::Pubkey) -> &mut Self {
        self.stake = Some(stake);
        self
    }
    /// Base account of stake's authority to be updated
    #[inline(always)]
    pub fn authority_base(&mut self, authority_base: solana_program::pubkey::Pubkey) -> &mut Self {
        self.authority_base = Some(authority_base);
        self
    }
    /// Clock sysvar
    #[inline(always)]
    pub fn clock(&mut self, clock: solana_program::pubkey::Pubkey) -> &mut Self {
        self.clock = Some(clock);
        self
    }
    /// stake's new stake or withdraw authority to change to. If stake Lockup is active, the signing lockup authority must follow if updating withdrawer.
    #[inline(always)]
    pub fn new_authority(&mut self, new_authority: solana_program::pubkey::Pubkey) -> &mut Self {
        self.new_authority = Some(new_authority);
        self
    }
    #[inline(always)]
    pub fn stake_authorize(&mut self, stake_authorize: StakeAuthorize) -> &mut Self {
        self.stake_authorize = Some(stake_authorize);
        self
    }
    #[inline(always)]
    pub fn authority_seed(&mut self, authority_seed: String) -> &mut Self {
        self.authority_seed = Some(authority_seed);
        self
    }
    #[inline(always)]
    pub fn authority_owner(&mut self, authority_owner: Pubkey) -> &mut Self {
        self.authority_owner = Some(authority_owner);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = AuthorizeCheckedWithSeed {
            stake: self.stake.expect("stake is not set"),
            authority_base: self.authority_base.expect("authority_base is not set"),
            clock: self.clock.expect("clock is not set"),
            new_authority: self.new_authority.expect("new_authority is not set"),
        };
        let args = AuthorizeCheckedWithSeedInstructionArgs {
            stake_authorize: self
                .stake_authorize
                .clone()
                .expect("stake_authorize is not set"),
            authority_seed: self
                .authority_seed
                .clone()
                .expect("authority_seed is not set"),
            authority_owner: self
                .authority_owner
                .clone()
                .expect("authority_owner is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `authorize_checked_with_seed` CPI accounts.
pub struct AuthorizeCheckedWithSeedCpiAccounts<'a, 'b> {
    /// The stake account to be updated
    pub stake: &'b solana_program::account_info::AccountInfo<'a>,
    /// Base account of stake's authority to be updated
    pub authority_base: &'b solana_program::account_info::AccountInfo<'a>,
    /// Clock sysvar
    pub clock: &'b solana_program::account_info::AccountInfo<'a>,
    /// stake's new stake or withdraw authority to change to. If stake Lockup is active, the signing lockup authority must follow if updating withdrawer.
    pub new_authority: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `authorize_checked_with_seed` CPI instruction.
pub struct AuthorizeCheckedWithSeedCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The stake account to be updated
    pub stake: &'b solana_program::account_info::AccountInfo<'a>,
    /// Base account of stake's authority to be updated
    pub authority_base: &'b solana_program::account_info::AccountInfo<'a>,
    /// Clock sysvar
    pub clock: &'b solana_program::account_info::AccountInfo<'a>,
    /// stake's new stake or withdraw authority to change to. If stake Lockup is active, the signing lockup authority must follow if updating withdrawer.
    pub new_authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: AuthorizeCheckedWithSeedInstructionArgs,
}

impl<'a, 'b> AuthorizeCheckedWithSeedCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: AuthorizeCheckedWithSeedCpiAccounts<'a, 'b>,
        args: AuthorizeCheckedWithSeedInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            stake: accounts.stake,
            authority_base: accounts.authority_base,
            clock: accounts.clock,
            new_authority: accounts.new_authority,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.stake.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.authority_base.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.clock.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.new_authority.key,
            true,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = AuthorizeCheckedWithSeedInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::STAKE_PROGRAM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(5 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.stake.clone());
        account_infos.push(self.authority_base.clone());
        account_infos.push(self.clock.clone());
        account_infos.push(self.new_authority.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `AuthorizeCheckedWithSeed` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` stake
///   1. `[signer]` authority_base
///   2. `[]` clock
///   3. `[signer]` new_authority
#[derive(Clone, Debug)]
pub struct AuthorizeCheckedWithSeedCpiBuilder<'a, 'b> {
    instruction: Box<AuthorizeCheckedWithSeedCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> AuthorizeCheckedWithSeedCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(AuthorizeCheckedWithSeedCpiBuilderInstruction {
            __program: program,
            stake: None,
            authority_base: None,
            clock: None,
            new_authority: None,
            stake_authorize: None,
            authority_seed: None,
            authority_owner: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// The stake account to be updated
    #[inline(always)]
    pub fn stake(&mut self, stake: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.stake = Some(stake);
        self
    }
    /// Base account of stake's authority to be updated
    #[inline(always)]
    pub fn authority_base(
        &mut self,
        authority_base: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.authority_base = Some(authority_base);
        self
    }
    /// Clock sysvar
    #[inline(always)]
    pub fn clock(&mut self, clock: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.clock = Some(clock);
        self
    }
    /// stake's new stake or withdraw authority to change to. If stake Lockup is active, the signing lockup authority must follow if updating withdrawer.
    #[inline(always)]
    pub fn new_authority(
        &mut self,
        new_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.new_authority = Some(new_authority);
        self
    }
    #[inline(always)]
    pub fn stake_authorize(&mut self, stake_authorize: StakeAuthorize) -> &mut Self {
        self.instruction.stake_authorize = Some(stake_authorize);
        self
    }
    #[inline(always)]
    pub fn authority_seed(&mut self, authority_seed: String) -> &mut Self {
        self.instruction.authority_seed = Some(authority_seed);
        self
    }
    #[inline(always)]
    pub fn authority_owner(&mut self, authority_owner: Pubkey) -> &mut Self {
        self.instruction.authority_owner = Some(authority_owner);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = AuthorizeCheckedWithSeedInstructionArgs {
            stake_authorize: self
                .instruction
                .stake_authorize
                .clone()
                .expect("stake_authorize is not set"),
            authority_seed: self
                .instruction
                .authority_seed
                .clone()
                .expect("authority_seed is not set"),
            authority_owner: self
                .instruction
                .authority_owner
                .clone()
                .expect("authority_owner is not set"),
        };
        let instruction = AuthorizeCheckedWithSeedCpi {
            __program: self.instruction.__program,

            stake: self.instruction.stake.expect("stake is not set"),

            authority_base: self
                .instruction
                .authority_base
                .expect("authority_base is not set"),

            clock: self.instruction.clock.expect("clock is not set"),

            new_authority: self
                .instruction
                .new_authority
                .expect("new_authority is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct AuthorizeCheckedWithSeedCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    stake: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    authority_base: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    clock: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    new_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    stake_authorize: Option<StakeAuthorize>,
    authority_seed: Option<String>,
    authority_owner: Option<Pubkey>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}