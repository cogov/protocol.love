// use std::borrow::Borrow;
use hdk::prelude::*;
use holochain_serialized_bytes::SerializedBytes;
use crate::utils::t;
use crate::person::{Person, PersonStatus, create_person, PersonParams, PersonPayload};
use crate::ledger::{CreateCollectiveLedgerParams, create_collective_ledger};
use std::fmt;
use crate::action::{Action, ActionOp, ActionStrategy, ActionEntry, ActionStatus};
/// A collective.
///
/// Has a name & an optional admin_entry_hash.
#[hdk_entry(id = "Collective")]
#[derive(Clone)]
pub struct Collective {
  /// Name of the Collective
  pub name: String,
  pub agent_initial_pubkey: AgentPubKey,
  /// Administrator address of the collective.
  ///
  /// Used during the initial creation of the collective.
  ///
  /// TODO: Move to a link or a [CollectivePersonTag](enum.CollectivePersonTag.html)
  pub admin_entry_hash: Option<EntryHash>,
}
/// Api params to create a [Collective](struct.Collective.html) along with an optional `admin_entry_hash`.
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
pub struct CreateCollectiveParams {
  pub name: String,
  pub agent_initial_pubkey: Option<AgentPubKey>,
  pub admin_entry_hash: Option<EntryHash>,
}
/// Api payload containing a collective_entry_hash & collective.
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
pub struct CollectivePayload {
  pub collective_entry_hash: EntryHash,
  pub collective: Collective,
}
/// Tag of a [Person](struct.Person.html) in a [Collective](struct.Collective.html).
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
pub enum CollectivePersonRole {
  /// Creator of the [Collective](struct.Collective.html)
  Creator,
}
impl fmt::Display for CollectivePersonRole {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}
/// Api Payload of People in a Collective.
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
pub struct CollectivePeoplePayload {
  pub collective_entry_hash: EntryHash,
  /// [People](struct.Person.html) in a [Collective](struct.Collective.html).
  pub collective_person_entry_hash_1d: Vec<EntryHash>,
}
#[hdk_extern]
fn validate_collective(data: ValidateData) -> ExternResult<ValidateCallbackResult> {
  get_validate_collective(
    data,
    |_collective| Ok(ValidateCallbackResult::Valid),
  )
}
#[hdk_extern]
fn validate_create_collective(data: ValidateData) -> ExternResult<ValidateCallbackResult> {
  get_validate_collective(
    data,
    |collective| validate_upsert_collective(collective),
  )
}
#[hdk_extern]
fn validate_update_collective(data: ValidateData) -> ExternResult<ValidateCallbackResult> {
  get_validate_collective(
    data,
    |collective| validate_upsert_collective(collective),
  )
}
#[hdk_extern]
fn validate_delete_collective(_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
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
  let Collective {
    admin_entry_hash,
    agent_initial_pubkey: _agent_initial_pubkey,
    name: _name,
  } = collective;
  match admin_entry_hash {
    Some(some_admin_entry_hash) => {
      if let Some(element) = get(some_admin_entry_hash.clone(), GetOptions::content())? {
        let option_content: Option<Person> = element.entry().to_app_option()?;
        if let Some(admin) = option_content {
          if admin.agent_initial_pubkey == *element.header().author() {
            return Ok(ValidateCallbackResult::Valid);
          } else {
            return Ok(ValidateCallbackResult::Invalid(
              "Collective must be modified with same agent as the given person".into())
            );
          };
        }
      }
    }
    None => return Ok(ValidateCallbackResult::Invalid("No admin_entry_hash".into()))
  }
  return Ok(ValidateCallbackResult::Invalid("No admin".into()));
}
fn create_collective_admin(name: String) -> ExternResult<PersonPayload> {
  create_person(PersonParams {
    collective_entry_hash: None,
    name: name.clone(),
    status: PersonStatus::default(),
  })
}
/// Api function to create & commit a [Collective](struct.Collective.html) along with the admin.
///
/// The optional admin_entry_hash defaults to the `hdk::AGENT_ADDRESS`.
///
/// # Test:
///
/// curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "protocol-love", "function": "create_collective", "args": { "collective": { "name": "Collective 0" } } }}' http://127.0.0.1:8888
#[hdk_extern]
fn create_collective(collective_params: CreateCollectiveParams) -> ExternResult<CollectivePayload> {
  let CreateCollectiveParams {
    admin_entry_hash,
    agent_initial_pubkey: _agent_initial_pubkey,
    name,
  } = collective_params;
  // TODO: Set name when answered: https://forum.holochain.org/t/writing-a-validation-rule-that-checks-the-entry-author-against-the-data-being-added-the-entry/1545/14?u=btakita
  let admin_payload = match admin_entry_hash.clone() {
    Some(admin_entry_hash) => {
      // get(admin_entry_hash, GetOptions::content())?
      // if let Some(element) = get(admin_entry_hash, GetOptions::content())? {
      //   let option_admin: Option<Person> = element.entry().to_app_option()?;
      //   if let Some(admin) = option_admin {
      //     admin
      //   }
      // };
      match get(admin_entry_hash, GetOptions::content())? {
        Some(element) => {
          let option_admin = element.entry().to_app_option()?;
          match option_admin {
            Some(admin) => admin,
            None => create_collective_admin(name.clone())?
          }
        }
        None => create_collective_admin(name.clone())?
      }
    }
    None => create_collective_admin(name.clone())?
  };
  let some_agent_initial_pubkey = admin_payload.person.agent_initial_pubkey;
  let collective = Collective {
    name,
    agent_initial_pubkey: some_agent_initial_pubkey.clone(),
    admin_entry_hash: admin_entry_hash.clone(),
  };
  let collective_entry_hash = hash_entry(collective.clone())?;
  let collective_name = collective.clone().name;
  t(create_entry(
    Collective {
      name: collective_name.clone(),
      agent_initial_pubkey: some_agent_initial_pubkey,
      admin_entry_hash: admin_entry_hash.clone(),
    }), "create_collective: ")?;
  t(create_create_collective_action(
    collective_entry_hash.clone(),
    collective.clone(),
  ), "create_collective: ")?;
  t(create_collective_ledger(CreateCollectiveLedgerParams {
    collective: collective.clone(),
    collective_entry_hash: collective_entry_hash.clone(),
  }), "create_collective: ")?;
  t(create_set_collective_name_action(
    collective_entry_hash.clone(),
    collective_name,
    None,
  ), "create_collective: ")?;
  if let Some(admin_entry_hash) = admin_entry_hash {
    t(add_collective_person(
      collective_entry_hash.clone(),
      admin_entry_hash,
    ), "create_collective: ")?;
  };
  Ok(CollectivePayload {
    collective_entry_hash,
    collective: collective.clone(),
  })
}
/// Api to get the [Collective](struct.Collective.html).
///
/// # Test:
///
/// curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "protocol-love", "function": "get_collective", "args": { "collective_entry_hash": "addr" } }}' http://127.0.0.1:8888
#[hdk_extern]
pub fn get_collective(collective_entry_hash: EntryHash) -> ExternResult<CollectivePayload> {
  if let Some(element) = get(collective_entry_hash.clone(), GetOptions::content())? {
    let option_content: Option<Collective> = element.entry().to_app_option()?;
    if let Some(collective) = option_content {
      return Ok(CollectivePayload {
        collective_entry_hash,
        collective,
      });
    }
  }
  Err(WasmError::Guest("collective hash not found".into()))
}
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
pub struct SetCollectiveNameParams {
  pub collective_header_hash: HeaderHash,
  pub name: String,
}
/// Api to set the [Collective](struct.Collective.html) name.
#[hdk_extern]
pub fn set_collective_name(params: SetCollectiveNameParams) -> ExternResult<CollectivePayload> {
  let collective_header_hash = params.collective_header_hash;
  let name = params.name;
  if let Some(element) = get(collective_header_hash.clone(), GetOptions::content())? {
    let option_content: Option<Collective> = element.entry().to_app_option()?;
    if let Some(saved_collective) = option_content {
      let collective = Collective {
        name: name.clone(),
        ..saved_collective
      };
      update_collective(UpdateCollectiveParams {
        collective_header_hash,
        collective: collective.clone(),
      })?;
      let collective_entry_hash = t(hash_entry(&collective), "set_collective_name: ")?;
      create_set_collective_name_action(
        collective_entry_hash.clone(),
        name,
        Some(saved_collective.name),
      )?;
      return Ok(CollectivePayload {
        collective_entry_hash: collective_entry_hash.clone(),
        collective,
      });
    }
  }
  Err(WasmError::Guest("Could not find collective".into()))
}
pub(crate) struct CollectivePersonTag;
impl CollectivePersonTag {
  const TAG: &'static [u8; 17] = b"collective_person";
  /// Create the tag
  pub(crate) fn tag() -> LinkTag {
    LinkTag::new(*Self::TAG)
  }
}
/// Api to get the [People](struct.Person.html) in the [Collective](struct.Collective.html).
#[hdk_extern]
pub fn get_collective_people(collective_entry_hash: EntryHash) -> ExternResult<CollectivePeoplePayload> {
  let links = get_links(
    collective_entry_hash.clone(),
    Some(CollectivePersonTag::tag()),
  )?.into_inner();
  let collective_person_entry_hash_1d = links.into_iter().map(|l| l.target).collect();
  Ok(CollectivePeoplePayload {
    collective_entry_hash,
    collective_person_entry_hash_1d,
  })
}
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
struct UpdateCollectiveParams {
  collective_header_hash: HeaderHash,
  collective: Collective,
}
#[hdk_extern]
fn update_collective(params: UpdateCollectiveParams) -> ExternResult<HeaderHash> {
  let UpdateCollectiveParams {
    collective_header_hash,
    collective
  } = params;
  update(collective_header_hash, EntryWithDefId::new(
    EntryDefId::App("collective".into()),
    Entry::App(
      AppEntryBytes::try_from(
        SerializedBytes::try_from(collective).unwrap()
      ).unwrap()),
  ))
}
fn create_create_collective_action(
  collective_entry_hash: EntryHash,
  collective: Collective,
) -> ExternResult<ActionEntry> {
  create_collective_action(
    collective_entry_hash,
    ActionOp::CreateCollective,
    collective.try_into().unwrap(),
    ().try_into().unwrap(),
    CreateCollectiveTag::tag(),
    ActionStrategy::SystemAutomatic,
  )
}
pub(crate) struct CreateCollectiveTag;
impl CreateCollectiveTag {
  const TAG: &'static [u8; 17] = b"create_collective";
  /// Create the tag
  pub(crate) fn tag() -> LinkTag {
    LinkTag::new(*Self::TAG)
  }
}
pub(crate) struct AddCollectivePersonTag;
impl AddCollectivePersonTag {
  const TAG: &'static [u8; 21] = b"add_collective_person";
  /// Create the tag
  pub(crate) fn tag() -> LinkTag {
    LinkTag::new(*Self::TAG)
  }
}
fn add_collective_person(
  collective_entry_hash: EntryHash,
  person_entry_hash: EntryHash,
) -> ExternResult<HeaderHash> {
  let collective_person_address =
    t(create_link(
      collective_entry_hash.clone(),
      person_entry_hash.clone(),
      AddCollectivePersonTag::tag(),
    ), "add_collective_person: ")?;
  create_add_collective_person_action(collective_entry_hash, person_entry_hash)?;
  Ok(collective_person_address)
}
#[hdk_entry(id = "AddCollectivePersonActionData")]
#[derive(Clone)]
struct AddCollectivePersonActionData {
  person_entry_hash: EntryHash,
}
pub(crate) struct AddCollectiveNameTag;
impl AddCollectiveNameTag {
  const TAG: &'static [u8; 19] = b"add_collective_name";
  /// Create the tag
  pub(crate) fn tag() -> LinkTag {
    LinkTag::new(*Self::TAG)
  }
}
fn create_add_collective_person_action(
  collective_entry_hash: EntryHash,
  person_entry_hash: EntryHash,
) -> ExternResult<ActionEntry> {
  create_collective_action(
    collective_entry_hash,
    ActionOp::AddCollectivePerson,
    AddCollectivePersonActionData {
      person_entry_hash: person_entry_hash.clone(),
    }.try_into().unwrap(),
    ().try_into().unwrap(),
    AddCollectiveNameTag::tag(),
    ActionStrategy::SystemAutomatic,
  )
}
#[hdk_entry(id = "SetCollectiveNameActionData")]
#[derive(Clone)]
struct SetCollectiveNameActionData {
  name: String,
}
pub(crate) struct SetCollectiveNameTag;
impl SetCollectiveNameTag {
  const TAG: &'static [u8; 19] = b"set_collective_name";
  /// Create the tag
  pub(crate) fn tag() -> LinkTag {
    LinkTag::new(*Self::TAG)
  }
}
fn create_set_collective_name_action(
  collective_entry_hash: EntryHash,
  name: String,
  prev_name_opt: Option<String>,
) -> ExternResult<ActionEntry> {
  create_collective_action(
    collective_entry_hash,
    ActionOp::SetCollectiveName,
    SetCollectiveNameActionData {
      name: name.clone()
    }.try_into().unwrap(),
    match prev_name_opt {
      Some(prev_name) => SetCollectiveNameActionData {
        name: prev_name.clone(),
      }.try_into().unwrap(),
      None => ().try_into().unwrap()
    },
    SetCollectiveNameTag::tag(),
    ActionStrategy::SystemAutomatic,
  )
}
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
struct SetTotalSharesActionData {
  total_shares: i64,
}
fn create_collective_action(
  collective_entry_hash: EntryHash,
  op: ActionOp,
  data: SerializedBytes,
  prev_data: SerializedBytes,
  link_tag: LinkTag,
  strategy: ActionStrategy,
) -> ExternResult<ActionEntry> {
  let collective_action = Action {
    op,
    status: ActionStatus::Executed,
    data,
    prev_data,
    link_tag: link_tag.clone(),
    strategy: strategy.into(),
  };
  let action_entry = create_entry(&collective_action)?;
  let action_hash = hash_entry(&collective_action)?;
  create_link(
    collective_entry_hash.clone(),
    action_hash.clone(),
    link_tag,
  )?;
  Ok((action_hash, action_entry, collective_action))
}
