use hdk::holochain_json_api::{
	json::JsonString,
	error::JsonError,
};
use hdk::holochain_core_types::dna::entry_types::Sharing;
use crate::collective::Collective;
use holochain_wasm_utils::holochain_persistence_api::cas::content::Address;
use hdk::prelude::{ZomeApiResult, ValidatingEntryType};
use holochain_wasm_utils::holochain_core_types::entry::Entry;
use crate::utils::match_tag_error;

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

pub fn ledger_def() -> ValidatingEntryType {
	entry!(
		name: "ledger",
		description: "A cogov collective ledger",
		sharing: Sharing::Public,
		validation_package: || {
			hdk::ValidationPackageDefinition::Entry
		},
		validation: | _validation_data: hdk::EntryValidationData<Ledger>| {
			Ok(())
		}
	)
}

pub fn create_collective_ledger(collective: &Collective, collective_address: &Address) -> ZomeApiResult<Address> {
	let ledger_name =
		format!("Primary Ledger for {}", collective.name).to_string();
	let ledger = Ledger {
		name: ledger_name,
		..Default::default()
	};
	let ledger_address =
		match_tag_error(
			commit_ledger(ledger),
			"create_collective_ledger: ",
		)?;
	match_tag_error(
		hdk::link_entries(
			&collective_address,
			&ledger_address,
			"collective_ledger",
			"ledger_primary",
		),
		"create_collective_ledger: link_entries: ",
	)?;
	Ok(ledger_address)
}

pub fn commit_ledger(ledger: Ledger) -> ZomeApiResult<Address> {
	let ledger_entry = Entry::App("ledger".into(), ledger.into());
	let ledger_address =
		match_tag_error(
			hdk::commit_entry(&ledger_entry),
			"commit_ledger: ",
		)?;
	Ok(ledger_address)
}
