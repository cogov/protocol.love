use std::borrow::Borrow;
use hdk::holochain_core_types::dna::entry_types::Sharing;
use hdk::holochain_json_api::{
	json::JsonString,
	error::JsonError,
};
use crate::ledger::create_collective_ledger;
use holochain_wasm_utils::holochain_core_types::entry::Entry;
use hdk::prelude::{ZomeApiResult, ValidatingEntryType};
use holochain_wasm_utils::holochain_persistence_api::cas::content::Address;
use crate::action::{Action, ActionStatus, ActionIntent, ActionOp};

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

pub fn collective_def() -> ValidatingEntryType {
	entry!(
			name: "collective",
			description: "A cogov collective",
			sharing: Sharing::Public,
			validation_package: || {
				hdk::ValidationPackageDefinition::Entry
			},
			validation: | _validation_data: hdk::EntryValidationData<Collective>| {
				Ok(())
			},
			links: [
				to!(
					"action",
					link_type: "collective_action",
					validation_package: || {
						hdk::ValidationPackageDefinition::Entry
					},
					validation: |_validation_data: hdk::LinkValidationData| {
						Ok(())
					}
				),
				to!(
					"ledger",
					link_type: "collective_ledger",
					validation_package: || {
						hdk::ValidationPackageDefinition::Entry
					},
					validation: |_validation_data: hdk::LinkValidationData| {
						Ok(())
					}
				)
			]
    )
}

// curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "cogov", "function": "commit_collective", "args": { "collective": { "name": "Collective 0" } } }}' http://127.0.0.1:8888
pub fn create_collective(collective: CollectiveParams) -> ZomeApiResult<CollectivePayload> {
	let (collective_address, _collective_entry, collective) = commit_collective(
		Collective {
			name: collective.name,
		})?;
	create_collective_ledger(&collective.borrow(), &collective_address)?;
	let create_collective_action = Action {
		op: ActionOp::CreateCollective,
		status: ActionStatus::Executed,
		data: (&collective).into(),
		tag: "".into(),
		action_intent: ActionIntent::SystemAutomatic,
	};
	let action_entry = Entry::App(
		"action".into(),
		create_collective_action.borrow().into());
	let action_address = hdk::commit_entry(&action_entry)?;
	hdk::link_entries(
		&collective_address,
		&action_address,
		"collective_action",
		"create_collective",
	)?;
	Ok(CollectivePayload {
		collective_address,
		collective,
	})
}

// curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "cogov", "function": "get_collective", "args": { "collective_address": "addr" } }}' http://127.0.0.1:8888
pub fn get_collective(collective_address: Address) -> ZomeApiResult<CollectivePayload> {
	let collective_address__ = collective_address.clone();
	let collective = hdk::utils::get_as_type(collective_address__)?;
	Ok(CollectivePayload {
		collective_address,
		collective,
	})
}

fn commit_collective(collective: Collective) -> ZomeApiResult<(Address, Entry, Collective)> {
	let collective_entry = Entry::App("collective".into(), collective.borrow().into());
	let collective_address = hdk::commit_entry(&collective_entry)?;
	Ok((collective_address, collective_entry, collective))
}
