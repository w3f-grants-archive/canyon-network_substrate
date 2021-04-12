//! Perma storage API.

pub mod error;

use jsonrpc_derive::rpc;
use self::error::Result;
use sp_core::Bytes;

pub use self::gen_client::Client as DataStorageClient;

/// Substrate offchain RPC API
#[rpc]
pub trait DataStorageApi {
	/// Set offchain local storage under given key and prefix.
	#[rpc(name = "datastorage_Set")]
	fn set_data_storage(&self, key: Bytes, value: Bytes) -> Result<()>;

	/// Get offchain local storage under given key and prefix.
	#[rpc(name = "datastorage_Get")]
	fn get_data_storage(&self, key: Bytes) -> Result<Option<Bytes>>;
}
