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

//! Autogenerated weights for pallet_author_mapping
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
// --pallet=pallet_author_mapping
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

/// Weight functions for pallet_author_mapping (automatically generated)
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_author_mapping::weights::WeightInfo for WeightInfo<T> {
    // Storage: AuthorMapping MappingWithDeposit (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: AuthorMapping NimbusLookup (r:0 w:1)
    fn add_association() -> Weight {
        Weight::from_ref_time(48_320_000)
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: AuthorMapping MappingWithDeposit (r:2 w:2)
    // Storage: AuthorMapping NimbusLookup (r:0 w:1)
    fn update_association() -> Weight {
        Weight::from_ref_time(39_490_000)
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: AuthorMapping MappingWithDeposit (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: AuthorMapping NimbusLookup (r:0 w:1)
    fn clear_association() -> Weight {
        Weight::from_ref_time(50_410_000)
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: AuthorMapping NimbusLookup (r:1 w:1)
    // Storage: AuthorMapping MappingWithDeposit (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    fn remove_keys() -> Weight {
        Weight::from_ref_time(48_640_000)
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: AuthorMapping NimbusLookup (r:1 w:1)
    // Storage: AuthorMapping MappingWithDeposit (r:2 w:2)
    fn set_keys() -> Weight {
        Weight::from_ref_time(47_160_000)
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
}
