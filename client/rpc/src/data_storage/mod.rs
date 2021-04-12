// This file is part of Substrate.

// Copyright (C) 2020-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Substrate offchain API.

#[cfg(test)]
mod tests;

/// Re-export the API for backward compatibility.
pub use sc_rpc_api::data_storage::*;
use sc_rpc_api::DenyUnsafe;
use self::error::{Error, Result};
use sp_core::{
	Bytes,
	perma_storage::PermaStorage,
};
use parking_lot::RwLock;
use std::sync::Arc;

/// Offchain API
#[derive(Debug)]
pub struct DataStorage<T: PermaStorage> {
	/// Offchain storage
	storage: Arc<RwLock<T>>,
	deny_unsafe: DenyUnsafe,
}

impl<T: PermaStorage> DataStorage<T> {
	/// Create new instance of Offchain API.
	pub fn new(storage: T, deny_unsafe: DenyUnsafe) -> Self {
		Self {
			storage: Arc::new(RwLock::new(storage)),
			deny_unsafe,
		}
	}
}

impl<T: PermaStorage + 'static> DataStorageApi for DataStorage<T> {
	/// Set offchain local storage under given key and prefix.
	fn set_data_storage(&self, key: Bytes, value: Bytes) -> Result<()> {
		self.deny_unsafe.check_if_safe()?;

		self.storage.write().set(&*key, &*value);
		Ok(())
	}

	/// Get offchain local storage under given key and prefix.
	fn get_data_storage(&self, key: Bytes) -> Result<Option<Bytes>> {
		self.deny_unsafe.check_if_safe()?;

		Ok(self.storage.read().get(&*key).map(Into::into))
	}
}
