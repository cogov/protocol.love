use std::prelude::v1::Into;
use std::borrow::Borrow;
use hdk::holochain_core_types::dna::entry_types::Sharing;
use hdk::holochain_json_api::{
	json::JsonString,
	error::JsonError,
};
use hdk::prelude::{ZomeApiResult, ValidatingEntryType};
use holochain_persistence_api::cas::content::Address;
use holochain_wasm_utils::holochain_core_types::entry::Entry;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct PersonParams {
	name: String,
}

impl Into<Person> for PersonParams {
	fn into(self) -> Person {
		Person {
			name: self.name,
		}
	}
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Person {
	name: String,
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
			validation: | _validation_data: hdk::EntryValidationData<Person>| {
				Ok(())
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

pub fn create_person(person_params: PersonParams) -> ZomeApiResult<PersonPayload> {
	let CommitPersonResponse(person_address, _person_entry, person) =
		commit_person(person_params.into())?;
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
