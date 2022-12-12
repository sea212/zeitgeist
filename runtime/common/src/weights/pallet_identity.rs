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

//! Autogenerated weights for pallet_identity
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-25, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/zeitgeist
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_identity
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./misc/frame_weight_template.hbs
// --output=./runtime/common/src/weights/

#![allow(unused_parens)]
#![allow(unused_imports)]

use core::marker::PhantomData;
use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};

/// Weight functions for pallet_identity (automatically generated)
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_identity::weights::WeightInfo for WeightInfo<T> {
    // Storage: Identity Registrars (r:1 w:1)
    fn add_registrar(r: u32) -> Weight {
        Weight::from_ref_time(20_161_000)
            // Standard Error: 66_000
            .saturating_add(Weight::from_ref_time(439_000).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Identity IdentityOf (r:1 w:1)
    fn set_identity(_r: u32, x: u32) -> Weight {
        Weight::from_ref_time(53_093_000)
            // Standard Error: 19_000
            .saturating_add(Weight::from_ref_time(736_000).saturating_mul(x as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Identity IdentityOf (r:1 w:0)
    // Storage: Identity SubsOf (r:1 w:1)
    // Storage: Identity SuperOf (r:1 w:1)
    fn set_subs_new(s: u32) -> Weight {
        Weight::from_ref_time(43_974_000)
            // Standard Error: 41_000
            .saturating_add(Weight::from_ref_time(5_696_000).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(s as u64)))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
            .saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
    }
    // Storage: Identity IdentityOf (r:1 w:0)
    // Storage: Identity SubsOf (r:1 w:1)
    // Storage: Identity SuperOf (r:0 w:1)
    fn set_subs_old(p: u32) -> Weight {
        Weight::from_ref_time(42_466_000)
            // Standard Error: 20_000
            .saturating_add(Weight::from_ref_time(1_425_000).saturating_mul(p as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
            .saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(p as u64)))
    }
    // Storage: Identity SubsOf (r:1 w:1)
    // Storage: Identity IdentityOf (r:1 w:1)
    // Storage: Identity SuperOf (r:0 w:64)
    fn clear_identity(_r: u32, s: u32, x: u32) -> Weight {
        Weight::from_ref_time(54_503_000)
            // Standard Error: 32_000
            .saturating_add(Weight::from_ref_time(1_464_000).saturating_mul(s as u64))
            // Standard Error: 32_000
            .saturating_add(Weight::from_ref_time(398_000).saturating_mul(x as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
            .saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
    }
    // Storage: Identity Registrars (r:1 w:0)
    // Storage: Identity IdentityOf (r:1 w:1)
    fn request_judgement(r: u32, x: u32) -> Weight {
        Weight::from_ref_time(41_292_000)
            // Standard Error: 200_000
            .saturating_add(Weight::from_ref_time(987_000).saturating_mul(r as u64))
            // Standard Error: 13_000
            .saturating_add(Weight::from_ref_time(921_000).saturating_mul(x as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Identity IdentityOf (r:1 w:1)
    fn cancel_request(r: u32, x: u32) -> Weight {
        Weight::from_ref_time(32_053_000)
            // Standard Error: 337_000
            .saturating_add(Weight::from_ref_time(2_829_000).saturating_mul(r as u64))
            // Standard Error: 22_000
            .saturating_add(Weight::from_ref_time(856_000).saturating_mul(x as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Identity Registrars (r:1 w:1)
    fn set_fee(r: u32) -> Weight {
        Weight::from_ref_time(8_819_000)
            // Standard Error: 21_000
            .saturating_add(Weight::from_ref_time(400_000).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Identity Registrars (r:1 w:1)
    fn set_account_id(r: u32) -> Weight {
        Weight::from_ref_time(7_687_000)
            // Standard Error: 47_000
            .saturating_add(Weight::from_ref_time(504_000).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Identity Registrars (r:1 w:1)
    fn set_fields(r: u32) -> Weight {
        Weight::from_ref_time(9_073_000)
            // Standard Error: 49_000
            .saturating_add(Weight::from_ref_time(186_000).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Identity Registrars (r:1 w:0)
    // Storage: Identity IdentityOf (r:1 w:1)
    fn provide_judgement(_r: u32, x: u32) -> Weight {
        Weight::from_ref_time(41_154_000)
            // Standard Error: 11_000
            .saturating_add(Weight::from_ref_time(670_000).saturating_mul(x as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Identity SubsOf (r:1 w:1)
    // Storage: Identity IdentityOf (r:1 w:1)
    // Storage: System Account (r:2 w:2)
    // Storage: Identity SuperOf (r:0 w:64)
    fn kill_identity(_r: u32, s: u32, _x: u32) -> Weight {
        Weight::from_ref_time(80_465_000)
            // Standard Error: 32_000
            .saturating_add(Weight::from_ref_time(1_386_000).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(4 as u64))
            .saturating_add(T::DbWeight::get().writes(4 as u64))
            .saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
    }
    // Storage: Identity IdentityOf (r:1 w:0)
    // Storage: Identity SuperOf (r:1 w:1)
    // Storage: Identity SubsOf (r:1 w:1)
    fn add_sub(s: u32) -> Weight {
        Weight::from_ref_time(55_008_000)
            // Standard Error: 15_000
            .saturating_add(Weight::from_ref_time(163_000).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    // Storage: Identity IdentityOf (r:1 w:0)
    // Storage: Identity SuperOf (r:1 w:1)
    fn rename_sub(s: u32) -> Weight {
        Weight::from_ref_time(16_967_000)
            // Standard Error: 3_000
            .saturating_add(Weight::from_ref_time(75_000).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Identity IdentityOf (r:1 w:0)
    // Storage: Identity SuperOf (r:1 w:1)
    // Storage: Identity SubsOf (r:1 w:1)
    fn remove_sub(s: u32) -> Weight {
        Weight::from_ref_time(58_373_000)
            // Standard Error: 24_000
            .saturating_add(Weight::from_ref_time(63_000).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    // Storage: Identity SuperOf (r:1 w:1)
    // Storage: Identity SubsOf (r:1 w:1)
    fn quit_sub(s: u32) -> Weight {
        Weight::from_ref_time(34_045_000)
            // Standard Error: 6_000
            .saturating_add(Weight::from_ref_time(184_000).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
}
