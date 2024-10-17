//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct InitializeChecked {
    /// The stake account to initialize
    pub stake: solana_program::pubkey::Pubkey,
    /// Rent sysvar
    pub rent: solana_program::pubkey::Pubkey,
    /// stake's new stake authority
    pub stake_authority: solana_program::pubkey::Pubkey,
    /// stake's new withdraw authority
    pub withdraw_authority: solana_program::pubkey::Pubkey,
}

impl InitializeChecked {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.stake, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.rent, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.stake_authority,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.withdraw_authority,
            true,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = InitializeCheckedInstructionData::new()
            .try_to_vec()
            .unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::STAKE_PROGRAM_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct InitializeCheckedInstructionData {
    discriminator: [u8; 8],
}

impl InitializeCheckedInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [219, 90, 58, 161, 139, 88, 246, 28],
        }
    }
}

impl Default for InitializeCheckedInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

/// Instruction builder for `InitializeChecked`.
///
/// ### Accounts:
///
///   0. `[writable]` stake
///   1. `[optional]` rent (default to `SysvarRent111111111111111111111111111111111`)
///   2. `[]` stake_authority
///   3. `[signer]` withdraw_authority
#[derive(Clone, Debug, Default)]
pub struct InitializeCheckedBuilder {
    stake: Option<solana_program::pubkey::Pubkey>,
    rent: Option<solana_program::pubkey::Pubkey>,
    stake_authority: Option<solana_program::pubkey::Pubkey>,
    withdraw_authority: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl InitializeCheckedBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// The stake account to initialize
    #[inline(always)]
    pub fn stake(&mut self, stake: solana_program::pubkey::Pubkey) -> &mut Self {
        self.stake = Some(stake);
        self
    }
    /// `[optional account, default to 'SysvarRent111111111111111111111111111111111']`
    /// Rent sysvar
    #[inline(always)]
    pub fn rent(&mut self, rent: solana_program::pubkey::Pubkey) -> &mut Self {
        self.rent = Some(rent);
        self
    }
    /// stake's new stake authority
    #[inline(always)]
    pub fn stake_authority(
        &mut self,
        stake_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.stake_authority = Some(stake_authority);
        self
    }
    /// stake's new withdraw authority
    #[inline(always)]
    pub fn withdraw_authority(
        &mut self,
        withdraw_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.withdraw_authority = Some(withdraw_authority);
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
        let accounts = InitializeChecked {
            stake: self.stake.expect("stake is not set"),
            rent: self.rent.unwrap_or(solana_program::pubkey!(
                "SysvarRent111111111111111111111111111111111"
            )),
            stake_authority: self.stake_authority.expect("stake_authority is not set"),
            withdraw_authority: self
                .withdraw_authority
                .expect("withdraw_authority is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `initialize_checked` CPI accounts.
pub struct InitializeCheckedCpiAccounts<'a, 'b> {
    /// The stake account to initialize
    pub stake: &'b solana_program::account_info::AccountInfo<'a>,
    /// Rent sysvar
    pub rent: &'b solana_program::account_info::AccountInfo<'a>,
    /// stake's new stake authority
    pub stake_authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// stake's new withdraw authority
    pub withdraw_authority: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `initialize_checked` CPI instruction.
pub struct InitializeCheckedCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The stake account to initialize
    pub stake: &'b solana_program::account_info::AccountInfo<'a>,
    /// Rent sysvar
    pub rent: &'b solana_program::account_info::AccountInfo<'a>,
    /// stake's new stake authority
    pub stake_authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// stake's new withdraw authority
    pub withdraw_authority: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> InitializeCheckedCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: InitializeCheckedCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            stake: accounts.stake,
            rent: accounts.rent,
            stake_authority: accounts.stake_authority,
            withdraw_authority: accounts.withdraw_authority,
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
            *self.rent.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.stake_authority.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.withdraw_authority.key,
            true,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = InitializeCheckedInstructionData::new()
            .try_to_vec()
            .unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::STAKE_PROGRAM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(5 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.stake.clone());
        account_infos.push(self.rent.clone());
        account_infos.push(self.stake_authority.clone());
        account_infos.push(self.withdraw_authority.clone());
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

/// Instruction builder for `InitializeChecked` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` stake
///   1. `[]` rent
///   2. `[]` stake_authority
///   3. `[signer]` withdraw_authority
#[derive(Clone, Debug)]
pub struct InitializeCheckedCpiBuilder<'a, 'b> {
    instruction: Box<InitializeCheckedCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> InitializeCheckedCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(InitializeCheckedCpiBuilderInstruction {
            __program: program,
            stake: None,
            rent: None,
            stake_authority: None,
            withdraw_authority: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// The stake account to initialize
    #[inline(always)]
    pub fn stake(&mut self, stake: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.stake = Some(stake);
        self
    }
    /// Rent sysvar
    #[inline(always)]
    pub fn rent(&mut self, rent: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.rent = Some(rent);
        self
    }
    /// stake's new stake authority
    #[inline(always)]
    pub fn stake_authority(
        &mut self,
        stake_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.stake_authority = Some(stake_authority);
        self
    }
    /// stake's new withdraw authority
    #[inline(always)]
    pub fn withdraw_authority(
        &mut self,
        withdraw_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.withdraw_authority = Some(withdraw_authority);
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
        let instruction = InitializeCheckedCpi {
            __program: self.instruction.__program,

            stake: self.instruction.stake.expect("stake is not set"),

            rent: self.instruction.rent.expect("rent is not set"),

            stake_authority: self
                .instruction
                .stake_authority
                .expect("stake_authority is not set"),

            withdraw_authority: self
                .instruction
                .withdraw_authority
                .expect("withdraw_authority is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct InitializeCheckedCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    stake: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    rent: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    stake_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    withdraw_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}