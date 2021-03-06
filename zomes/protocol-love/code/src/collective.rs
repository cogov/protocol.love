use std::borrow::Borrow;
use hdk::EntryValidationData;
use hdk::holochain_core_types::dna::entry_types::Sharing;
use hdk::holochain_json_api::{
	json::JsonString,
	error::JsonError,
};
use crate::ledger::create_collective_ledger;
use holochain_wasm_utils::holochain_core_types::entry::Entry;
use hdk::prelude::{ZomeApiResult, ValidatingEntryType};
use holochain_wasm_utils::holochain_persistence_api::cas::content::Address;
use crate::action::{Action, ActionStatus, ActionStrategy, ActionOp, ActionEntry};
use crate::utils::{get_as_type_ref, t};
use crate::person::{Person, create_person, PersonParams, PersonPayload};
use holochain_wasm_utils::holochain_core_types::link::LinkMatch;
use std::fmt;
use holochain_wasm_utils::api_serialization::get_links::{GetLinksOptions};

/// A collective.
///
/// Has a name & an optional admin_address.
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Collective {
	/// Name of the Collective
	pub name: String,
	/// Administrator address of the collective.
	///
	/// Used during the initial creation of the collective.
	///
	/// TODO: Move to a link or a [CollectivePersonTag](enum.CollectivePersonTag.html)
	pub admin_address: Option<Address>,
}

/// Api params to create a [Collective](struct.Collective.html) along with an optional `admin_address`.
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct CreateCollectiveParams {
	pub name: String,
	pub admin_address: Option<Address>,
}

impl Into<Collective> for CreateCollectiveParams {
	fn into(self) -> Collective {
		Collective {
			name: self.name,
			admin_address: self.admin_address,
		}
	}
}

impl Default for Collective {
	fn default() -> Self {
		Collective {
			name: "unnamed collective".to_string(),
			admin_address: Default::default(),
		}
	}
}

/// Api payload containing a collective_address & collective.
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct CollectivePayload {
	pub collective_address: Address,
	pub collective: Collective,
}

impl Default for CollectivePayload {
	fn default() -> Self {
		CollectivePayload {
			collective_address: Default::default(),
			collective: Default::default(),
		}
	}
}

/// Tag of a [Person](struct.Person.html) in a [Collective](struct.Collective.html).
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub enum CollectivePersonTag {
	/// Creator of the [Collective](struct.Collective.html)
	Creator,
}

impl fmt::Display for CollectivePersonTag {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

/// Api Payload of People in a Collective.
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct CollectivePeoplePayload {
	pub collective_address: Address,
	/// [People](struct.Person.html) in a [Collective](struct.Collective.html).
	pub collective_people: Vec<Person>,
}

/// Returns a Holochain entry definition for a collective.
pub fn collective_def() -> ValidatingEntryType {
	entry!(
		name: "collective",
		description: "A protocol.love collective",
		sharing: Sharing::Public,
		validation_package: || {
			hdk::ValidationPackageDefinition::Entry
		},
		validation: | validation_data: hdk::EntryValidationData<Collective>| {
			match validation_data{
				EntryValidationData::Create { entry, validation_data } => {
					match entry.admin_address {
						Some(admin_address) => {
							let admin:Person = t("validation error: collective: fetch admin: ",
								hdk::utils::get_as_type(admin_address), )?;
							if !validation_data.sources().contains(&admin.agent_address) {
								return Err(
									"Collective must be created with same agent as the given person".into()
								);
							}
						},
						None => {
							return Err("Collective being created must have an admin".into())
						}
					}
					Ok(())
				}
				EntryValidationData::Modify { old_entry, validation_data, .. } => {
					let admin_address = old_entry.admin_address;
					match admin_address {
						Some(admin_address) => {
							let admin:Person = hdk::utils::get_as_type(admin_address)?;
							if !validation_data.sources().contains(&admin.agent_address) {
								return Err(
									"Collective can only be modified by the admin".into()
								);
							};
						},
						None => {
							// TODO: Logic for a Proposal to update this collective
							return Err(
								"Collective can only be modified with an executed proposal".into()
							);
						}
					}

					Ok(())
				}
				EntryValidationData::Delete { .. } => {
					return Err("Collective cannot be deleted".into());
				}
			}
		},
		links: [
			to!(
				"action",
				link_type: "collective->action",
				validation_package: || {
					hdk::ValidationPackageDefinition::Entry
				},
				validation: |_validation_data: hdk::LinkValidationData| {
					Ok(())
				}
			),
			to!(
				"person",
				link_type: "collective->person",
				validation_package: || {
					hdk::ValidationPackageDefinition::Entry
				},
				validation: |_validation_data: hdk::LinkValidationData| {
					Ok(())
				}
			),
			to!(
				"ledger",
				link_type: "collective->ledger",
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

/// Api function to create & commit a [Collective](struct.Collective.html) along with the admin.
///
/// The optional admin_address defaults to the `hdk::AGENT_ADDRESS`.
///
/// # Test:
///
/// curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "protocol-love", "function": "create_collective", "args": { "collective": { "name": "Collective 0" } } }}' http://127.0.0.1:8888
pub fn create_collective(
	collective_params: CreateCollectiveParams
) -> ZomeApiResult<CollectivePayload> {
	// TODO: Set name when answered: https://forum.holochain.org/t/writing-a-validation-rule-that-checks-the-entry-author-against-the-data-being-added-the-entry/1545/14?u=btakita
	let PersonPayload {
		person: _admin,
		person_address: admin_address,
	} = match collective_params.admin_address {
		Some(admin_address) => {
			PersonPayload {
				person_address: admin_address.clone(),
				person: (
					t("create_collective: get_as_type: ",
						hdk::utils::get_as_type(
							admin_address.clone()
						),
					)?),
			}
		}
		None => {
			t("create_collective: create_person: ",
				create_person(PersonParams {
					agent_address: (
						match collective_params.admin_address {
							Some(admin_address) => admin_address,
							None => PersonParams::default().agent_address
						}
					),
					..PersonParams::default()
				}))?
		}
	};
	let CommitCollectiveResponse(
		collective_address,
		_collective_entry,
		collective,
	) =
		t("create_collective: ", commit_collective(
			Collective {
				name: collective_params.name,
				admin_address: Some(admin_address.clone()),
			}))?;
	t("create_collective: ", create_create_collective_action(
		&collective_address,
		&collective,
	))?;
	t("create_collective: ", create_collective_ledger(
		&collective.borrow(),
		&collective_address,
	))?;
	t("create_collective: ", create_set_collective_name_action(
		&collective_address,
		&collective.name,
		None,
	))?;
	t("create_collective: ", add_collective_person(
		&collective_address,
		&admin_address,
	))?;
	Ok(CollectivePayload {
		collective_address,
		collective,
	})
}

/// Api to get the [Collective](struct.Collective.html).
///
/// # Test:
///
/// curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "protocol-love", "function": "get_collective", "args": { "collective_address": "addr" } }}' http://127.0.0.1:8888
pub fn get_collective(collective_address: Address) -> ZomeApiResult<CollectivePayload> {
	let collective_address__ = collective_address.clone();
	let collective =
		hdk::utils::get_as_type(collective_address__)?;
	Ok(CollectivePayload {
		collective_address,
		collective,
		..CollectivePayload::default()
	})
}

/// Api to set the [Collective](struct.Collective.html) name.
pub fn set_collective_name(
	collective_address: Address,
	name: String,
) -> ZomeApiResult<CollectivePayload> {
	let saved_collective = get_as_type_ref(&collective_address)?;
	let collective = Collective {
		name,
		..saved_collective
	};
	update_collective(&collective_address, &collective)?;
	create_set_collective_name_action(
		&collective_address,
		&collective.name,
		Some(&saved_collective.name),
	)?;
	Ok(CollectivePayload {
		collective_address,
		collective,
		..CollectivePayload::default()
	})
}

/// Api to get the [People](struct.Person.html) in the [Collective](struct.Collective.html).
pub fn get_collective_people(
	collective_address: Address
) -> ZomeApiResult<CollectivePeoplePayload> {
	hdk::get_links_with_options(
		&collective_address,
		LinkMatch::Exactly("collective->person"),
		LinkMatch::Any,
		GetLinksOptions::default(),
	)?;
	let collective_people =
		t("get_collective_people: get_links_and_load_type: ",
			hdk::utils::get_links_and_load_type(
				&collective_address,
				LinkMatch::Exactly("collective->person"),
				LinkMatch::Any,
			),
		)?;
	Ok(CollectivePeoplePayload {
		collective_address,
		collective_people,
	})
}

fn update_collective(
	collective_address: &Address,
	collective: &Collective,
) -> ZomeApiResult<Address> {
	let collective_entry = Entry::App("collective".into(), collective.into());
	hdk::update_entry(collective_entry, &collective_address)
}

struct CommitCollectiveResponse(Address, Entry, Collective);

fn commit_collective(collective: Collective) -> ZomeApiResult<CommitCollectiveResponse> {
	let collective_entry = Entry::App("collective".into(), collective.borrow().into());
	let collective_address =
		t("commit_collective: ", hdk::commit_entry(&collective_entry))?;
	Ok(CommitCollectiveResponse(collective_address, collective_entry, collective))
}

fn create_create_collective_action(
	collective_address: &Address,
	collective: &Collective,
) -> ZomeApiResult<ActionEntry> {
	create_collective_action(
		collective_address,
		ActionOp::CreateCollective,
		collective.into(),
		serde_json::value::Value::Null.into(),
		&"create_collective".into(),
		ActionStrategy::SystemAutomatic,
	)
}

fn add_collective_person(
	collective_address: &Address,
	person_address: &Address,
) -> ZomeApiResult<Address> {
	let collective_person_address =
		t("add_collective_person: ", hdk::link_entries(
			collective_address,
			person_address,
			"collective->person",
			&CollectivePersonTag::Creator.to_string(),
		))?;
	create_add_collective_person_action(collective_address, person_address)?;
	Ok(collective_person_address)
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
struct AddCollectivePersonActionData {
	person_address: Address,
}

fn create_add_collective_person_action(
	collective_address: &Address,
	person_address: &Address,
) -> ZomeApiResult<ActionEntry> {
	create_collective_action(
		collective_address,
		ActionOp::AddCollectivePerson,
		AddCollectivePersonActionData {
			person_address: person_address.clone(),
		}.into(),
		serde_json::value::Value::Null.into(),
		&"add_collective_person".into(),
		ActionStrategy::SystemAutomatic,
	)
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
struct SetCollectiveNameActionData {
	name: String
}

fn create_set_collective_name_action(
	collective_address: &Address,
	name: &String,
	prev_name_opt: Option<&String>,
) -> ZomeApiResult<ActionEntry> {
	create_collective_action(
		collective_address,
		ActionOp::SetCollectiveName,
		SetCollectiveNameActionData {
			name: name.clone()
		}.into(),
		match prev_name_opt {
			Some(prev_name) => SetCollectiveNameActionData {
				name: prev_name.clone(),
			}.into(),
			None => serde_json::value::Value::Null.into()
		},
		&"set_collective_name".into(),
		ActionStrategy::SystemAutomatic,
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
	prev_data: JsonString,
	tag: &String,
	strategy: ActionStrategy,
) -> ZomeApiResult<ActionEntry> {
	let collective_action = Action {
		op,
		status: ActionStatus::Executed,
		data,
		prev_data,
		tag: tag.into(),
		strategy: strategy.into(),
	};
	let action_entry = Entry::App(
		"action".into(),
		collective_action.borrow().into());
	let action_address =
		t("create_collective_action: commit_entry: ",
			hdk::commit_entry(&action_entry))?;
	t("create_collective_action: collective->action: ",
		hdk::link_entries(
			&collective_address,
			&action_address,
			"collective->action",
			tag,
		))?;
	Ok((action_address, action_entry, collective_action))
}
