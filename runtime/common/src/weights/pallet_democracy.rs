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

//! Autogenerated weights for pallet_democracy
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
// --pallet=pallet_democracy
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

/// Weight functions for pallet_democracy (automatically generated)
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_democracy::weights::WeightInfo for WeightInfo<T> {
    // Storage: Democracy PublicPropCount (r:1 w:1)
    // Storage: Democracy PublicProps (r:1 w:1)
    // Storage: Democracy Blacklist (r:1 w:0)
    // Storage: Democracy DepositOf (r:0 w:1)
    fn propose() -> Weight {
        Weight::from_ref_time(68_491_000)
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: Democracy DepositOf (r:1 w:1)
    fn second(s: u32) -> Weight {
        Weight::from_ref_time(42_396_000)
            // Standard Error: 5_000
            .saturating_add(Weight::from_ref_time(144_000).saturating_mul(s as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Democracy ReferendumInfoOf (r:1 w:1)
    // Storage: Democracy VotingOf (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    fn vote_new(r: u32) -> Weight {
        Weight::from_ref_time(53_867_000)
            // Standard Error: 7_000
            .saturating_add(Weight::from_ref_time(200_000).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: Democracy ReferendumInfoOf (r:1 w:1)
    // Storage: Democracy VotingOf (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    fn vote_existing(r: u32) -> Weight {
        Weight::from_ref_time(56_948_000)
            // Standard Error: 9_000
            .saturating_add(Weight::from_ref_time(145_000).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: Democracy ReferendumInfoOf (r:1 w:1)
    // Storage: Democracy Cancellations (r:1 w:1)
    fn emergency_cancel() -> Weight {
        Weight::from_ref_time(25_531_000)
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    // Storage: Democracy PublicProps (r:1 w:1)
    // Storage: Democracy NextExternal (r:1 w:1)
    // Storage: Democracy ReferendumInfoOf (r:1 w:1)
    // Storage: Democracy Blacklist (r:0 w:1)
    // Storage: Democracy DepositOf (r:1 w:1)
    // Storage: System Account (r:2 w:2)
    fn blacklist(p: u32) -> Weight {
        Weight::from_ref_time(82_409_000)
            // Standard Error: 9_000
            .saturating_add(Weight::from_ref_time(277_000).saturating_mul(p as u64))
            .saturating_add(T::DbWeight::get().reads(6 as u64))
            .saturating_add(T::DbWeight::get().writes(7 as u64))
    }
    // Storage: Democracy NextExternal (r:1 w:1)
    // Storage: Democracy Blacklist (r:1 w:0)
    fn external_propose(v: u32) -> Weight {
        Weight::from_ref_time(14_556_000)
            // Standard Error: 1_000
            .saturating_add(Weight::from_ref_time(37_000).saturating_mul(v as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Democracy NextExternal (r:0 w:1)
    fn external_propose_majority() -> Weight {
        Weight::from_ref_time(1_830_000).saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Democracy NextExternal (r:0 w:1)
    fn external_propose_default() -> Weight {
        Weight::from_ref_time(1_910_000).saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Democracy NextExternal (r:1 w:1)
    // Storage: Democracy ReferendumCount (r:1 w:1)
    // Storage: Democracy ReferendumInfoOf (r:0 w:1)
    fn fast_track() -> Weight {
        Weight::from_ref_time(27_890_000)
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: Democracy NextExternal (r:1 w:1)
    // Storage: Democracy Blacklist (r:1 w:1)
    fn veto_external(v: u32) -> Weight {
        Weight::from_ref_time(31_540_000)
            // Standard Error: 7_000
            .saturating_add(Weight::from_ref_time(54_000).saturating_mul(v as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    // Storage: Democracy PublicProps (r:1 w:1)
    // Storage: Democracy DepositOf (r:1 w:1)
    // Storage: System Account (r:2 w:2)
    fn cancel_proposal(p: u32) -> Weight {
        Weight::from_ref_time(66_972_000)
            // Standard Error: 11_000
            .saturating_add(Weight::from_ref_time(249_000).saturating_mul(p as u64))
            .saturating_add(T::DbWeight::get().reads(4 as u64))
            .saturating_add(T::DbWeight::get().writes(4 as u64))
    }
    // Storage: Democracy ReferendumInfoOf (r:0 w:1)
    fn cancel_referendum() -> Weight {
        Weight::from_ref_time(16_700_000).saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Scheduler Lookup (r:1 w:1)
    // Storage: Scheduler Agenda (r:1 w:1)
    fn cancel_queued(r: u32) -> Weight {
        Weight::from_ref_time(36_296_000)
            // Standard Error: 10_000
            .saturating_add(Weight::from_ref_time(894_000).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    // Storage: Democracy LowestUnbaked (r:1 w:1)
    // Storage: Democracy ReferendumCount (r:1 w:0)
    // Storage: Democracy ReferendumInfoOf (r:1 w:0)
    fn on_initialize_base(r: u32) -> Weight {
        Weight::from_ref_time(13_953_000)
            // Standard Error: 37_000
            .saturating_add(Weight::from_ref_time(5_806_000).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Democracy LowestUnbaked (r:1 w:1)
    // Storage: Democracy ReferendumCount (r:1 w:0)
    // Storage: Democracy LastTabledWasExternal (r:1 w:0)
    // Storage: Democracy NextExternal (r:1 w:0)
    // Storage: Democracy PublicProps (r:1 w:0)
    // Storage: Democracy ReferendumInfoOf (r:1 w:0)
    fn on_initialize_base_with_launch_period(r: u32) -> Weight {
        Weight::from_ref_time(29_984_000)
            // Standard Error: 46_000
            .saturating_add(Weight::from_ref_time(5_446_000).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(5 as u64))
            .saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Democracy VotingOf (r:3 w:3)
    // Storage: Democracy ReferendumInfoOf (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    fn delegate(r: u32) -> Weight {
        Weight::from_ref_time(68_679_000)
            // Standard Error: 54_000
            .saturating_add(Weight::from_ref_time(7_169_000).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(4 as u64))
            .saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
            .saturating_add(T::DbWeight::get().writes(4 as u64))
            .saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(r as u64)))
    }
    // Storage: Democracy VotingOf (r:2 w:2)
    // Storage: Democracy ReferendumInfoOf (r:1 w:1)
    fn undelegate(r: u32) -> Weight {
        Weight::from_ref_time(32_517_000)
            // Standard Error: 68_000
            .saturating_add(Weight::from_ref_time(7_631_000).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
            .saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(r as u64)))
    }
    // Storage: Democracy PublicProps (r:0 w:1)
    fn clear_public_proposals() -> Weight {
        Weight::from_ref_time(2_330_000).saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Democracy Preimages (r:1 w:1)
    fn note_preimage(b: u32) -> Weight {
        Weight::from_ref_time(38_550_000)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(3_000).saturating_mul(b as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Democracy Preimages (r:1 w:1)
    fn note_imminent_preimage(b: u32) -> Weight {
        Weight::from_ref_time(30_310_000)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(2_000).saturating_mul(b as u64))
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Democracy Preimages (r:1 w:1)
    // Storage: System Account (r:1 w:0)
    fn reap_preimage(b: u32) -> Weight {
        Weight::from_ref_time(38_453_000)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(2_000).saturating_mul(b as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Democracy VotingOf (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    fn unlock_remove(r: u32) -> Weight {
        Weight::from_ref_time(34_805_000)
            // Standard Error: 4_000
            .saturating_add(Weight::from_ref_time(35_000).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: Democracy VotingOf (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    fn unlock_set(r: u32) -> Weight {
        Weight::from_ref_time(32_006_000)
            // Standard Error: 4_000
            .saturating_add(Weight::from_ref_time(203_000).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    // Storage: Democracy ReferendumInfoOf (r:1 w:1)
    // Storage: Democracy VotingOf (r:1 w:1)
    fn remove_vote(r: u32) -> Weight {
        Weight::from_ref_time(21_521_000)
            // Standard Error: 4_000
            .saturating_add(Weight::from_ref_time(141_000).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    // Storage: Democracy ReferendumInfoOf (r:1 w:1)
    // Storage: Democracy VotingOf (r:1 w:1)
    fn remove_other_vote(r: u32) -> Weight {
        Weight::from_ref_time(21_385_000)
            // Standard Error: 3_000
            .saturating_add(Weight::from_ref_time(132_000).saturating_mul(r as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
}
