use hdk::holochain_json_api::{
	json::JsonString,
	error::JsonError,
};
use hdk::holochain_core_types::dna::entry_types::Sharing;
use crate::collective::Collective;
use holochain_wasm_utils::holochain_persistence_api::cas::content::Address;
use hdk::prelude::{ZomeApiResult, ValidatingEntryType};
use holochain_wasm_utils::holochain_core_types::entry::Entry;
use crate::utils::t;

/// A ledger to account for transactions relating to a [Collective](struct.Collective.html).
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

/// Returns a Holochain entry definition for a ledger.
pub fn ledger_def() -> ValidatingEntryType {
	entry!(
		name: "ledger",
		description: "A protocol.love collective ledger",
		sharing: Sharing::Public,
		validation_package: || {
			hdk::ValidationPackageDefinition::Entry
		},
		validation: | _validation_data: hdk::EntryValidationData<Ledger>| {
			Ok(())
		}
	)
}

/// Create & commit a [Ledger](struct.Ledger.html) for a [Collective](struct.Collective.html).
pub fn create_collective_ledger(collective: &Collective, collective_address: &Address) -> ZomeApiResult<Address> {
	let ledger_name =
		format!("Primary Ledger for {}", collective.name).to_string();
	let ledger = Ledger {
		name: ledger_name,
		..Default::default()
	};
	let ledger_address =
		t("create_collective_ledger: ", commit_ledger(ledger))?;
	t("create_collective_ledger: collective->ledger: ", hdk::link_entries(
		&collective_address,
		&ledger_address,
		"collective->ledger",
		"ledger_primary",
	))?;
	Ok(ledger_address)
}

fn commit_ledger(ledger: Ledger) -> ZomeApiResult<Address> {
	let ledger_entry = Entry::App("ledger".into(), ledger.into());
	let ledger_address =
		t("commit_ledger: ", hdk::commit_entry(&ledger_entry))?;
	Ok(ledger_address)
}
