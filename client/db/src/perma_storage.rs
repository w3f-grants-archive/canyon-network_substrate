//! RocksDB-based offchain workers local storage.

use std::{collections::HashMap, sync::Arc};

use crate::{columns, Database, DbHash, Transaction};
use parking_lot::Mutex;
use log::error;

/// Offchain local storage
#[derive(Clone)]
pub struct DataStorage {
	db: Arc<dyn Database<DbHash>>,
}

impl std::fmt::Debug for DataStorage {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
		fmt.debug_struct("DataStorage")
			.finish()
	}
}

impl DataStorage {
	/// Create new offchain storage for tests (backed by memorydb)
	#[cfg(any(feature = "test-helpers", test))]
	pub fn new_test() -> Self {
		let db = kvdb_memorydb::create(crate::utils::NUM_COLUMNS);
		let db = sp_database::as_database(db);
		Self::new(db as _)
	}

	/// Create offchain local storage with given `KeyValueDB` backend.
	pub fn new(db: Arc<dyn Database<DbHash>>) -> Self {
		Self {
			db,
		}
	}
}

impl sp_core::perma_storage::PermaStorage for DataStorage {
	fn set(&mut self, key: &[u8], value: &[u8]) {
		let mut tx = Transaction::new();
		tx.set(columns::PERMA_STORAGE, key, value);

		if let Err(err) = self.db.commit(tx) {
			error!("Error setting on perma storage: {}", err)
		}
	}

	fn remove(&mut self, key: &[u8]) {
		let mut tx = Transaction::new();
		tx.remove(columns::PERMA_STORAGE, key);

		if let Err(err) = self.db.commit(tx) {
			error!("Error removing on perma storage: {}", err)
		}
	}

	fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
		self.db.get(columns::PERMA_STORAGE, key)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use sp_core::offchain::OffchainStorage;

	#[test]
	fn perma_storage_should_work() {
		let mut storage = DataStorage::new_test();
		let key = b"key";
		let value = b"value";

		storage.set(key, value);
		assert_eq!(storage.get(key), Some(value.to_vec()));
	}
}
