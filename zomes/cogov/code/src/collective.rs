use std::borrow::Borrow;
use hdk::holochain_json_api::{
	json::JsonString,
	error::JsonError,
};
use crate::leger::create_collective_ledger;
use holochain_wasm_utils::holochain_core_types::entry::Entry;
use hdk::prelude::ZomeApiResult;
use holochain_wasm_utils::holochain_persistence_api::cas::content::Address;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct CollectiveParams {
	pub name: String,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Collective {
	pub name: String,
}

impl Default for Collective {
	fn default() -> Self {
		Collective {
			name: "unnamed collective".to_string(),
		}
	}
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct CollectivePayload {
	pub collective_address: Address,
	pub collective: Collective,
}

pub fn commit_collective(collective: Collective) -> ZomeApiResult<(Address, Entry, Collective)> {
	let collective_entry = Entry::App("collective".into(), collective.borrow().into());
	let collective_address = hdk::commit_entry(&collective_entry)?;
	create_collective_ledger(&collective.borrow(), &collective_address)?;
	Ok((collective_address, collective_entry, collective))
}

pub fn get_collective_entry(collective_address: Address) -> ZomeApiResult<Option<Entry>> {
	let collective_entry = hdk::get_entry(&collective_address)?;
	Ok(collective_entry)
}
