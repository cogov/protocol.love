use hdk::prelude::*;
/// A proposal to change the collective.
#[hdk_entry]
#[derive(Clone)]
pub struct Proposal {
  /// Name of the proposal
  pub name: String,
  /// Text content of the proposal.
  pub content: String,
}
impl Default for Proposal {
  fn default() -> Self {
    Proposal {
      name: "unnamed proposal".to_string(),
      content: "".to_string(),
    }
  }
}
/// Api params for [create_proposal](fn.create_proposal.html).
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
pub struct ProposalParams {
  /// Name of the proposal.
  pub name: String,
  /// Text content of the proposal.
  pub content: String,
}
/// Api payload for a [Proposal](struct.Proposal.html)
/// returned by [create_proposal](fn.create_proposal.html).
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
pub struct ProposalPayload {
  pub proposal_header_hash: HeaderHash,
  pub proposal_entry_hash: EntryHash,
  pub proposal: Proposal,
}
/// Api to create & commit a [Proposal](struct.Proposal.html).
pub fn create_proposal(proposal_params: ProposalParams) -> ExternResult<ProposalPayload> {
  let (proposal_header_hash, proposal_entry_hash, proposal) =
    commit_proposal(Proposal {
      name: proposal_params.name,
      content: proposal_params.content,
    })?;
  Ok(ProposalPayload {
    proposal_header_hash,
    proposal_entry_hash,
    proposal,
  })
}
fn commit_proposal(proposal: Proposal) -> ExternResult<(HeaderHash, EntryHash, Proposal)> {
  let proposal_header_hash = create_entry(proposal.clone())?;
  Ok((proposal_header_hash, hash_entry(proposal.clone())?, proposal))
}
