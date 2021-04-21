use std::prelude::v1::Into;
use std::borrow::Borrow;
use hdk::{EntryValidationData};
use hdk::holochain_core_types::dna::entry_types::Sharing;
use hdk::holochain_json_api::{
  json::JsonString,
  error::JsonError,
};
use hdk::prelude::*;
use holochain_persistence_api::cas::content::EntryHash;
use holochain_wasm_utils::holochain_core_types::entry::Entry;
use crate::utils::{t};
use hdk::prelude::ElementEntry::Present;
/// Api params with name, optional agent_pubkey, & optional status.
///
/// Convertable into [PersonParams](struct.PersonParams.html).
#[derive(Clone, Serialize, Deserialize, SerializeBytes, Debug)]
pub struct OptionalPersonParams {
  /// Optional agent_pubkey defaults to agent_initial_pubkey
  pub agent_pubkey: Option<AgentPubKey>,
  pub collective_hash: EntryHash,
  pub name: String,
  /// Optional status defaults to [PersonStatus::Active](enum.PersonStatus.html).
  pub status: Option<PersonStatus>,
}
impl Into<PersonParams> for OptionalPersonParams {
  fn into(self) -> PersonParams {
    PersonParams {
      agent_pubkey: match self.agent_pubkey {
        Some(agent_pubkey) => agent_pubkey,
        None => PersonParams::default().agent_pubkey,
      },
      collective_hash: self.collective_hash,
      name: self.name,
      status: match self.status {
        Some(status) => status,
        None => PersonParams::default().status,
      },
    }
  }
}
/// Params for [create_person](fn.create_person.html).
#[derive(Clone, Serialize, Deserialize, SerializeBytes, Debug)]
pub struct PersonParams {
  pub agent_pubkey: AgentPubKey,
  pub collective_hash: EntryHash,
  pub name: String,
  pub status: PersonStatus,
}
impl Default for PersonParams {
  fn default() -> Self {
    PersonParams {
      agent_pubkey: agent_info()?.agent_initial_pubkey,
      collective_hash: Default::default(),
      name: "".to_string(),
      status: PersonStatus::Active,
    }
  }
}
/// Is the [Person](struct.Person.html) Active or Inactive.
#[derive(Clone, Serialize, Deserialize, SerializeBytes, Debug)]
pub enum PersonStatus {
  /// [Person](struct.Person.html) is currently active in the [Collective](struct.Collective.html).
  Inactive,
  /// [Person](struct.Person.html) is currently inactive in the [Collective](struct.Collective.html).
  Active,
}
/// Person participating with a [Collective](struct.Collective.html).
#[derive(Clone, Serialize, Deserialize, SerializeBytes, Debug)]
pub struct Person {
  /// EntryHash of the Person's Holochain agent.
  pub agent_pubkey: AgentPubKey,
  pub collective_hash: EntryHash,
  /// Name of the Person.
  pub name: String,
  /// Is the Person Active or Inactive?
  pub status: PersonStatus,
}
impl Default for Person {
  fn default() -> Self {
    Person {
      agent_pubkey: agent_info()?.agent_initial_pubkey,
      collective_hash: Default::default(),
      name: "".to_string(),
      status: PersonStatus::Active,
    }
  }
}
/// Api payload containing the `person_hash` & [person](struct.Person.html).
#[derive(Clone, Serialize, Deserialize, SerializeBytes, Debug)]
pub struct PersonPayload {
  pub person_hash: EntryHash,
  pub person: Person,
}
#[hdk_extern]
fn validate_person(data: ValidateData) -> ExternResult<ValidateCallbackResult> {
  match data.element.entry().to_app_option::<Person>() {
    Ok(Some(person)) => Ok(ValidateCallbackResult::Valid),
    _ => Ok(ValidateCallbackResult::Invalid("No Person".into()))
  }
}
#[hdk_extern]
fn validate_create_person(data: ValidateData) -> ExternResult<ValidateCallbackResult> {
  match data.element.entry().to_app_option::<Person>() {
    Ok(Some(person)) =>
      if &person.agent_pubkey == data.element.header().author() {
        validate_upsert(person)
      } else {
        Ok(ValidateCallbackResult::Invalid("Agent can only create Person representing oneself".into()));
      },
    _ => Ok(ValidateCallbackResult::Invalid("No person".into()))
  }
}
#[hdk_extern]
fn validate_update_person(data: ValidateData) -> ExternResult<ValidateCallbackResult> {
  match data.element.entry().to_app_option::<Person>() {
    Ok(Some(person)) =>
      if &person.agent_pubkey == data.element.header().author() {
        validate_upsert(person)
      } else {
        Ok(ValidateCallbackResult::Invalid("Agent can only update Person representing oneself".into()));
      },
    _ => Ok(ValidateCallbackResult::Invalid("No person".into()))
  }
}
fn validate_upsert(person: Person) -> ExternResult<ValidateCallbackResult> {
  validate_name(&person.name)
}
fn validate_name(name: &str) -> ExternResult<ValidateCallbackResult> {
  if name.len() > 64 {
    Ok(ValidateCallbackResult::Invalid("Name is too long".into()))
  } else {
    Ok(ValidateCallbackResult::Valid)
  }
}
/// Api function to create & commit a [Person](struct.Person.html).
#[hdk_extern]
pub fn create_person(person_params: PersonParams) -> ExternResult<PersonPayload> {
  create_entry(&person)?;
  let person_hash = hash_entry(&person)?;
  let tag = LinkTag::from("collective_person");
  create_link(collective_hash.clone(), person_hash.clone(), tag);
  Ok(PersonPayload {
    person_hash,
    person,
  })
}
/// Api function to get a [Person](struct.Person.html).
#[hdk_extern]
pub fn get_person(person_hash: EntryHash) -> ExternResult<PersonPayload> {
  let c1_person_hash = person_hash.clone();
  let person = get(c1_person_hash, { strategy: GetStrategy::Latest })?.into();
  Ok(PersonPayload {
    person_hash,
    person,
  })
}
