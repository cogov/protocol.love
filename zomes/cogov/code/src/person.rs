use std::prelude::v1::Into;
use std::borrow::Borrow;
use hdk::{EntryValidationData};
use hdk::holochain_core_types::dna::entry_types::Sharing;
use hdk::holochain_json_api::{
	json::JsonString,
	error::JsonError,
};
use hdk::prelude::{ZomeApiResult, ValidatingEntryType};
use holochain_persistence_api::cas::content::Address;
use holochain_wasm_utils::holochain_core_types::entry::Entry;
use crate::utils::{match_tag_error};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct PersonParamsTZome {
	pub name: String,
	pub agent_address: Option<Address>,
	pub status: Option<PersonStatus>,
}

impl Into<PersonParams> for PersonParamsTZome {
	fn into(self) -> PersonParams {
		PersonParams {
			name: self.name,
			agent_address: match self.agent_address {
				Some(agent_address) => agent_address,
				None => PersonParams::default().agent_address,
			},
			status: match self.status {
				Some(status) => status,
				None => PersonParams::default().status,
			},
		}
	}
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct PersonParams {
	pub name: String,
	pub agent_address: Address,
	pub status: PersonStatus,
}

impl Default for PersonParams {
	fn default() -> Self {
		PersonParams {
			name: "".to_string(),
			agent_address: hdk::AGENT_ADDRESS.clone(),
			status: PersonStatus::Active,
		}
	}
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub enum PersonStatus {
	Active,
	Inactive,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Person {
	pub agent_address: Address,
	pub name: String,
	pub status: PersonStatus,
}

impl Default for Person {
	fn default() -> Self {
		Person {
			agent_address: hdk::AGENT_ADDRESS.clone(),
			name: "".to_string(),
			status: PersonStatus::Active,
		}
	}
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct PersonPayload {
	pub person_address: Address,
	pub person: Person,
}

pub fn person_def() -> ValidatingEntryType {
	entry!(
		name: "person",
		description: "A cogov person",
		sharing: Sharing::Public,
		validation_package: || {
			hdk::ValidationPackageDefinition::Entry
		},
		validation: | validation_data: hdk::EntryValidationData<Person>| {
			match validation_data{
				EntryValidationData::Create { entry, validation_data } => {
					if !validation_data.sources().contains(&entry.agent_address) {
						return Err(
							String::from("Person representing agent must be created by agent")
						);
					}
					validate_name(&entry.name)?;
					Ok(())
				}
				EntryValidationData::Modify { new_entry, old_entry, validation_data, .. } => {
					if new_entry.agent_address != old_entry.agent_address {
						return Err(String::from("agent_address cannot be updated"));
					}
					if !validation_data.sources().contains(&old_entry.agent_address) {
						return Err(String::from("Person can only update by oneself"));
					}
					validate_name(&new_entry.name)?;
					Ok(())
				}
				EntryValidationData::Delete { .. } => {
					return Err(String::from("Person cannot be deleted"));
				}
			}
		},
		links: [
			to!(
				"collective",
				link_type: "person_collective",
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

fn validate_name(name: &str) -> Result<(), String> {
	if name.len() > 64 {
		Err("Name is too long".into())
	} else {
		Ok(())
	}
}

pub fn create_person(person_params: PersonParams) -> ZomeApiResult<PersonPayload> {
	let CommitPersonResponse(
		person_address,
		_person_entry,
		person,
	) =
		match_tag_error(
			commit_person(Person {
				name: person_params.name,
				agent_address: person_params.agent_address,
				status: person_params.status,
			}),
			"create_person: "
		)?;
	Ok(PersonPayload {
		person_address,
		person,
	})
}

pub fn get_person(person_address: Address) -> ZomeApiResult<PersonPayload> {
	let person_address__ = person_address.clone();
	let person = hdk::utils::get_as_type(person_address__)?;
	Ok(PersonPayload {
		person_address,
		person,
	})
}

struct CommitPersonResponse(Address, Entry, Person);

fn commit_person(person: Person) -> ZomeApiResult<CommitPersonResponse> {
	let person_entry = Entry::App("person".into(), person.borrow().into());
	let person_address = hdk::commit_entry(&person_entry)?;
	Ok(CommitPersonResponse(person_address, person_entry, person))
}
