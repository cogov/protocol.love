use hdk::holochain_json_api::{
	json::JsonString,
	error::JsonError,
};
use hdk::holochain_core_types::dna::entry_types::Sharing;
use holochain_wasm_utils::holochain_persistence_api::cas::content::EntryHash;
use holochain_wasm_utils::holochain_core_types::entry::Entry;
use std::borrow::Borrow;
use hdk::error::ExternResult;
use hdk::prelude::ValidatingEntryType;

/// Api params for [create_proposal](fn.create_proposal.html).
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
pub struct ProposalParams {
	/// Name of the proposal.
	pub name: String,
	/// Text content of the proposal.
	pub content: String,
}

/// A proposal to change the collective.
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
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

/// Api payload for a [Proposal](struct.Proposal.html)
/// returned by [create_proposal](fn.create_proposal.html).
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
pub struct ProposalPayload {
	pub proposal_address: EntryHash,
	pub proposal: Proposal,
}

/// Returns a Holochain entry definition for a proposal.
pub fn proposal_def() -> ValidatingEntryType {
	entry!(
		name: "proposal",
		description: "A pro",
		sharing: Sharing::Public,
		validation_package: || {
			hdk::ValidationPackageDefinition::Entry
		},
		validation: | _validation_data: hdk::EntryValidationData<Proposal>| {
			Ok(())
		}
	)
}

/// Api to create & commit a [Proposal](struct.Proposal.html).
pub fn create_proposal(proposal_params: ProposalParams) -> ExternResult<ProposalPayload> {
	let (proposal_address, _proposal_entry, proposal2) =
		commit_proposal(Proposal {
			name: proposal_params.name,
			content: proposal_params.content,
		})?;
	Ok(ProposalPayload {
		proposal_address,
		proposal: proposal2,
	})
}

fn commit_proposal(proposal: Proposal) -> ExternResult<(EntryHash, Entry, Proposal)> {
	let proposal_entry = Entry::App("proposal".into(), proposal.borrow().into());
	let proposal_address = hdk::commit_entry(&proposal_entry)?;
	Ok((proposal_address, proposal_entry, proposal))
}
