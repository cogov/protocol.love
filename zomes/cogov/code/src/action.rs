use hdk::holochain_json_api::{
	json::JsonString,
	error::JsonError,
};
use holochain_wasm_utils::holochain_persistence_api::cas::content::Address;
use holochain_wasm_utils::holochain_core_types::entry::Entry;
use std::borrow::Borrow;
use hdk::error::ZomeApiResult;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Action {
	pub op: String,
	pub status: ActionStatus,
	pub data: JsonString,
	pub tag: String,
	pub action_intent: ActionIntent,
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

pub trait RootAction {
	fn commit_action(self, collective_address: Address) -> ZomeApiResult<(Address, Entry, Action)>;
}

pub trait ChildAction {
	fn commit_action(self, collective_address: Address, parent_action_address: Address) -> ZomeApiResult<(Address, Entry, Action)>;
}

impl RootAction for Action {
	fn commit_action(self, collective_address: Address) -> ZomeApiResult<(Address, Entry, Action)> {
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
	fn commit_action(self, collective_address: Address, parent_action_address: Address) -> ZomeApiResult<(Address, Entry, Action)> {
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
