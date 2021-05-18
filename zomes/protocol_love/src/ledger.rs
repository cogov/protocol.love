use hdk::prelude::*;
use crate::collective::{Collective};
use crate::utils::t;
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
pub struct CreateCollectiveLedgerParams {
  pub collective: Collective,
  pub collective_entry_hash: EntryHash,
}
pub(crate) struct CollectiveLedgerTag;
impl CollectiveLedgerTag {
  const TAG: &'static [u8; 17] = b"collective_ledger";
  /// Create the tag
  pub(crate) fn tag() -> LinkTag {
    LinkTag::new(*Self::TAG)
  }
}
/// Create & commit a [Ledger](struct.Ledger.html) for a [Collective](struct.Collective.html).
#[hdk_extern]
pub fn create_collective_ledger(params: CreateCollectiveLedgerParams) -> ExternResult<HeaderHash> {
  let CreateCollectiveLedgerParams {
    collective,
    collective_entry_hash,
  } = params;
  let ledger_name =
    format!("Primary Ledger for {}", collective.name).to_string();
  let ledger = Ledger {
    name: ledger_name,
    ..Default::default()
  };
  let ledger_entry_hash = hash_entry(ledger.clone())?;
  let ledger_header_hash =
    t(commit_ledger(ledger), "create_collective_ledger: ")?;
  t(create_link(
    collective_entry_hash,
    ledger_entry_hash,
    CollectiveLedgerTag::tag(),
  ), "create_collective_ledger: collective->ledger: ")?;
  Ok(ledger_header_hash)
}
fn commit_ledger(ledger: Ledger) -> ExternResult<HeaderHash> {
  let ledger_address = t(create_entry(&ledger), "commit_ledger: ")?;
  Ok(ledger_address)
}
/// A ledger to account for transactions relating to a [Collective](struct.Collective.html).
#[hdk_entry]
#[derive(Clone)]
pub struct Ledger {
  pub name: String,
}
impl Default for Ledger {
  fn default() -> Self {
    Ledger {
      name: "unnamed ledger".to_string(),
    }
  }
}
