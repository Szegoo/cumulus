// Copyright Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_collator_selection`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-05, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm4`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("bridge-hub-rococo-dev"), DB CACHE: 1024

// Executed Command:
// ./artifacts/polkadot-parachain
// benchmark
// pallet
// --chain=bridge-hub-rococo-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_collator_selection
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./parachains/runtimes/bridge-hubs/bridge-hub-rococo/src/weights/pallet_collator_selection.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_collator_selection`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collator_selection::WeightInfo for WeightInfo<T> {
	/// Storage: Session NextKeys (r:100 w:0)
	/// Proof Skipped: Session NextKeys (max_values: None, max_size: None, mode: Measured)
	/// Storage: CollatorSelection Invulnerables (r:0 w:1)
	/// Proof: CollatorSelection Invulnerables (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// The range of component `b` is `[1, 100]`.
	fn set_invulnerables(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `214 + b * (78 ±0)`
		//  Estimated: `1203 + b * (2554 ±0)`
		// Minimum execution time: 15_056_000 picoseconds.
		Weight::from_parts(15_300_844, 0)
			.saturating_add(Weight::from_parts(0, 1203))
			// Standard Error: 2_903
			.saturating_add(Weight::from_parts(2_591_470, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 2554).saturating_mul(b.into()))
	}
	/// Storage: CollatorSelection DesiredCandidates (r:0 w:1)
	/// Proof: CollatorSelection DesiredCandidates (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn set_desired_candidates() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_296_000 picoseconds.
		Weight::from_parts(7_526_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: CollatorSelection CandidacyBond (r:0 w:1)
	/// Proof: CollatorSelection CandidacyBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn set_candidacy_bond() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_483_000 picoseconds.
		Weight::from_parts(7_694_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: CollatorSelection Candidates (r:1 w:1)
	/// Proof: CollatorSelection Candidates (max_values: Some(1), max_size: Some(48002), added: 48497, mode: MaxEncodedLen)
	/// Storage: CollatorSelection DesiredCandidates (r:1 w:0)
	/// Proof: CollatorSelection DesiredCandidates (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: CollatorSelection Invulnerables (r:1 w:0)
	/// Proof: CollatorSelection Invulnerables (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: Session NextKeys (r:1 w:0)
	/// Proof Skipped: Session NextKeys (max_values: None, max_size: None, mode: Measured)
	/// Storage: CollatorSelection CandidacyBond (r:1 w:0)
	/// Proof: CollatorSelection CandidacyBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
	/// Proof: CollatorSelection LastAuthoredBlock (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// The range of component `c` is `[1, 999]`.
	fn register_as_candidate(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1104 + c * (48 ±0)`
		//  Estimated: `49487 + c * (49 ±0)`
		// Minimum execution time: 42_882_000 picoseconds.
		Weight::from_parts(34_865_364, 0)
			.saturating_add(Weight::from_parts(0, 49487))
			// Standard Error: 1_292
			.saturating_add(Weight::from_parts(107_420, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 49).saturating_mul(c.into()))
	}
	/// Storage: CollatorSelection Candidates (r:1 w:1)
	/// Proof: CollatorSelection Candidates (max_values: Some(1), max_size: Some(48002), added: 48497, mode: MaxEncodedLen)
	/// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
	/// Proof: CollatorSelection LastAuthoredBlock (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// The range of component `c` is `[6, 1000]`.
	fn leave_intent(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `428 + c * (48 ±0)`
		//  Estimated: `49487`
		// Minimum execution time: 34_025_000 picoseconds.
		Weight::from_parts(23_170_113, 0)
			.saturating_add(Weight::from_parts(0, 49487))
			// Standard Error: 1_333
			.saturating_add(Weight::from_parts(108_636, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: System BlockWeight (r:1 w:1)
	/// Proof: System BlockWeight (max_values: Some(1), max_size: Some(48), added: 543, mode: MaxEncodedLen)
	/// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
	/// Proof: CollatorSelection LastAuthoredBlock (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	fn note_author() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `155`
		//  Estimated: `6196`
		// Minimum execution time: 45_689_000 picoseconds.
		Weight::from_parts(46_236_000, 0)
			.saturating_add(Weight::from_parts(0, 6196))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: CollatorSelection Invulnerables (r:1 w:1)
	/// Proof: CollatorSelection Invulnerables (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: Session NextKeys (r:1 w:0)
	/// Proof Skipped: Session NextKeys (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[1, 99]`.
	fn add_invulnerable(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `581 + b * (37 ±0)`
		//  Estimated: `4687 + b * (37 ±0)`
		// Minimum execution time: 269_126_000 picoseconds.
		Weight::from_parts(286_711_880, 0)
			.saturating_add(Weight::from_parts(0, 4687))
			// Standard Error: 22_887
			.saturating_add(Weight::from_parts(813_399, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 37).saturating_mul(b.into()))
	}
	/// Storage: CollatorSelection Invulnerables (r:1 w:1)
	/// Proof: CollatorSelection Invulnerables (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// The range of component `b` is `[1, 100]`.
	fn remove_invulnerable(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `119 + b * (32 ±0)`
		//  Estimated: `4687`
		// Minimum execution time: 183_054_000 picoseconds.
		Weight::from_parts(197_205_427, 0)
			.saturating_add(Weight::from_parts(0, 4687))
			// Standard Error: 13_533
			.saturating_add(Weight::from_parts(376_231, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: CollatorSelection Candidates (r:1 w:0)
	/// Proof: CollatorSelection Candidates (max_values: Some(1), max_size: Some(48002), added: 48497, mode: MaxEncodedLen)
	/// Storage: CollatorSelection LastAuthoredBlock (r:999 w:0)
	/// Proof: CollatorSelection LastAuthoredBlock (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// Storage: CollatorSelection Invulnerables (r:1 w:0)
	/// Proof: CollatorSelection Invulnerables (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: System BlockWeight (r:1 w:1)
	/// Proof: System BlockWeight (max_values: Some(1), max_size: Some(48), added: 543, mode: MaxEncodedLen)
	/// Storage: System Account (r:995 w:995)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 1000]`.
	/// The range of component `c` is `[1, 1000]`.
	fn new_session(r: u32, c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `22815 + c * (97 ±0) + r * (116 ±0)`
		//  Estimated: `49487 + c * (2519 ±0) + r * (2602 ±0)`
		// Minimum execution time: 16_853_000 picoseconds.
		Weight::from_parts(16_995_000, 0)
			.saturating_add(Weight::from_parts(0, 49487))
			// Standard Error: 900_191
			.saturating_add(Weight::from_parts(31_884_166, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2519).saturating_mul(c.into()))
			.saturating_add(Weight::from_parts(0, 2602).saturating_mul(r.into()))
	}
}
