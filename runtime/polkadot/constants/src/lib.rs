// Copyright 2019-2020 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]

pub mod weights;

// pub use self::currency::DOLLARS;

/// Money matters.
pub mod currency {
	use primitives::v2::Balance;

	/// The existential deposit.
	pub const EXISTENTIAL_DEPOSIT: Balance = 100 * CENTS;

	pub const UNITS: Balance = 1_000_000_000_000;
	pub const MILLICENTS: Balance = 1_000_000_000;
	pub const CENTS: Balance = 1_000 * MILLICENTS; // assume this is worth about a cent.
	pub const DOLLARS: Balance = 100 * CENTS;

	pub const fn deposit(items: u32, bytes: u32) -> Balance {
		items as Balance * 15 * CENTS + (bytes as Balance) * 6 * CENTS
	}
}

#[allow(dead_code)]
/// Time and blocks.
pub mod time_dev {
	use primitives::v2::{BlockNumber, Moment};
	
	pub const MILLISECS_PER_BLOCK: Moment = 6000;
	pub const SECS_PER_BLOCK: Moment = MILLISECS_PER_BLOCK / 1000;
	pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

	// These time units are defined in number of blocks.
	pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
	pub const HOURS: BlockNumber = MINUTES * 60;
	pub const DAYS: BlockNumber = HOURS * 24;
	pub const WEEKS: BlockNumber = DAYS * 7;

	// 1 in 4 blocks (on average, not counting collisions) will be primary babe blocks.
	pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

	pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 2 * MINUTES;
	pub const EPOCH_DURATION_IN_SLOTS: BlockNumber = EPOCH_DURATION_IN_BLOCKS;
	pub const SESSIONS_PER_ERA: sp_staking::SessionIndex = 3;
	pub const BONDING_DURATION: sp_staking::EraIndex = 24 * 8;
	pub const SLASH_DEFER_DURATION: sp_staking::EraIndex = 24 * 2;
	pub const REPORT_LONGEVITY: u64 =
		BONDING_DURATION as u64 * SESSIONS_PER_ERA as u64 * EPOCH_DURATION_IN_SLOTS as u64;

	pub const TERM_DURATION: BlockNumber = 2 * MINUTES;
	pub const COUNCIL_MOTION_DURATION: BlockNumber = 10 * MINUTES;
	pub const TECHNICAL_MOTION_DURATION: BlockNumber = 10 * MINUTES;
	pub const ENACTMENT_PERIOD: BlockNumber = 1 * MINUTES;
	pub const LAUNCH_PERIOD: BlockNumber = 15 * MINUTES;
	pub const VOTING_PERIOD: BlockNumber = 10 * MINUTES;
	pub const FAST_TRACK_VOTING_PERIOD: BlockNumber = 2 * MINUTES;
	pub const COOLOFF_PERIOD: BlockNumber = 60 * MINUTES;

	pub const ALLOWED_PROPOSAL_PERIOD: BlockNumber = 14;
	pub const SPEND_PERIOD: BlockNumber = 2 * MINUTES;
	pub const BOUNTY_DEPOSIT_PAYOUT_DELAY: BlockNumber = 1 * MINUTES;
	pub const TIP_COUNTDOWN: BlockNumber = 1 * MINUTES;
	pub const BOUNTY_UPDATE_PERIOD: BlockNumber = 5 * MINUTES;
}

#[allow(dead_code)]
/// Time and blocks.
pub mod time_prod {
	use primitives::v2::{BlockNumber, Moment};

	pub const MILLISECS_PER_BLOCK: Moment = 6000;
	pub const SECS_PER_BLOCK: Moment = MILLISECS_PER_BLOCK / 1000;
	pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

	// These time units are defined in number of blocks.
	pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
	pub const HOURS: BlockNumber = MINUTES * 60;
	pub const DAYS: BlockNumber = HOURS * 24;
	pub const WEEKS: BlockNumber = DAYS * 7;

	// 1 in 4 blocks (on average, not counting collisions) will be primary babe blocks.
	pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

	pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 10 * MINUTES;
	pub const EPOCH_DURATION_IN_SLOTS: BlockNumber = EPOCH_DURATION_IN_BLOCKS;

	// NOTE: Currently it is not possible to change the epoch duration after the chain has started.
	//       Attempting to do so will brick block production.
	pub const SESSIONS_PER_ERA: sp_staking::SessionIndex = 6;
	pub const BONDING_DURATION: sp_staking::EraIndex = 24 * 28;
	pub const SLASH_DEFER_DURATION: sp_staking::EraIndex = 24 * 27;
	pub const REPORT_LONGEVITY: u64 =
		BONDING_DURATION as u64 * SESSIONS_PER_ERA as u64 * EPOCH_DURATION_IN_SLOTS as u64;

	pub const TERM_DURATION: BlockNumber = 7 * DAYS;
	pub const COUNCIL_MOTION_DURATION: BlockNumber = 5 * DAYS;
	pub const TECHNICAL_MOTION_DURATION: BlockNumber = 5 * DAYS;
	pub const ENACTMENT_PERIOD: BlockNumber = 30 * 24 * 60 * MINUTES;
	pub const LAUNCH_PERIOD: BlockNumber = 28 * 24 * 60 * MINUTES;
	pub const VOTING_PERIOD: BlockNumber = 28 * 24 * 60 * MINUTES;
	pub const FAST_TRACK_VOTING_PERIOD: BlockNumber = 3 * 24 * 60 * MINUTES;
	pub const COOLOFF_PERIOD: BlockNumber = 28 * 24 * 60 * MINUTES;

	pub const ALLOWED_PROPOSAL_PERIOD: BlockNumber = 24 * DAYS;
	pub const SPEND_PERIOD: BlockNumber = 28 * DAYS;
	pub const BOUNTY_DEPOSIT_PAYOUT_DELAY: BlockNumber = 1 * DAYS;
	pub const TIP_COUNTDOWN: BlockNumber = 1 * DAYS;
	pub const BOUNTY_UPDATE_PERIOD: BlockNumber = 14 * DAYS;
}

/// Fee-related.
pub mod fee {
	use crate::weights::ExtrinsicBaseWeight;
	use frame_support::weights::{
		WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial,
	};
	use primitives::v2::Balance;
	use smallvec::smallvec;
	pub use sp_runtime::Perbill;

	/// The block saturation level. Fees will be updates based on this value.
	pub const TARGET_BLOCK_FULLNESS: Perbill = Perbill::from_percent(25);

	/// Handles converting a weight scalar to a fee value, based on the scale and granularity of the
	/// node's balance type.
	///
	/// This should typically create a mapping between the following ranges:
	///   - [0, `MAXIMUM_BLOCK_WEIGHT`]
	///   - [Balance::min, Balance::max]
	///
	/// Yet, it can be used for any other sort of change to weight-fee. Some examples being:
	///   - Setting it to `0` will essentially disable the weight fee.
	///   - Setting it to `1` will cause the literal `#[weight = x]` values to be charged.
	pub struct WeightToFee;
	impl WeightToFeePolynomial for WeightToFee {
		type Balance = Balance;
		fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
			// in Polkadot, extrinsic base weight (smallest non-zero weight) is mapped to 1/10 CENT:
			let p = super::currency::CENTS;
			let q = 10 * Balance::from(ExtrinsicBaseWeight::get());
			smallvec![WeightToFeeCoefficient {
				degree: 1,
				negative: false,
				coeff_frac: Perbill::from_rational(p % q, q),
				coeff_integer: p / q,
			}]
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{
		currency::{CENTS, DOLLARS, MILLICENTS},
		fee::WeightToFee,
	};
	use crate::weights::ExtrinsicBaseWeight;
	use frame_support::weights::WeightToFee as WeightToFeeT;
	use runtime_common::MAXIMUM_BLOCK_WEIGHT;

	#[test]
	// Test that the fee for `MAXIMUM_BLOCK_WEIGHT` of weight has sane bounds.
	fn full_block_fee_is_correct() {
		// A full block should cost between 10 and 100 DOLLARS.
		let full_block = WeightToFee::weight_to_fee(&MAXIMUM_BLOCK_WEIGHT);
		assert!(full_block >= 10 * DOLLARS);
		assert!(full_block <= 100 * DOLLARS);
	}

	#[test]
	// This function tests that the fee for `ExtrinsicBaseWeight` of weight is correct
	fn extrinsic_base_fee_is_correct() {
		// `ExtrinsicBaseWeight` should cost 1/10 of a CENT
		println!("Base: {}", ExtrinsicBaseWeight::get());
		let x = WeightToFee::weight_to_fee(&ExtrinsicBaseWeight::get());
		let y = CENTS / 10;
		assert!(x.max(y) - x.min(y) < MILLICENTS);
	}
}
