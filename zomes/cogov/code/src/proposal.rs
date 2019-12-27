use hdk::holochain_json_api::{
	json::JsonString,
	error::JsonError,
};
use holochain_wasm_utils::holochain_persistence_api::cas::content::Address;
use hdk::prelude::ZomeApiError;
use holochain_wasm_utils::holochain_core_types::entry::Entry;

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

pub fn commit_proposal(proposal: Proposal) -> Result<(Address, Entry), ZomeApiError> {
	let proposal_entry = Entry::App("proposal".into(), proposal.into());
	let proposal_address = hdk::commit_entry(&proposal_entry)?;
	Ok((proposal_address, proposal_entry))
}
