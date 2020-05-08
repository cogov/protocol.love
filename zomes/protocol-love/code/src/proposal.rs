use hdk::holochain_json_api::{
	json::JsonString,
	error::JsonError,
};
use hdk::holochain_core_types::dna::entry_types::Sharing;
use holochain_wasm_utils::holochain_persistence_api::cas::content::Address;
use holochain_wasm_utils::holochain_core_types::entry::Entry;
use std::borrow::Borrow;
use hdk::error::ZomeApiResult;
use hdk::prelude::ValidatingEntryType;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct ProposalParams {
	pub name: String,
	pub content: String,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Proposal {
	pub name: String,
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

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct ProposalPayload {
	pub proposal_address: Address,
	pub proposal: Proposal,
}

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

pub fn create_proposal(proposal_params: ProposalParams) -> ZomeApiResult<ProposalPayload> {
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

fn commit_proposal(proposal: Proposal) -> ZomeApiResult<(Address, Entry, Proposal)> {
	let proposal_entry = Entry::App("proposal".into(), proposal.borrow().into());
	let proposal_address = hdk::commit_entry(&proposal_entry)?;
	Ok((proposal_address, proposal_entry, proposal))
}
