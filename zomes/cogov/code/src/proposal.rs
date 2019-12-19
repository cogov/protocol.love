use time::{Tm, Timespec};
use hdk::holochain_json_api::{
	json::JsonString,
	error::JsonError,
};
use holochain_wasm_utils::holochain_persistence_api::cas::content::Address;
use hdk::prelude::ZomeApiResult;
use holochain_wasm_utils::holochain_core_types::entry::Entry;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct ProposalParams {
	pub name: String,
	pub content: String,
	pub created_at: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Proposal {
	pub name: String,
	pub content: String,
	pub created_at_sec: i64,
}

impl Proposal {
	#[allow(dead_code)]
	fn created_at(&self) -> Tm {
		time::at(Timespec::new(self.created_at_sec, 0))
	}
}

impl Default for Proposal {
	fn default() -> Self {
		Proposal {
			name: "unnamed proposal".to_string(),
			content: "".to_string(),
			created_at_sec: time::now_utc().to_timespec().sec,
		}
	}
}

//	#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
//	pub struct ProposalCreate {
//		name: String,
//		content: String,
//		created?: SystemTime,
//	}

pub fn commit_proposal(proposal_params: ProposalParams) -> ZomeApiResult<Address> {
	let proposal = if proposal_params.created_at.is_some() {
		Proposal {
			name: proposal_params.name,
			content: proposal_params.content,
			created_at_sec: proposal_params.created_at.unwrap(),
		}
	} else {
		Proposal {
			name: proposal_params.name,
			content: proposal_params.content,
			..Default::default()
		}
	};
	let proposal_entry = Entry::App("proposal".into(), proposal.into());
	let proposal_address = hdk::commit_entry(&proposal_entry)?;
	Ok(proposal_address)
}
