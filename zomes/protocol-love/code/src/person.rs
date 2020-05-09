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
use crate::utils::{t};

/// Api params with name, optional agent_address, & optional status.
///
/// Convertable into [PersonParams](struct.PersonParams.html).
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct OptionalPersonParams {
	pub name: String,
	/// Optional agent_address defaults to hdk::AGENT_ADDRESS
	pub agent_address: Option<Address>,
	/// Optional status defaults to [PersonStatus::Active](enum.PersonStatus.html).
	pub status: Option<PersonStatus>,
}

impl Into<PersonParams> for OptionalPersonParams {
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

/// Params for [create_person](fn.create_person.html).
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

/// Is the [Person](struct.Person.html) Active or Inactive.
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub enum PersonStatus {
	/// [Person](struct.Person.html) is currently active in the [Collective](struct.Collective.html).
	Inactive,
	/// [Person](struct.Person.html) is currently inactive in the [Collective](struct.Collective.html).
	Active,
}

/// Person participating with a [Collective](struct.Collective.html).
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Person {
	/// Address of the Person's Holochain agent.
	pub agent_address: Address,
	/// Name of the Person.
	pub name: String,
	/// Is the Person Active or Inactive?
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

/// Api payload containing the `person_address` & [person](struct.Person.html).
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct PersonPayload {
	pub person_address: Address,
	pub person: Person,
}

/// Returns a Holochain entry definition for a person.
pub fn person_def() -> ValidatingEntryType {
	entry!(
		name: "person",
		description: "A protocol.love person",
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

/// Api function to create & commit a [Person](struct.Person.html).
pub fn create_person(person_params: PersonParams) -> ZomeApiResult<PersonPayload> {
	let CommitPersonResponse(
		person_address,
		_person_entry,
		person,
	) =
		t("create_person: ", commit_person(Person {
			name: person_params.name,
			agent_address: person_params.agent_address,
			status: person_params.status,
		}))?;
	Ok(PersonPayload {
		person_address,
		person,
	})
}

/// Api function to get a [Person](struct.Person.html).
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
