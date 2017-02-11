// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! RPC Error codes and error objects

macro_rules! rpc_unimplemented {
	() => (Err(::v1::helpers::errors::unimplemented(None)))
}

use std::fmt;
use rlp::DecoderError;
use ethcore::error::{Error as EthcoreError, CallError, TransactionError};
use ethcore::account_provider::{SignError as AccountError};
use jsonrpc_core::{Error, ErrorCode, Value};

mod codes {
	// NOTE [ToDr] Codes from [-32099, -32000]
	pub const UNSUPPORTED_REQUEST: i64 = -32000;
	pub const NO_WORK: i64 = -32001;
	pub const NO_AUTHOR: i64 = -32002;
	pub const NO_NEW_WORK: i64 = -32003;
	pub const NOT_ENOUGH_DATA: i64 = -32006;
	pub const UNKNOWN_ERROR: i64 = -32009;
	pub const TRANSACTION_ERROR: i64 = -32010;
	pub const EXECUTION_ERROR: i64 = -32015;
	pub const EXCEPTION_ERROR: i64 = -32016;
	pub const ACCOUNT_LOCKED: i64 = -32020;
	pub const PASSWORD_INVALID: i64 = -32021;
	pub const ACCOUNT_ERROR: i64 = -32023;
	pub const SIGNER_DISABLED: i64 = -32030;
	pub const DAPPS_DISABLED: i64 = -32031;
	pub const NETWORK_DISABLED: i64 = -32035;
	pub const REQUEST_REJECTED: i64 = -32040;
	pub const REQUEST_REJECTED_LIMIT: i64 = -32041;
	pub const REQUEST_NOT_FOUND: i64 = -32042;
	pub const COMPILATION_ERROR: i64 = -32050;
	pub const ENCRYPTION_ERROR: i64 = -32055;
	pub const FETCH_ERROR: i64 = -32060;
	pub const NO_FILTER_ERROR: i64 = -32065;
}

pub fn unimplemented(details: Option<String>) -> Error {
	Error {
		code: ErrorCode::ServerError(codes::UNSUPPORTED_REQUEST),
		message: "This request is not implemented yet. Please create an issue on Github repo.".into(),
		data: details.map(Value::String),
	}
}

pub fn request_not_found() -> Error {
	Error {
		code: ErrorCode::ServerError(codes::REQUEST_NOT_FOUND),
		message: "Request not found.".into(),
		data: None,
	}
}

pub fn request_rejected() -> Error {
	Error {
		code: ErrorCode::ServerError(codes::REQUEST_REJECTED),
		message: "Request has been rejected.".into(),
		data: None,
	}
}

pub fn request_rejected_limit() -> Error {
	Error {
		code: ErrorCode::ServerError(codes::REQUEST_REJECTED_LIMIT),
		message: "Request has been rejected because of queue limit.".into(),
		data: None,
	}
}

pub fn account<T: fmt::Debug>(error: &str, details: T) -> Error {
	Error {
		code: ErrorCode::ServerError(codes::ACCOUNT_ERROR),
		message: error.into(),
		data: Some(Value::String(format!("{:?}", details))),
	}
}

pub fn compilation<T: fmt::Debug>(error: T) -> Error {
	Error {
		code: ErrorCode::ServerError(codes::COMPILATION_ERROR),
		message: "Error while compiling code.".into(),
		data: Some(Value::String(format!("{:?}", error))),
	}
}

pub fn internal<T: fmt::Debug>(error: &str, data: T) -> Error {
	Error {
		code: ErrorCode::InternalError,
		message: format!("Internal error occurred: {}", error),
		data: Some(Value::String(format!("{:?}", data))),
	}
}

pub fn invalid_params<T: fmt::Debug>(param: &str, details: T) -> Error {
	Error {
		code: ErrorCode::InvalidParams,
		message: format!("Couldn't parse parameters: {}", param),
		data: Some(Value::String(format!("{:?}", details))),
	}
}

pub fn execution<T: fmt::Debug>(data: T) -> Error {
	Error {
		code: ErrorCode::ServerError(codes::EXECUTION_ERROR),
		message: "Transaction execution error.".into(),
		data: Some(Value::String(format!("{:?}", data))),
	}
}

pub fn state_pruned() -> Error {
	Error {
		code: ErrorCode::ServerError(codes::UNSUPPORTED_REQUEST),
		message: "This request is not supported because your node is running with state pruning. Run with --pruning=archive.".into(),
		data: None
	}
}

pub fn exceptional() -> Error {
	Error {
		code: ErrorCode::ServerError(codes::EXCEPTION_ERROR),
		message: "The execution failed due to an exception.".into(),
		data: None
	}
}

pub fn no_work() -> Error {
	Error {
		code: ErrorCode::ServerError(codes::NO_WORK),
		message: "Still syncing.".into(),
		data: None
	}
}

pub fn no_new_work() -> Error {
	Error {
		code: ErrorCode::ServerError(codes::NO_NEW_WORK),
		message: "Work has not changed.".into(),
		data: None
	}
}

pub fn no_author() -> Error {
	Error {
		code: ErrorCode::ServerError(codes::NO_AUTHOR),
		message: "Author not configured. Run Parity with --author to configure.".into(),
		data: None
	}
}

pub fn not_enough_data() -> Error {
	Error {
		code: ErrorCode::ServerError(codes::NOT_ENOUGH_DATA),
		message: "The node does not have enough data to compute the given statistic.".into(),
		data: None
	}
}

pub fn token(e: String) -> Error {
	Error {
		code: ErrorCode::ServerError(codes::UNKNOWN_ERROR),
		message: "There was an error when saving your authorization tokens.".into(),
		data: Some(Value::String(e)),
	}
}

pub fn signer_disabled() -> Error {
	Error {
		code: ErrorCode::ServerError(codes::SIGNER_DISABLED),
		message: "Trusted Signer is disabled. This API is not available.".into(),
		data: None
	}
}

pub fn dapps_disabled() -> Error {
	Error {
		code: ErrorCode::ServerError(codes::DAPPS_DISABLED),
		message: "Dapps Server is disabled. This API is not available.".into(),
		data: None
	}
}

pub fn network_disabled() -> Error {
	Error {
		code: ErrorCode::ServerError(codes::NETWORK_DISABLED),
		message: "Network is disabled or not yet up.".into(),
		data: None
	}
}

pub fn encryption_error<T: fmt::Debug>(error: T) -> Error {
	Error {
		code: ErrorCode::ServerError(codes::ENCRYPTION_ERROR),
		message: "Encryption error.".into(),
		data: Some(Value::String(format!("{:?}", error))),
	}
}

pub fn from_fetch_error<T: fmt::Debug>(error: T) -> Error {
	Error {
		code: ErrorCode::ServerError(codes::FETCH_ERROR),
		message: "Error while fetching content.".into(),
		data: Some(Value::String(format!("{:?}", error))),
	}
}

pub fn from_signing_error(error: AccountError) -> Error {
	Error {
		code: ErrorCode::ServerError(codes::ACCOUNT_LOCKED),
		message: "Your account is locked. Unlock the account via CLI, personal_unlockAccount or use Trusted Signer.".into(),
		data: Some(Value::String(format!("{:?}", error))),
	}
}

pub fn from_password_error(error: AccountError) -> Error {
	Error {
		code: ErrorCode::ServerError(codes::PASSWORD_INVALID),
		message: "Account password is invalid or account does not exist.".into(),
		data: Some(Value::String(format!("{:?}", error))),
	}
}

pub fn transaction_message(error: TransactionError) -> String {
	use ethcore::error::TransactionError::*;

	match error {
		AlreadyImported => "Transaction with the same hash was already imported.".into(),
		Old => "Transaction nonce is too low. Try incrementing the nonce.".into(),
		TooCheapToReplace => {
			"Transaction gas price is too low. There is another transaction with same nonce in the queue. Try increasing the gas price or incrementing the nonce.".into()
		},
		LimitReached => {
			"There are too many transactions in the queue. Your transaction was dropped due to limit. Try increasing the fee.".into()
		},
		InsufficientGas { minimal, got } => {
			format!("Transaction gas is too low. There is not enough gas to cover minimal cost of the transaction (minimal: {}, got: {}). Try increasing supplied gas.", minimal, got)
		},
		InsufficientGasPrice { minimal, got } => {
			format!("Transaction gas price is too low. It does not satisfy your node's minimal gas price (minimal: {}, got: {}). Try increasing the gas price.", minimal, got)
		},
		InsufficientBalance { balance, cost } => {
			format!("Insufficient funds. The account you tried to send transaction from does not have enough funds. Required {} and got: {}.", cost, balance)
		},
		GasLimitExceeded { limit, got } => {
			format!("Transaction cost exceeds current gas limit. Limit: {}, got: {}. Try decreasing supplied gas.", limit, got)
		},
		InvalidNetworkId => "Invalid network id.".into(),
		InvalidGasLimit(_) => "Supplied gas is beyond limit.".into(),
		SenderBanned => "Sender is banned in local queue.".into(),
		RecipientBanned => "Recipient is banned in local queue.".into(),
		CodeBanned => "Code is banned in local queue.".into(),
	}
}

pub fn from_transaction_error(error: EthcoreError) -> Error {

	if let EthcoreError::Transaction(e) = error {
		Error {
			code: ErrorCode::ServerError(codes::TRANSACTION_ERROR),
			message: transaction_message(e),
			data: None,
		}
	} else {
		Error {
			code: ErrorCode::ServerError(codes::UNKNOWN_ERROR),
			message: "Unknown error when sending transaction.".into(),
			data: Some(Value::String(format!("{:?}", error))),
		}
	}
}

pub fn from_rlp_error(error: DecoderError) -> Error {
	Error {
		code: ErrorCode::InvalidParams,
		message: "Invalid RLP.".into(),
		data: Some(Value::String(format!("{:?}", error))),
	}
}

pub fn from_call_error(error: CallError) -> Error {
	match error {
		CallError::StatePruned => state_pruned(),
		CallError::Exceptional => exceptional(),
		CallError::Execution(e) => execution(e),
		CallError::TransactionNotFound => internal("{}, this should not be the case with eth_call, most likely a bug.", CallError::TransactionNotFound),
	}
}

pub fn unknown_block() -> Error {
	Error {
		code: ErrorCode::ServerError(codes::UNSUPPORTED_REQUEST),
		message: "Unknown block number".into(),
		data: None,
	}
}

pub fn no_filter_error() -> Error {
	Error {
		code: ErrorCode::ServerError(codes::NO_FILTER_ERROR),
		message: "Filter not found".into(),
		data: None
	}
}
