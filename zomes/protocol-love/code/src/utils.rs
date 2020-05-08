use hdk::prelude::{ZomeApiResult, ZomeApiError};
use holochain_wasm_utils::holochain_core_types::entry::AppEntryValue;
use holochain_wasm_utils::holochain_persistence_api::cas::content::Address;
use std::convert::TryFrom;

pub fn get_as_type_ref<R: TryFrom<AppEntryValue>>(address: &Address) -> ZomeApiResult<R> {
	hdk::utils::get_as_type(address.clone())
}

pub fn tag_error<T>(error: ZomeApiError, tag: &str) -> ZomeApiResult<T> {
	let mut error_msg = "".to_owned();
	error_msg.push_str(tag);
	error_msg.push_str(&error.to_string());
	return Err(error_msg.into());
}

pub fn t<T>(tag: &str, result: ZomeApiResult<T>) -> ZomeApiResult<T> {
	match result {
		Ok(val) => Ok(val),
		Err(error) =>
			tag_error(error, tag)
	}
}
