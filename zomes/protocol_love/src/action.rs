use hdk::holochain_json_api::{
  json::JsonString,
  error::JsonError,
};
use hdk::holochain_core_types::dna::entry_types::Sharing;
use holochain_wasm_utils::holochain_persistence_api::cas::content::EntryHash;
use holochain_wasm_utils::holochain_core_types::entry::Entry;
use std::borrow::Borrow;
use hdk::prelude::*;
use holochain_wasm_utils::holochain_core_types::link::LinkMatch;
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
/// 	data: json!({"name": "My Collective", "admin_hash": hdk::AGENT_ADDRESS.clone()}).into(),
/// 	prev_data: serde_json::value::Value::Null.into(),
/// 	tag: "create_collective".into(),
/// 	strategy: ActionStrategy::SystemAutomatic
/// }
/// ```
///
#[derive(Clone, Serialize, Deserialize, SerializeBytes, Debug)]
pub struct Action {
  /// Represents each of the allowed operations
  pub op: ActionOp,
  /// Lifecycle Status of the Action
  pub status: ActionStatus,
  /// Action Data encoded as JSON.
  pub data: JsonString,
  /// Previous Action Data encoded as JSON for undo purposes.
  pub prev_data: JsonString,
  pub tag: String,
  /// How the Action was performed
  pub strategy: ActionStrategy,
}
/// An operation for an [Action](struct.Action.html).
#[derive(Clone, Serialize, Deserialize, SerializeBytes, Debug)]
pub enum ActionOp {
  CreateCollective,
  AddCollectivePerson,
  SetCollectiveName,
}
/// The lifecycle status of an [Action](struct.Action.html).
#[derive(Clone, Serialize, Deserialize, SerializeBytes, Debug)]
pub enum ActionStatus {
  /// Action is currently opened & not completed
  Open,
  /// Action is executed & completed
  ///
  /// TODO: Rename
  Executed,
}
/// How an [Action](struct.Action.html) is performed.
#[derive(Clone, Serialize, Deserialize, SerializeBytes, Debug)]
pub enum ActionStrategy {
  /// Performed via automation by the system
  SystemAutomatic,
  /// TODO: Evaluate
  PrivilegedAction,
  /// TODO: Evaluate
  NewDiscussionMessage,
}
/// An api payload returning [Actions](struct.Action.html) & the `collective_hash`.
#[derive(Clone, Serialize, Deserialize, SerializeBytes, Debug)]
pub struct ActionsPayload {
  pub collective_hash: EntryHash,
  pub actions: Vec<Action>,
}
/// A tuple containing an [EntryHash](type.EntryHash.html), [Entry](enum.Entry.html), & [Action](struct.Action.html)
pub type ActionEntry = (EntryHash, Entry, Action);
pub trait RootAction {
  fn commit_action(self, collective_hash: EntryHash) -> ExternResult<ActionEntry>;
}
pub trait ChildAction {
  fn commit_action(self, collective_hash: EntryHash, parent_action_address: EntryHash) -> ExternResult<ActionEntry>;
}
impl RootAction for Action {
  fn commit_action(self, collective_hash: EntryHash) -> ExternResult<ActionEntry> {
    let action_entry = Entry::App("action".into(), self.borrow().into());
    let action_hash = hdk::commit_entry(&action_entry)?;
    create_link(
      collective_hash, action_hash, collective_action_tag,
    )?;
    Ok((action_hash, action_entry, self))
  }
}
impl ChildAction for Action {
  fn commit_action(self, collective_hash: EntryHash, parent_action_hash: EntryHash) -> ExternResult<ActionEntry> {
    let action_entry = Entry::App("action".into(), self.borrow().into());
    let action_hash = hdk::commit_entry(&action_entry)?;
    create_link(
      collective_hash, action_hash, collective_action_tag,
    )?;
    let parent_action_child_action = LinkTag::from("parent_action_child_action");
    create_link(
      parent_action_hash, action_hash, parent_action_child_action,
    )?;
    Ok((action_hash, action_entry, self))
  }
}
/// Get an [ActionsPayload](struct.ActionsPayload.html) of all of the [Actions](struct.Action.html)
/// linked to the [Collective](struct.Collective.html).
///
/// # Test:
/// ```
/// curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "protocol-love", "function": "get_collective", "args": { "collective_hash": "addr" } }}' http://127.0.0.1:8888
/// ```
pub fn get_actions(collective_hash: EntryHash) -> ExternResult<ActionsPayload> {
  let mut links = get_links(
    collective_hash, Some(collective_action_tag),
  )?;
  let actions = links.into_iter().map(|link| {
    get(link, { strategy: GetStrategy::Latest })
  }).rev()?;
  Ok(ActionsPayload {
    collective_hash: collective_hash.clone(),
    actions,
  })
}
pub const collective_action_tag: LinkTag = LinkTag::from("collective_action");
