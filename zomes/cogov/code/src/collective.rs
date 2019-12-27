use std::borrow::Borrow;
use hdk::holochain_json_api::{
	json::JsonString,
	error::JsonError,
};
use crate::leger::create_collective_ledger;
use holochain_wasm_utils::holochain_core_types::entry::Entry;
use hdk::prelude::ZomeApiError;
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

pub fn commit_collective(collective: Collective) -> Result<(Address, Entry), ZomeApiError> {
	let collective_entry = Entry::App("collective".into(), collective.borrow().into());
	let collective_address = hdk::commit_entry(&collective_entry)?;
	create_collective_ledger(&collective, &collective_address)?;
	Ok((collective_address, collective_entry))
}
