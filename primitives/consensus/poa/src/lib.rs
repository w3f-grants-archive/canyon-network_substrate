//! Primitives for Canyon Proof-of-Access (PoA) consensus.

#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::vec::Vec;
use sp_runtime::ConsensusEngineId;
use codec::Decode;

/// The `ConsensusEngineId` of PoA.
pub const POA_ENGINE_ID: ConsensusEngineId = [b'p', b'o', b'a', b'_'];

/// Type of seal.
pub type Seal = Vec<u8>;

// TODO: poa runtime api.
sp_api::decl_runtime_apis! {
	/// API necessary for timestamp-based difficulty adjustment algorithms.
	pub trait TimestampApi<Moment: Decode> {
		/// Return the timestamp in the current block.
		fn timestamp() -> Moment;
	}

	/// API for those chains that put their difficulty adjustment algorithm directly
	/// onto runtime. Note that while putting difficulty adjustment algorithm to
	/// runtime is safe, putting the PoW algorithm on runtime is not.
	pub trait DifficultyApi<Difficulty: Decode> {
		/// Return the target difficulty of the next block.
		fn difficulty() -> Difficulty;
	}
}
