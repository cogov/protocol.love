use hdk::prelude::ZomeApiResult;
use holochain_wasm_utils::holochain_core_types::entry::AppEntryValue;
use holochain_wasm_utils::holochain_persistence_api::cas::content::Address;
use std::convert::TryFrom;

pub fn get_as_type_ref<R: TryFrom<AppEntryValue>>(address: &Address) -> ZomeApiResult<R> {
	hdk::utils::get_as_type(address.clone())
}
