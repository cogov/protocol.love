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

/// An `Action` that updates the state in the CoGov system.
///
/// Every action that updates the state is represented by
/// the Action struct.
///
/// # Examples
///
/// ```
/// Action {
///		op: ActionOp::CreateCollective,
/// 	status: ActionStatus::Executed,
/// 	data: json!({"name": "My Collective", "admin_address": hdk::AGENT_ADDRESS.clone()}),
/// 	tag: "create_collective".into(),
/// 	strategy: ActionStrategy::SystemAutomatic
/// }
/// ```
///
/// TODO: `Action` should enable a reversible action to support undo operations.
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Action {
	/// Represents each of the allowed operations
	pub op: ActionOp,
	/// Lifecycle Status of the Action
	pub status: ActionStatus,
	/// Action Data encoded as JSON.
	pub data: JsonString,
	pub tag: String,
	/// How the Action was performed
	pub strategy: ActionStrategy,
}

/// An operation for an [Action](struct.Action.html).
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub enum ActionOp {
	CreateCollective,
	AddCollectivePerson,
	SetCollectiveName,
}

/// The lifecycle status of an [Action](struct.Action.html).
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub enum ActionStatus {
	/// Action is currently opened & not completed
	Open,
	/// Action is executed & completed
	///
	/// TODO: Rename
	Executed,
}

/// How an [Action](struct.Action.html) is performed.
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub enum ActionStrategy {
	/// Performed via automation by the system
	SystemAutomatic,
	/// TODO: Evaluate
	PrivilegedAction,
	/// TODO: Evaluate
	NewDiscussionMessage,
}

/// An api payload returning [Actions](struct.Action.html) & the `collective_address`.
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct ActionsPayload {
	pub collective_address: Address,
	pub actions: Vec<Action>,
}

/// A tuple containing an [Address](type.Address.html), [Entry](enum.Entry.html), & [Action](struct.Action.html)
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
			"collective->action",
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
			"collective->action",
			"",
		)?;
		hdk::link_entries(
			&parent_action_address,
			&action_address,
			"child->action",
			"",
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
				link_type: "child->action",
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

/// Get an [ActionsPayload](struct.ActionsPayload.html) of all of the [Actions](struct.Action.html)
/// linked to the [Collective](struct.Collective.html).
///
/// Test:
/// ```
/// curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "cogov", "function": "get_collective", "args": { "collective_address": "addr" } }}' http://127.0.0.1:8888
/// ```
pub fn get_actions(collective_address: Address) -> ZomeApiResult<ActionsPayload> {
	let mut actions = hdk::utils::get_links_and_load_type(
		&collective_address,
		LinkMatch::Exactly("collective->action"),
		LinkMatch::Any,
	)?;
	actions.reverse();
	Ok(ActionsPayload {
		collective_address,
		actions,
	})
}
