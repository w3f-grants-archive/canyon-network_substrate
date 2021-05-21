//! Generic implementation of an unchecked (pre-verification) extrinsic.

use codec::{Decode, Encode};

use sp_std::prelude::*;
use sp_core::RuntimeDebug;

use crate::{
	traits::{
		BlakeTwo256, Hash as HashT,
	},
};

/// 256KiB per chunk.
const CHUNK_SIZE: usize = 256 * 1024;

/// Maximum bytes of data payload is 10MiB.
const MAXIMUM_DATA_PAYLOAD: u32 = 10 * 1024 * 1024;

/// State info for a stored transaction data.
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub struct DataInfo<Hash: HashT> {
	/// Number of data in bytes.
	pub size: u64,
	/// Trie root of data in chunks.
	pub data_root: Hash::Output,
}

/// Unit type wrapper of raw data bytes.
///
/// The maximum payload of data is 10 MiB(10 * 1024 * 1024 bytes).
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub struct DataPayload(Vec<u8>);

impl From<Vec<u8>> for DataPayload {
	fn from(inner: Vec<u8>) -> Self {
		assert!(MAXIMUM_DATA_PAYLOAD as usize > inner.len());
		Self(inner)
	}
}

/// Type that represents the data of transaction.
///
/// It can have an optional payload which is no more than 10MiB.
///
/// TODO: Impl Codec manually, the payload should be skipped.
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub struct TransactionData<Hash: HashT> {
	/// Optional raw data in bytes.
	pub payload: Option<DataPayload>,
	/// Data info.
	pub info: DataInfo<Hash>,
}

impl<Hash: HashT> From<Vec<u8>> for TransactionData<Hash> {
	fn from(raw_bytes: Vec<u8>) -> Self {
		let chunks = raw_bytes.chunks(CHUNK_SIZE).map(|c| c.to_vec()).collect();
		let data_root = Hash::ordered_trie_root(chunks);
		let size = raw_bytes.len() as u64;
		let payload = Some(raw_bytes.into());
		Self {
			payload,
			info: DataInfo {
				size,
				data_root,
			}
		}
	}
}

/// `TransactionData` with concrete `BlakeTwo256` hasher.
pub type Data = TransactionData<BlakeTwo256>;
