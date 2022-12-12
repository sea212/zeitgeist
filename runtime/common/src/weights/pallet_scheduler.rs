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

//! Autogenerated weights for pallet_scheduler
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
// --pallet=pallet_scheduler
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

/// Weight functions for pallet_scheduler (automatically generated)
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_scheduler::weights::WeightInfo for WeightInfo<T> {
    // Storage: Scheduler Agenda (r:2 w:2)
    // Storage: Preimage PreimageFor (r:1 w:1)
    // Storage: Preimage StatusFor (r:1 w:1)
    // Storage: Scheduler Lookup (r:0 w:1)
    fn on_initialize_periodic_named_resolved(s: u32) -> Weight {
        Weight::from_ref_time(50_017_000)
            // Standard Error: 328_000
            .saturating_add(Weight::from_ref_time(30_710_000).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(s as u64)))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
            .saturating_add(T::DbWeight::get().writes((4 as u64).saturating_mul(s as u64)))
    }
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Preimage PreimageFor (r:1 w:1)
    // Storage: Preimage StatusFor (r:1 w:1)
    // Storage: Scheduler Lookup (r:0 w:1)
    fn on_initialize_named_resolved(s: u32) -> Weight {
        Weight::from_ref_time(12_919_000)
            // Standard Error: 293_000
            .saturating_add(Weight::from_ref_time(27_221_000).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(s as u64)))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
            .saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(s as u64)))
    }
    // Storage: Scheduler Agenda (r:2 w:2)
    // Storage: Preimage PreimageFor (r:1 w:1)
    // Storage: Preimage StatusFor (r:1 w:1)
    fn on_initialize_periodic_resolved(s: u32) -> Weight {
        Weight::from_ref_time(70_244_000)
            // Standard Error: 506_000
            .saturating_add(Weight::from_ref_time(29_412_000).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(s as u64)))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
            .saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(s as u64)))
    }
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Preimage PreimageFor (r:1 w:1)
    // Storage: Preimage StatusFor (r:1 w:1)
    fn on_initialize_resolved(s: u32) -> Weight {
        Weight::from_ref_time(35_004_000)
            // Standard Error: 534_000
            .saturating_add(Weight::from_ref_time(26_289_000).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(s as u64)))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
            .saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(s as u64)))
    }
    // Storage: Scheduler Agenda (r:2 w:2)
    // Storage: Preimage PreimageFor (r:1 w:0)
    // Storage: Scheduler Lookup (r:0 w:1)
    fn on_initialize_named_aborted(s: u32) -> Weight {
        Weight::from_ref_time(0)
            // Standard Error: 261_000
            .saturating_add(Weight::from_ref_time(11_151_000).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(s as u64)))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
            .saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
    }
    // Storage: Scheduler Agenda (r:2 w:2)
    // Storage: Preimage PreimageFor (r:1 w:0)
    fn on_initialize_aborted(s: u32) -> Weight {
        Weight::from_ref_time(7_919_000)
            // Standard Error: 52_000
            .saturating_add(Weight::from_ref_time(5_988_000).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(s as u64)))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    // Storage: Scheduler Agenda (r:2 w:2)
    // Storage: Scheduler Lookup (r:0 w:1)
    fn on_initialize_periodic_named(s: u32) -> Weight {
        Weight::from_ref_time(0)
            // Standard Error: 209_000
            .saturating_add(Weight::from_ref_time(19_239_000).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(s as u64)))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
            .saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(s as u64)))
    }
    // Storage: Scheduler Agenda (r:2 w:2)
    fn on_initialize_periodic(s: u32) -> Weight {
        Weight::from_ref_time(35_677_000)
            // Standard Error: 195_000
            .saturating_add(Weight::from_ref_time(12_177_000).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(s as u64)))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
            .saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
    }
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Scheduler Lookup (r:0 w:1)
    fn on_initialize_named(s: u32) -> Weight {
        Weight::from_ref_time(35_375_000)
            // Standard Error: 61_000
            .saturating_add(Weight::from_ref_time(7_880_000).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
            .saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
    }
    // Storage: Scheduler Agenda (r:1 w:1)
    fn on_initialize(s: u32) -> Weight {
        Weight::from_ref_time(19_047_000)
            // Standard Error: 182_000
            .saturating_add(Weight::from_ref_time(7_076_000).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Scheduler Agenda (r:1 w:1)
    fn schedule(s: u32) -> Weight {
        Weight::from_ref_time(25_088_000)
            // Standard Error: 11_000
            .saturating_add(Weight::from_ref_time(56_000).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Scheduler Lookup (r:0 w:1)
    fn cancel(s: u32) -> Weight {
        Weight::from_ref_time(25_442_000)
            // Standard Error: 16_000
            .saturating_add(Weight::from_ref_time(977_000).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    // Storage: Scheduler Lookup (r:1 w:1)
    // Storage: Scheduler Agenda (r:1 w:1)
    fn schedule_named(s: u32) -> Weight {
        Weight::from_ref_time(31_096_000)
            // Standard Error: 8_000
            .saturating_add(Weight::from_ref_time(65_000).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    // Storage: Scheduler Lookup (r:1 w:1)
    // Storage: Scheduler Agenda (r:1 w:1)
    fn cancel_named(s: u32) -> Weight {
        Weight::from_ref_time(26_010_000)
            // Standard Error: 26_000
            .saturating_add(Weight::from_ref_time(1_119_000).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
}
