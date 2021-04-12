//! Offchain RPC errors.

use jsonrpc_core as rpc;

/// Offchain RPC Result type.
pub type Result<T> = std::result::Result<T, Error>;

/// Offchain RPC errors.
#[derive(Debug, derive_more::Display, derive_more::From)]
pub enum Error {
	/// Call to an unsafe RPC was denied.
	UnsafeRpcCalled(crate::policy::UnsafeRpcError),
}

impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			Self::UnsafeRpcCalled(err) => Some(err),
		}
	}
}

/// Base error code for all offchain errors.
const BASE_ERROR: i64 = 5000;

impl From<Error> for rpc::Error {
	fn from(e: Error) -> Self {
		match e {
			Error::UnsafeRpcCalled(e) => e.into(),
		}
	}
}
