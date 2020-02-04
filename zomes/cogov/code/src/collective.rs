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
use crate::action::{Action, ActionStatus, ActionIntent, ActionOp, ActionEntry};
use crate::utils::get_as_type_ref;
use crate::person::Person;
use holochain_wasm_utils::holochain_core_types::link::LinkMatch;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct CollectiveParams {
	pub name: String,
	pub person_address: Address,
}

impl Into<Collective> for CollectiveParams {
	fn into(self) -> Collective {
		Collective {
			name: self.name,
		}
	}
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

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub enum CollectivePersonTag {
	Creator,
}

impl fmt::Display for CollectivePersonTag {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct CollectiveCreatorPayload {
	pub collective_address: Address,
	pub collective_creator: Person,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct CollectivePeoplePayload {
	pub collective_address: Address,
	pub collective_people: Vec<Person>,
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
					"person",
					link_type: "collective_person",
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
	let person_address = collective_params.person_address.clone();
	let CommitCollectiveResponse(collective_address, _collective_entry, collective) =
		commit_collective(collective_params.into())?;
	create_create_collective_action(&collective_address, &collective)?;
	create_collective_ledger(&collective.borrow(), &collective_address)?;
	create_set_collective_name_action(&collective_address, &collective.name)?;
	add_collective_person(&collective_address, &person_address)?;
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

pub fn set_collective_name(collective_address: Address, name: String) -> ZomeApiResult<CollectivePayload> {
	let saved_collective = get_as_type_ref(&collective_address)?;
	let collective = Collective {
		name,
		..saved_collective
	};
	update_collective(&collective_address, &collective)?;
	create_set_collective_name_action(&collective_address, &collective.name)?;
	Ok(CollectivePayload {
		collective_address,
		collective,
	})
}

pub fn get_collective_creator(collective_address: Address) -> ZomeApiResult<CollectiveCreatorPayload> {
	let mut collective_creators =
		hdk::utils::get_links_and_load_type(
			&collective_address,
			LinkMatch::Exactly("collective_person"),
			LinkMatch::Exactly(&CollectivePersonTag::Creator.to_string()),
		)?;
	Ok(CollectiveCreatorPayload {
		collective_address,
		collective_creator: collective_creators.remove(0),
	})
}

pub fn get_collective_people(collective_address: Address) -> ZomeApiResult<CollectivePeoplePayload> {
	let collective_people =
		hdk::utils::get_links_and_load_type(
			&collective_address,
			LinkMatch::Exactly("collective_person"),
			LinkMatch::Any,
		)?;
	Ok(CollectivePeoplePayload {
		collective_address,
		collective_people,
	})
}

fn update_collective(collective_address: &Address, collective: &Collective) -> ZomeApiResult<Address> {
	let collective_entry = Entry::App("collective".into(), collective.into());
	hdk::update_entry(collective_entry, &collective_address)
}

struct CommitCollectiveResponse(Address, Entry, Collective);

fn commit_collective(collective: Collective) -> ZomeApiResult<CommitCollectiveResponse> {
	let collective_entry = Entry::App("collective".into(), collective.borrow().into());
	let collective_address = hdk::commit_entry(&collective_entry)?;
	Ok(CommitCollectiveResponse(collective_address, collective_entry, collective))
}

fn create_create_collective_action(collective_address: &Address, collective: &Collective) -> ZomeApiResult<ActionEntry> {
	create_collective_action(
		collective_address,
		ActionOp::CreateCollective,
		collective.into(),
		&"create_collective".into(),
		ActionIntent::SystemAutomatic,
	)
}

fn add_collective_person(
	collective_address: &Address,
	person_address: &Address,
) -> ZomeApiResult<Address> {
	let link_address = hdk::link_entries(
		collective_address,
		person_address,
		"collective_person",
		&CollectivePersonTag::Creator.to_string(),
	)?;
	create_add_collective_person_action(collective_address, person_address)?;
	Ok(link_address)
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
struct AddCollectivePersonActionData {
	person_address: Address,
}

fn create_add_collective_person_action(collective_address: &Address, person_address: &Address) -> ZomeApiResult<ActionEntry> {
	create_collective_action(
		collective_address,
		ActionOp::AddCollectivePerson,
		AddCollectivePersonActionData {
			person_address: person_address.clone(),
		}.into(),
		&"add_collective_person".into(),
		ActionIntent::SystemAutomatic,
	)
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
struct SetCollectiveNameActionData {
	name: String
}

fn create_set_collective_name_action(collective_address: &Address, name: &String) -> ZomeApiResult<ActionEntry> {
	create_collective_action(
		collective_address,
		ActionOp::SetCollectiveName,
		SetCollectiveNameActionData {
			name: name.clone()
		}.into(),
		&"set_collective_name".into(),
		ActionIntent::SystemAutomatic,
	)
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
struct SetTotalSharesActionData {
	total_shares: i64,
}

fn create_collective_action(
	collective_address: &Address,
	op: ActionOp,
	data: JsonString,
	tag: &String,
	action_intent: ActionIntent,
) -> ZomeApiResult<ActionEntry> {
	let collective_action = Action {
		op,
		status: ActionStatus::Executed,
		data: data.into(),
		tag: tag.into(),
		action_intent: action_intent.into(),
	};
	let action_entry = Entry::App(
		"action".into(),
		collective_action.borrow().into());
	let action_address = hdk::commit_entry(&action_entry)?;
	hdk::link_entries(
		&collective_address,
		&action_address,
		"collective_action",
		tag,
	)?;
	Ok((action_address, action_entry, collective_action))
}
