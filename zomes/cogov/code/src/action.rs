use hdk::holochain_json_api::{
	json::JsonString,
	error::JsonError,
};
use hdk::holochain_core_types::dna::entry_types::Sharing;
use holochain_wasm_utils::holochain_persistence_api::cas::content::Address;
use holochain_wasm_utils::holochain_core_types::entry::Entry;
use std::borrow::Borrow;
use hdk::error::ZomeApiResult;
use holochain_wasm_utils::holochain_core_types::link::LinkMatch;
use hdk::prelude::ValidatingEntryType;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Action {
	pub op: ActionOp,
	pub status: ActionStatus,
	pub data: JsonString,
	pub tag: String,
	pub action_intent: ActionIntent,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub enum ActionOp {
	CreateCollective,
	AddCollectivePerson,
	SetCollectiveName,
	SetCollectiveTotalShares,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub enum ActionStatus {
	Open,
	Executed,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub enum ActionIntent {
	SystemAutomatic,
	PrivilegedAction,
	NewDiscussionMessage,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct ActionsPayload {
	pub collective_address: Address,
	pub actions: Vec<Action>,
}

pub type ActionEntry = (Address, Entry, Action);

pub trait RootAction {
	fn commit_action(self, collective_address: Address) -> ZomeApiResult<ActionEntry>;
}

pub trait ChildAction {
	fn commit_action(self, collective_address: Address, parent_action_address: Address) -> ZomeApiResult<ActionEntry>;
}

impl RootAction for Action {
	fn commit_action(self, collective_address: Address) -> ZomeApiResult<ActionEntry> {
		let action_entry = Entry::App("action".into(), self.borrow().into());
		let action_address = hdk::commit_entry(&action_entry)?;
		hdk::link_entries(
			&collective_address,
			&action_address,
			"collective_action",
			"root_action",
		)?;
		Ok((action_address, action_entry, self))
	}
}

impl ChildAction for Action {
	fn commit_action(self, collective_address: Address, parent_action_address: Address) -> ZomeApiResult<ActionEntry> {
		let action_entry = Entry::App("action".into(), self.borrow().into());
		let action_address = hdk::commit_entry(&action_entry)?;
		hdk::link_entries(
			&collective_address,
			&action_address,
			"collective_action",
			"child_action",
		)?;
		hdk::link_entries(
			&parent_action_address,
			&action_address,
			"child_action",
			"child_action",
		)?;
		Ok((action_address, action_entry, self))
	}
}

pub fn action_def() -> ValidatingEntryType {
	entry!(
			name: "action",
			description: "A cogov collective action",
			sharing: Sharing::Public,
			validation_package: || {
				hdk::ValidationPackageDefinition::Entry
			},
			validation: | _validation_data: hdk::EntryValidationData<Action>| {
				Ok(())
			},
			links: [
				from!(
					"collective",
					link_type: "action_collective",
					validation_package: || {
						hdk::ValidationPackageDefinition::Entry
					},
					validation: |_validation_data: hdk::LinkValidationData| {
						Ok(())
					}
				),
				to!(
					"action",
					link_type: "child_action",
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

// curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "cogov", "function": "get_collective", "args": { "collective_address": "addr" } }}' http://127.0.0.1:8888
pub fn get_actions(collective_address: Address) -> ZomeApiResult<ActionsPayload> {
	let actions = hdk::utils::get_links_and_load_type(
		&collective_address,
		LinkMatch::Exactly("collective_action"),
		LinkMatch::Any,
	)?;
	Ok(ActionsPayload {
		collective_address,
		actions,
	})
}
