use hdk::prelude::*;
use holochain_wasm_utils::holochain_core_types::entry::AppEntryValue;
use holochain_wasm_utils::holochain_persistence_api::cas::content::EntryHash;
use std::convert::TryFrom;

/// Calls `hdk::utils::get_as_type` with a cloned `address`.
pub fn get_as_type_ref<R: TryFrom<AppEntryValue>>(address: &EntryHash) -> ExternResult<R> {
	hdk::utils::get_as_type(address.clone())
}

/// Prepends `tag` to a `ZomeApiError`.
pub fn tag_error<T>(error: ZomeApiError, tag: &str) -> ExternResult<T> {
	let mut error_msg = "".to_owned();
	error_msg.push_str(tag);
	error_msg.push_str(&error.to_string());
	return Err(error_msg.into());
}

/// Prepends `tag` to a `ZomeApiError` resulting from `ExternResult`.
pub fn t<T>(tag: &str, result: ExternResult<T>) -> ExternResult<T> {
	match result {
		Ok(val) => Ok(val),
		Err(error) =>
			tag_error(error, tag)
	}
}
