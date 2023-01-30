// Copyright 2021-2022 Zeitgeist PM LLC.
//
// This file is part of Zeitgeist.
//
// Zeitgeist is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the
// Free Software Foundation, either version 3 of the License, or (at
// your option) any later version.
//
// Zeitgeist is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Zeitgeist. If not, see <https://www.gnu.org/licenses/>.

#![cfg(feature = "with-zeitgeist-runtime")]

use super::{
    generate_generic_genesis_function, telemetry_endpoints, token_properties, AdditionalChainSpec,
    EndowedAccountWithBalance,
};
use hex_literal::hex;
use sc_service::ChainType;
use sp_core::crypto::UncheckedInto;
use zeitgeist_runtime::parameters::SS58Prefix;

use zeitgeist_primitives::{types::{AccountId,Balance}, constants::{
    BASE,
    ztg::{LIQUIDITY_MINING, LIQUIDITY_MINING_PTD}
}};

#[cfg(feature = "parachain")]
use {
    super::{generate_inflation_config_function, Extensions},
    crate::KUSAMA_PARACHAIN_ID,
    zeitgeist_primitives::constants::ztg::TOTAL_INITIAL_ZTG,
    zeitgeist_runtime::{
        CollatorDeposit, DefaultBlocksPerRound, DefaultCollatorCommission,
        DefaultParachainBondReservePercent, EligibilityValue, MinCollatorStk, PolkadotXcmConfig,
    },
};

cfg_if::cfg_if! {
    if #[cfg(feature = "parachain")] {
        const DEFAULT_STAKING_AMOUNT_ZEITGEIST: u128 = MinCollatorStk::get();
        const DEFAULT_COLLATOR_BALANCE_ZEITGEIST: Option<u128> =
            DEFAULT_STAKING_AMOUNT_ZEITGEIST.checked_add(CollatorDeposit::get());
        pub type ZeitgeistChainSpec = sc_service::GenericChainSpec<zeitgeist_runtime::GenesisConfig, Extensions>;
    } else {
        pub type ZeitgeistChainSpec = sc_service::GenericChainSpec<zeitgeist_runtime::GenesisConfig>;
    }
}

const DEFAULT_SUDO_BALANCE: Balance = 1000 * BASE;

fn endowed_accounts_staging_zeitgeist() -> Vec<EndowedAccountWithBalance> {
    vec![
        // dE4NNpcWPCk8TH3GM9eJV1jauEmHC3rQcxMdnTtrc3NgDGUNo
        #[cfg(feature = "parachain")]
        EndowedAccountWithBalance(
            hex!["ec9a6c37972582ce411546f96f806cfc2bb0670f60c30cbc3ad4276834b0253c"].into(),
            DEFAULT_COLLATOR_BALANCE_ZEITGEIST.unwrap(),
        ),
        // dDzXWuvDPSRXMQFAq2cJdr9NtEjtB8bohFhE3Ap9yM9s7rUQf
        #[cfg(feature = "parachain")]
        EndowedAccountWithBalance(
            hex!["42a1ef95149913305fb05b6ac325ab9ed4b68c8d7aa60e3ea4daf4237dd9fc09"].into(),
            DEFAULT_COLLATOR_BALANCE_ZEITGEIST.unwrap(),
        ),
        // dE375YCauT8vxvXwzBGaeCfPsKTXuuBpJaqCsBqRhoySNdmtE
        #[cfg(feature = "parachain")]
        EndowedAccountWithBalance(
            hex!["b4b3541a95c83a71de977a6f1e7e66e594a4d47c48b030802c90ba589c8bba16"].into(),
            DEFAULT_COLLATOR_BALANCE_ZEITGEIST.unwrap(),
        ),
        EndowedAccountWithBalance(root_key_staging_zeitgeist(), DEFAULT_SUDO_BALANCE),
    ]
}

#[cfg(feature = "parachain")]
fn additional_chain_spec_staging_zeitgeist(
    parachain_id: cumulus_primitives_core::ParaId,
) -> AdditionalChainSpec {
    use zeitgeist_primitives::constants::BASE;

    AdditionalChainSpec {
        blocks_per_round: DefaultBlocksPerRound::get(),
        candidates: vec![
            (
                hex!["ec9a6c37972582ce411546f96f806cfc2bb0670f60c30cbc3ad4276834b0253c"].into(),
                hex!["725d4d2948ae3a703f7a4911daa6d3022b45dc54fe1998ea88cb33a6f2bd805a"]
                    .unchecked_into(),
                DEFAULT_STAKING_AMOUNT_ZEITGEIST,
            ),
            (
                hex!["42a1ef95149913305fb05b6ac325ab9ed4b68c8d7aa60e3ea4daf4237dd9fc09"].into(),
                hex!["2cb04566bb52665950acf535c6b03312b00d896a3e33534e09dc948e16c06042"]
                    .unchecked_into(),
                DEFAULT_STAKING_AMOUNT_ZEITGEIST,
            ),
            (
                hex!["b4b3541a95c83a71de977a6f1e7e66e594a4d47c48b030802c90ba589c8bba16"].into(),
                hex!["e23846832242a083b94df7640257a243fe1c5a730890b254600d953ddd65011c"]
                    .unchecked_into(),
                DEFAULT_STAKING_AMOUNT_ZEITGEIST,
            ),
        ],
        collator_commission: DefaultCollatorCommission::get(),
        inflation_info: inflation_config(
            Perbill::from_parts(50),
            Perbill::from_parts(50),
            Perbill::from_parts(50),
            TOTAL_INITIAL_ZTG * BASE,
        ),
        nominations: vec![],
        parachain_bond_reserve_percent: DefaultParachainBondReservePercent::get(),
        parachain_id,
    }
}

#[cfg(not(feature = "parachain"))]
fn additional_chain_spec_staging_zeitgeist() -> AdditionalChainSpec {
    AdditionalChainSpec {
        initial_authorities: vec![(
            // Aura
            hex!["5ce5033dba3f6f730f11c20d00c34c4d3fbe23eb81471255bfde689f25dc966e"]
                .unchecked_into(),
            // Grandpa
            hex!["ffd00bcb47e83ed435ce55264cf89969041a5108fdfb3198c79dfe0b75f66600"]
                .unchecked_into(),
        )],
    }
}

pub(super) fn get_wasm() -> Result<&'static [u8], String> {
    zeitgeist_runtime::WASM_BINARY.ok_or_else(|| "WASM binary is not available".to_string())
}

#[inline]
fn root_key_staging_zeitgeist() -> AccountId {
    hex!["e6c622c6f2eaba444b68955501e535247c192b35e7b3e44e4c1dc24a514b4965"].into()
}

generate_generic_genesis_function!(
    zeitgeist_runtime,
    sudo: zeitgeist_runtime::SudoConfig { key: Some(root_key_staging_zeitgeist()) },
);

#[cfg(feature = "parachain")]
generate_inflation_config_function!(zeitgeist_runtime);

pub fn zeitgeist_staging_config() -> Result<ZeitgeistChainSpec, String> {
    let wasm = get_wasm()?;

    Ok(ZeitgeistChainSpec::from_genesis(
        "Zeitgeist Staging",
        "zeitgeist_staging",
        ChainType::Live,
        move || {
            generic_genesis(
                additional_chain_spec_staging_zeitgeist(
                    #[cfg(feature = "parachain")]
                    ZEITGEIST_POLKADOT_PARACHAIN_ID.into(),
                ),
                endowed_accounts_staging_zeitgeist(),
                wasm,
            )
        },
        vec![],
        telemetry_endpoints(),
        Some("zeitgeist"),
        None,
        Some(token_properties("ZTG", SS58Prefix::get())),
        #[cfg(feature = "parachain")]
        crate::chain_spec::Extensions {
            relay_chain: "polkadot".into(),
            parachain_id: ZEITGEIST_POLKADOT_PARACHAIN_ID.into(),
        },
        #[cfg(not(feature = "parachain"))]
        Default::default(),
    ))
}
