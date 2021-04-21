use std::borrow::Borrow;
use hdk::EntryValidationData;
use hdk::holochain_core_types::dna::entry_types::Sharing;
use hdk::holochain_json_api::{
  json::JsonString,
  error::JsonError,
};
use crate::ledger::create_collective_ledger;
use holochain_wasm_utils::holochain_core_types::entry::Entry;
use hdk::prelude::*;
use holochain_wasm_utils::holochain_persistence_api::cas::content::EntryHash;
use crate::action::{Action, ActionStatus, ActionStrategy, ActionOp, ActionEntry};
use crate::utils::{get_as_type_ref, t};
use crate::person::{Person, create_person, PersonParams, PersonPayload};
use holochain_wasm_utils::holochain_core_types::link::LinkMatch;
use std::fmt;
use holochain_wasm_utils::api_serialization::get_links::{GetLinksOptions};
/// A collective.
///
/// Has a name & an optional admin_hash.
#[derive(Clone, Serialize, Deserialize, SerializeBytes, Debug)]
pub struct Collective {
  /// Name of the Collective
  pub name: String,
  pub agent_pubkey: AgentPubKey,
  /// Administrator address of the collective.
  ///
  /// Used during the initial creation of the collective.
  ///
  /// TODO: Move to a link or a [CollectivePersonTag](enum.CollectivePersonTag.html)
  pub admin_hash: Option<EntryHash>,
}
/// Api params to create a [Collective](struct.Collective.html) along with an optional `admin_hash`.
#[derive(Clone, Serialize, Deserialize, SerializeBytes, Debug)]
pub struct CreateCollectiveParams {
  pub name: String,
  pub agent_pubkey: Option<AgentPubKey>,
  pub admin_hash: Option<EntryHash>,
}
impl Into<Collective> for CreateCollectiveParams {
  fn into(self) -> Collective {
    Collective {
      name: self.name,
      agent_pubkey: self.agent_pubkey?,
      admin_hash: self.admin_hash,
    }
  }
}
impl Default for Collective {
  fn default() -> Self {
    Collective {
      name: "unnamed collective".to_string(),
      agent_pubkey: agent_info()?.agent_initial_pubkey,
      admin_hash: Default::default(),
    }
  }
}
/// Api payload containing a collective_hash & collective.
#[derive(Clone, Serialize, Deserialize, SerializeBytes, Debug)]
pub struct CollectivePayload {
  pub collective_hash: EntryHash,
  pub collective: Collective,
}
impl Default for CollectivePayload {
  fn default() -> Self {
    CollectivePayload {
      collective_hash: Default::default(),
      collective: Default::default(),
    }
  }
}
/// Tag of a [Person](struct.Person.html) in a [Collective](struct.Collective.html).
#[derive(Clone, Serialize, Deserialize, SerializeBytes, Debug)]
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
#[derive(Clone, Serialize, Deserialize, SerializeBytes, Debug)]
pub struct CollectivePeoplePayload {
  pub collective_hash: EntryHash,
  /// [People](struct.Person.html) in a [Collective](struct.Collective.html).
  pub collective_people: Vec<Person>,
}
#[hdk_extern]
fn validate_collective(data: ValidateData) -> ExternResult<ValidateCallbackResult> {
  get_validate_collective(
    data,
    |collective| Ok(ValidateCallbackResult::Valid),
  )
}
#[hdk_extern]
fn validate_create_collective(data: ValidateData) -> ExternResult<ValidateCallbackResult> {
  get_validate_collective(
    data,
    |collective| validate_upsert_collective(collective)
  )
}
#[hdk_extern]
fn validate_update_collective(data: ValidateData) -> ExternResult<ValidateCallbackResult> {
  get_validate_collective(
    data,
    |collective| validate_upsert_collective(collective)
  )
}
#[hdk_extern]
fn validate_delete_collective(data: ValidateData) -> ExternResult<ValidateCallbackResult> {
  Ok(ValidateCallbackResult::Invalid("Collective cannot be deleted".into()))
}
fn get_validate_collective(
  data: ValidateData,
  f: impl Fn(Collective) -> ExternResult<ValidateCallbackResult>,
) -> ExternResult<ValidateCallbackResult> {
  match data.element.entry().to_app_option::<Collective>() {
    Ok(Some(collective)) => f(collective),
    _ => Ok(ValidateCallbackResult::Invalid("No Collective".into()))
  }
}
fn validate_upsert_collective(collective: Collective) -> ExternResult<ValidateCallbackResult> {
  match get(collective.admin_hash, { strategy: GetStrategy::Latest }) {
    Ok(Some(admin_e)) => {
      match admin_e.entry().to_app_option::<Person>() {
        Ok(Some(admin)) =>
          if &admin.agent_pubkey == data.element.header().author() {
            Ok(ValidateCallbackResult::Valid)
          } else {
            Ok(ValidateCallbackResult::Invalid(
              "Collective must be modified with same agent as the given person".into())
            )
          },
        _ => Ok(ValidateCallbackResult::Invalid("No admin".into()))
      }
    }
    _ => Ok(ValidateCallbackResult::Invalid("No admin".into()))
  }
}
/// Api function to create & commit a [Collective](struct.Collective.html) along with the admin.
///
/// The optional admin_hash defaults to the `hdk::AGENT_ADDRESS`.
///
/// # Test:
///
/// curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "protocol-love", "function": "create_collective", "args": { "collective": { "name": "Collective 0" } } }}' http://127.0.0.1:8888
#[hdk_extern]
fn create_collective(
  collective_params: CreateCollectiveParams
) -> ExternResult<CollectivePayload> {
  // TODO: Set name when answered: https://forum.holochain.org/t/writing-a-validation-rule-that-checks-the-entry-author-against-the-data-being-added-the-entry/1545/14?u=btakita
  let PersonPayload {
    person: _admin,
    person_hash: admin_hash,
  } = match collective_params.admin_hash {
    Some(admin_hash) => {
      PersonPayload {
        person_hash: admin_hash.clone(),
        person: get(admin_hash.clone(), GetOptions::content())?.into(),
      }
    }
    None => {
      create_person(PersonParams {
        agent_pubkey: collective_params.agent_pubkey?,
        ..PersonParams::default()
      })
    }
  };
  let CommitCollectiveResponse(
    collective_hash,
    _collective_entry,
    collective,
  ) =
    t("create_collective: ", commit_collective(
      Collective {
        name: collective_params.name,
        admin_hash: Some(admin_hash.clone()),
        ..Collective::default()
      }))?;
  t("create_collective: ", create_create_collective_action(
    &collective_hash,
    &collective,
  ))?;
  t("create_collective: ", create_collective_ledger(
    &collective.borrow(),
    &collective_hash,
  ))?;
  t("create_collective: ", create_set_collective_name_action(
    &collective_hash,
    &collective.name,
    None,
  ))?;
  t("create_collective: ", add_collective_person(
    &collective_hash,
    &admin_hash,
  ))?;
  Ok(CollectivePayload {
    collective_hash,
    collective,
  })
}
/// Api to get the [Collective](struct.Collective.html).
///
/// # Test:
///
/// curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "protocol-love", "function": "get_collective", "args": { "collective_hash": "addr" } }}' http://127.0.0.1:8888
pub fn get_collective(collective_hash: EntryHash) -> ExternResult<CollectivePayload> {
  let collective_address__ = collective_hash.clone();
  let collective =
    hdk::utils::get_as_type(collective_address__)?;
  Ok(CollectivePayload {
    collective_hash,
    collective,
    ..CollectivePayload::default()
  })
}
/// Api to set the [Collective](struct.Collective.html) name.
pub fn set_collective_name(
  collective_hash: EntryHash,
  name: String,
) -> ExternResult<CollectivePayload> {
  let saved_collective = get_as_type_ref(&collective_hash)?;
  let collective = Collective {
    name,
    ..saved_collective
  };
  update_collective(&collective_hash, &collective)?;
  create_set_collective_name_action(
    &collective_hash,
    &collective.name,
    Some(&saved_collective.name),
  )?;
  Ok(CollectivePayload {
    collective_hash,
    collective,
    ..CollectivePayload::default()
  })
}
/// Api to get the [People](struct.Person.html) in the [Collective](struct.Collective.html).
pub fn get_collective_people(
  collective_hash: EntryHash
) -> ExternResult<CollectivePeoplePayload> {
  hdk::get_links_with_options(
    &collective_hash,
    LinkMatch::Exactly("collective->person"),
    LinkMatch::Any,
    GetLinksOptions::default(),
  )?;
  let collective_people =
    t("get_collective_people: get_links_and_load_type: ",
      hdk::utils::get_links_and_load_type(
        &collective_hash,
        LinkMatch::Exactly("collective->person"),
        LinkMatch::Any,
      ),
    )?;
  Ok(CollectivePeoplePayload {
    collective_hash,
    collective_people,
  })
}
fn update_collective(
  collective_hash: &EntryHash,
  collective: &Collective,
) -> ExternResult<EntryHash> {
  let collective_entry = Entry::App("collective".into(), collective.into());
  hdk::update_entry(collective_entry, &collective_hash)
}
struct CommitCollectiveResponse(EntryHash, Entry, Collective);
fn commit_collective(collective: Collective) -> ExternResult<CommitCollectiveResponse> {
  let collective_entry = Entry::App("collective".into(), collective.borrow().into());
  let collective_hash =
    t("commit_collective: ", hdk::commit_entry(&collective_entry))?;
  Ok(CommitCollectiveResponse(collective_hash, collective_entry, collective))
}
fn create_create_collective_action(
  collective_hash: &EntryHash,
  collective: &Collective,
) -> ExternResult<ActionEntry> {
  create_collective_action(
    collective_hash,
    ActionOp::CreateCollective,
    collective.into(),
    serde_json::value::Value::Null.into(),
    &"create_collective".into(),
    ActionStrategy::SystemAutomatic,
  )
}
fn add_collective_person(
  collective_hash: &EntryHash,
  person_hash: &EntryHash,
) -> ExternResult<EntryHash> {
  let collective_person_address =
    t("add_collective_person: ", hdk::link_entries(
      collective_hash,
      person_hash,
      "collective->person",
      &CollectivePersonTag::Creator.to_string(),
    ))?;
  create_add_collective_person_action(collective_hash, person_hash)?;
  Ok(collective_person_address)
}
#[derive(Clone, Serialize, Deserialize, SerializeBytes, Debug)]
struct AddCollectivePersonActionData {
  person_hash: EntryHash,
}
fn create_add_collective_person_action(
  collective_hash: &EntryHash,
  person_hash: &EntryHash,
) -> ExternResult<ActionEntry> {
  create_collective_action(
    collective_hash,
    ActionOp::AddCollectivePerson,
    AddCollectivePersonActionData {
      person_hash: person_hash.clone(),
    }.into(),
    serde_json::value::Value::Null.into(),
    &"add_collective_person".into(),
    ActionStrategy::SystemAutomatic,
  )
}
#[derive(Clone, Serialize, Deserialize, SerializeBytes, Debug)]
struct SetCollectiveNameActionData {
  name: String
}
fn create_set_collective_name_action(
  collective_hash: &EntryHash,
  name: &String,
  prev_name_opt: Option<&String>,
) -> ExternResult<ActionEntry> {
  create_collective_action(
    collective_hash,
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
#[derive(Clone, Serialize, Deserialize, SerializeBytes, Debug)]
struct SetTotalSharesActionData {
  total_shares: i64,
}
fn create_collective_action(
  collective_hash: &EntryHash,
  op: ActionOp,
  data: JsonString,
  prev_data: JsonString,
  tag: &String,
  strategy: ActionStrategy,
) -> ExternResult<ActionEntry> {
  let collective_action = Action {
    op,
    status: ActionStatus::Executed,
    data,
    prev_data,
    tag: tag.into(),
    strategy: strategy.into(),
  };
  let action_entry = create_entry(&collective_action)?;
  let action_hash = hash_entry(&collective_action)?;
  create_link(collective_hash.clone(), action_hash.clone(), LinkTag::from(tag))?;
  Ok((action_hash, action_entry, collective_action))
}
