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
use crate::utils::get_as_type_ref;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct CollectiveParams {
	pub name: String,
	pub total_shares: i64,
}

impl Into<Collective> for CollectiveParams {
	fn into(self) -> Collective {
		Collective {
			name: self.name,
			total_shares: self.total_shares,
		}
	}
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Collective {
	pub name: String,
	pub total_shares: i64,
}

impl Default for Collective {
	fn default() -> Self {
		Collective {
			name: "unnamed collective".to_string(),
			total_shares: 100000,
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
pub fn create_collective(collective_params: CollectiveParams) -> ZomeApiResult<CollectivePayload> {
	let CommitCollectiveResponse(collective_address, _collective_entry, collective) =
		commit_collective(collective_params.into())?;
	create_collective_ledger(&collective.borrow(), &collective_address)?;
	create_create_collective_action(&collective_address, &collective)?;
	create_set_collective_name_action(&collective_address, &collective.name)?;
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

pub fn set_collective_name(collective_address: Address, collective_name: String) -> ZomeApiResult<CollectivePayload> {
	let saved_collective = get_as_type_ref(&collective_address)?;
	let collective = Collective {
		name: collective_name,
		..saved_collective
	};
	let collective_entry = Entry::App("collective".into(), (&collective).into());
	hdk::update_entry(collective_entry, &collective_address)?;
	create_set_collective_name_action(&collective_address, &collective.name)?;
	Ok(CollectivePayload {
		collective_address,
		collective,
	})
}

struct CommitCollectiveResponse(Address, Entry, Collective);

fn commit_collective(collective: Collective) -> ZomeApiResult<CommitCollectiveResponse> {
	let collective_entry = Entry::App("collective".into(), collective.borrow().into());
	let collective_address = hdk::commit_entry(&collective_entry)?;
	Ok(CommitCollectiveResponse(collective_address, collective_entry, collective))
}

fn create_create_collective_action(collective_address: &Address, collective: &Collective) -> ZomeApiResult<(Address, Entry, Action)> {
	let create_collective_action = Action {
		op: ActionOp::CreateCollective,
		status: ActionStatus::Executed,
		data: collective.into(),
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
	Ok((action_address, action_entry, create_collective_action))
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
struct CollectiveNameActionData {
	collective_name: String
}

fn create_set_collective_name_action(collective_address: &Address, collective_name: &String) -> ZomeApiResult<(Address, Entry, Action)> {
	let set_collective_name_action = Action {
		op: ActionOp::SetCollectiveName,
		status: ActionStatus::Executed,
		data: CollectiveNameActionData {
			collective_name: collective_name.clone()
		}.into(),
		tag: "".into(),
		action_intent: ActionIntent::SystemAutomatic,
	};
	let action_entry = Entry::App(
		"action".into(),
		set_collective_name_action.borrow().into());
	let action_address = hdk::commit_entry(&action_entry)?;
	hdk::link_entries(
		&collective_address,
		&action_address,
		"collective_action",
		"set_collective_name",
	)?;
	Ok((action_address, action_entry, set_collective_name_action))
}
