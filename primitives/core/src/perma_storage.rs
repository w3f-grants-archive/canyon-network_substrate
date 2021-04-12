//! Permanent storage types

#[cfg(feature = "std")]
use std::collections::hash_map::HashMap;

use codec::{Encode, Decode};
use sp_std::{prelude::{Vec, Box}, convert::TryFrom};
use crate::{OpaquePeerId, RuntimeDebug};
use sp_runtime_interface::pass_by::{PassByCodec, PassByInner, PassByEnum};

/// Persistent storage prefix used by the Offchain Worker API when creating a DB key.
pub const STORAGE_PREFIX : &[u8] = b"storage";

/// Permanent storage.
pub trait PermaStorage: Clone + Send + Sync {
	/// Persist a value in storage under given key and prefix.
	fn set(&mut self, key: &[u8], value: &[u8]);

	/// Clear a storage entry under given key and prefix.
	fn remove(&mut self, key: &[u8]);

	/// Retrieve a value from storage under given key and prefix.
	fn get(&self, key: &[u8]) -> Option<Vec<u8>>;
}

/// In-memory storage for offchain workers.
#[derive(Debug, Clone, Default)]
#[cfg(feature = "std")]
pub struct InMemoryPermaStorage {
	storage: HashMap<Vec<u8>, Vec<u8>>,
}

#[cfg(feature = "std")]
impl InMemoryPermaStorage {
	/// Consume the offchain storage and iterate over all key value pairs.
	pub fn into_iter(self) -> impl Iterator<Item=(Vec<u8>,Vec<u8>)> {
		self.storage.into_iter()
	}

	/// Iterate over all key value pairs by reference.
	pub fn iter<'a>(&'a self) -> impl Iterator<Item=(&'a Vec<u8>,&'a Vec<u8>)> {
		self.storage.iter()
	}

	/// Remove a key and its associated value from the offchain database.
	pub fn remove(&mut self, key: &[u8]) {
		self.storage.remove(key);
	}
}

#[cfg(feature = "std")]
impl PermaStorage for InMemoryPermaStorage {
	fn set(&mut self, key: &[u8], value: &[u8]) {
		self.storage.insert(key.to_vec(), value.to_vec());
	}

	fn remove(&mut self, key: &[u8]) {
		self.storage.remove(key);
	}

	fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
		self.storage.get(key).cloned()
	}
}
