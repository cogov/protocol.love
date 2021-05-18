use hdk::prelude::*;
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
/// 	data: json!({"name": "My Collective", "admin_entry_hash": hdk::AGENT_ADDRESS.clone()}).into(),
/// 	prev_data: serde_json::value::Value::Null.into(),
/// 	tag: "create_collective".into(),
/// 	strategy: ActionStrategy::SystemAutomatic
/// }
/// ```
///
#[hdk_entry(id = "action")]
#[derive(Clone)]
pub struct Action {
  /// Represents each of the allowed operations
  pub op: ActionOp,
  /// Lifecycle Status of the Action
  pub status: ActionStatus,
  /// Action Data encoded as JSON.
  pub data: SerializedBytes,
  /// Previous Action Data encoded as JSON for undo purposes.
  pub prev_data: SerializedBytes,
  pub link_tag: LinkTag,
  /// How the Action was performed
  pub strategy: ActionStrategy,
}
/// An operation for an [Action](struct.Action.html).
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
pub enum ActionOp {
  CreateCollective,
  AddCollectivePerson,
  SetCollectiveName,
}
/// The lifecycle status of an [Action](struct.Action.html).
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
pub enum ActionStatus {
  /// Action is currently opened & not completed
  Open,
  /// Action is executed & completed
  ///
  /// TODO: Rename
  Executed,
}
/// How an [Action](struct.Action.html) is performed.
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
pub enum ActionStrategy {
  /// Performed via automation by the system
  SystemAutomatic,
  /// TODO: Evaluate
  PrivilegedAction,
  /// TODO: Evaluate
  NewDiscussionMessage,
}
/// An api payload returning [Actions](struct.Action.html) & the `collective_entry_hash`.
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
pub struct ActionsPayload {
  pub collective_entry_hash: EntryHash,
  pub action_entry_hash_1d: Vec<EntryHash>,
}
/// A tuple containing an [EntryHash](type.EntryHash.html), [Entry](enum.Entry.html), & [Action](struct.Action.html)
pub type ActionEntry = (EntryHash, HeaderHash, Action);
pub trait RootAction {
  fn commit_action(self, collective_entry_hash: EntryHash) -> ExternResult<ActionEntry>;
}
pub trait ChildAction {
  fn commit_action(self, collective_entry_hash: EntryHash, parent_action_address: EntryHash) -> ExternResult<ActionEntry>;
}
impl RootAction for Action {
  fn commit_action(self, collective_entry_hash: EntryHash) -> ExternResult<ActionEntry> {
    let action_entry_hash = hash_entry(&self)?;
    let action_header_hash = create_entry(&self)?;
    create_link(
      collective_entry_hash, action_entry_hash.clone(), CollectiveActionTag::tag(),
    )?;
    Ok((action_entry_hash, action_header_hash, self))
  }
}
pub(crate) struct ParentActionChildActionTag;
impl ParentActionChildActionTag {
  const TAG: &'static [u8; 26] = b"parent_action_child_action";
  /// Create the tag
  pub(crate) fn tag() -> LinkTag {
    LinkTag::new(*Self::TAG)
  }
}
impl ChildAction for Action {
  fn commit_action(
    self, collective_entry_hash: EntryHash, parent_action_hash: EntryHash,
  ) -> ExternResult<ActionEntry> {
    let action_entry_hash = hash_entry(&self)?;
    let action_header_hash = create_entry(&self)?;
    create_link(
      collective_entry_hash, action_entry_hash.clone(), CollectiveActionTag::tag(),
    )?;
    create_link(
      parent_action_hash, action_entry_hash.clone(), ParentActionChildActionTag::tag(),
    )?;
    Ok((action_entry_hash, action_header_hash, self))
  }
}
/// Get an [ActionsPayload](struct.ActionsPayload.html) of all of the [Actions](struct.Action.html)
/// linked to the [Collective](struct.Collective.html).
///
/// # Test:
/// ```
/// curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "protocol-love", "function": "get_collective", "args": { "collective_entry_hash": "addr" } }}' http://127.0.0.1:8888
/// ```
pub fn get_actions(collective_entry_hash: EntryHash) -> ExternResult<ActionsPayload> {
  let links = get_links(
    collective_entry_hash.clone(), Some(CollectiveActionTag::tag()),
  )?.into_inner();
  let action_entry_hash_1d =
    links.into_iter().map(|link| link.target).rev().collect();
  Ok(ActionsPayload {
    collective_entry_hash: collective_entry_hash.clone(),
    action_entry_hash_1d,
  })
}
pub(crate) struct CollectiveActionTag;
impl CollectiveActionTag {
  const TAG: &'static [u8; 17] = b"collective_action";
  /// Create the tag
  pub(crate) fn tag() -> LinkTag {
    LinkTag::new(*Self::TAG)
  }
}
