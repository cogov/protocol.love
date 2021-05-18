use std::prelude::v1::Into;
use hdk::prelude::*;
fn _person_path() -> Path {
  Path::from("person")
}
/// Api params with name, optional agent_pub_key, & optional status.
///
/// Convertable into [PersonParams](struct.PersonParams.html).
/// Api function to create & commit a [Person](struct.Person.html).
#[hdk_extern]
pub fn create_person(person_params: PersonParams) -> ExternResult<PersonPayload> {
  let option_collective_entry_hash = person_params.clone().collective_entry_hash;
  match person_params.clone().into() {
    Ok(person) => {
      let person_entry_hash = hash_entry(&person)?;
      let person_header_hash = create_entry(&person)?;
      let bytes: SerializedBytes = person_entry_hash.clone().try_into()?;
      let tag = LinkTag::from(bytes.bytes().to_vec());
      if let Some(collective_entry_hash) = option_collective_entry_hash {
        create_link(
          collective_entry_hash, person_entry_hash.clone(), tag,
        )?;
      };
      Ok(PersonPayload {
        person_header_hash,
        person_entry_hash,
        person,
      })
    }
    Err(err) => Err(err)
  }
}
/// Api function to get a [Person](struct.Person.html).
#[hdk_extern]
pub fn get_person(person_entry_hash: EntryHash) -> ExternResult<PersonPayload> {
  let person_path = _person_path();
  person_path.ensure()?;
  if let Some(element) = get(person_entry_hash.clone(), GetOptions::content())? {
    let option_content: Option<Person> = element.entry().to_app_option()?;
    if let Some(person) = option_content {
      let person_header_hash: HeaderHash = element.header_address().clone();
      return Ok(PersonPayload {
        person_header_hash,
        person_entry_hash,
        person,
      });
    }
  }
  Err(WasmError::Guest("person hash not found".into()))
}
// impl From<Person> for Entry {
//   fn from(person: Person) -> Self {
//     person.person_entry_hash
//   }
// }
#[hdk_extern]
fn validate_person(data: ValidateData) -> ExternResult<ValidateCallbackResult> {
  match data.element.entry().to_app_option::<Person>() {
    Ok(Some(_)) => Ok(ValidateCallbackResult::Valid),
    _ => Ok(ValidateCallbackResult::Invalid("No Person".into()))
  }
}
#[hdk_extern]
fn validate_create_person(data: ValidateData) -> ExternResult<ValidateCallbackResult> {
  match data.element.entry().to_app_option::<Person>() {
    Ok(Some(person)) =>
      if &person.agent_initial_pubkey == data.element.header().author() {
        validate_upsert(person)
      } else {
        Ok(ValidateCallbackResult::Invalid(
          "Agent can only create Person representing oneself".into()
        ))
      },
    _ => Ok(ValidateCallbackResult::Invalid("No person".into()))
  }
}
#[hdk_extern]
fn validate_update_person(data: ValidateData) -> ExternResult<ValidateCallbackResult> {
  match data.element.entry().to_app_option::<Person>() {
    Ok(Some(person)) =>
      if &person.agent_initial_pubkey == data.element.header().author() {
        validate_upsert(person)
      } else {
        Ok(
          ValidateCallbackResult::Invalid(
            "Agent can only update Person representing oneself".to_string()
          )
        )
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
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
pub struct OptionalPersonParams {
  /// Optional agent_initial_pubkey defaults to agent_initial_pubkey
  pub agent_initial_pubkey: Option<AgentPubKey>,
  pub collective_entry_hash: EntryHash,
  pub name: String,
  /// Optional status defaults to [PersonStatus::Active](enum.PersonStatus.html).
  pub status: Option<PersonStatus>,
}
impl Into<PersonParams> for OptionalPersonParams {
  fn into(self) -> PersonParams {
    PersonParams {
      collective_entry_hash: Some(self.collective_entry_hash),
      name: self.name,
      status: match self.status {
        Some(status) => status,
        None => PersonStatus::default(),
      },
    }
  }
}
/// Params for [create_person](fn.create_person.html).
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
pub struct PersonParams {
  pub collective_entry_hash: Option<EntryHash>,
  pub name: String,
  pub status: PersonStatus,
}
impl Into<ExternResult<Person>> for PersonParams {
  fn into(self) -> ExternResult<Person> {
    match agent_info() {
      Ok(agent_info) => Ok(Person {
        agent_initial_pubkey: agent_info.agent_initial_pubkey,
        name: self.name,
        status: self.status,
      }),
      Err(err) => Err(err)
    }
  }
}
/// Is the [Person](struct.Person.html) Active or Inactive.
#[derive(Clone, Copy, Serialize, Deserialize, SerializedBytes, Debug)]
pub enum PersonStatus {
  /// [Person](struct.Person.html) is currently active in the [Collective](struct.Collective.html).
  Inactive,
  /// [Person](struct.Person.html) is currently inactive in the [Collective](struct.Collective.html).
  Active,
}
impl Default for PersonStatus {
  fn default() -> Self {
    PersonStatus::Active
  }
}
/// Person participating with a [Collective](struct.Collective.html).
#[hdk_entry]
#[derive(Clone)]
pub struct Person {
  /// EntryHash of the Person's Holochain agent.
  pub agent_initial_pubkey: AgentPubKey,
  /// Name of the Person.
  pub name: String,
  /// Is the Person Active or Inactive?
  pub status: PersonStatus,
}
/// Api payload containing the `person_entry_hash` & [person](struct.Person.html).
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
pub struct PersonPayload {
  pub person_header_hash: HeaderHash,
  pub person_entry_hash: EntryHash,
  pub person: Person,
}
