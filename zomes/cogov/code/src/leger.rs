use hdk::holochain_json_api::{
	json::JsonString,
	error::JsonError,
};
use crate::collective::Collective;
use holochain_wasm_utils::holochain_persistence_api::cas::content::Address;
use hdk::prelude::ZomeApiResult;
use holochain_wasm_utils::holochain_core_types::entry::Entry;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Ledger {
	pub name: String,
}

impl Default for Ledger {
	fn default() -> Self {
		Ledger {
			name: "unnamed ledger".to_string(),
		}
	}
}

#[allow(dead_code)]
pub fn create_collective_ledger(collective: &Collective, _collective_address: &Address) -> ZomeApiResult<Address> {
	let ledger_name = format!("Primary Ledger for {}", collective.name).to_string();
	let ledger = Ledger {
		name: ledger_name,
		..Default::default()
	};
	let ledger_address = commit_ledger(ledger)?;
//	hdk::link_entries(
//		&collective_address,
//		&ledger_address,
//		"collective_leger",
//		"ledger_primary",
//	)?;
	Ok(ledger_address)
}

pub fn commit_ledger(ledger: Ledger) -> ZomeApiResult<Address> {
	let ledger_entry = Entry::App("ledger".into(), ledger.into());
	let ledger_address = hdk::commit_entry(&ledger_entry)?;
	Ok(ledger_address)
}
