// see https://developer.holochain.org/api/0.0.38-alpha14/hdk/ for info on using the hdk library
#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;
//#[macro_use]
//extern crate log;

pub mod collective;
pub mod leger;
pub mod proposal;

use hdk_proc_macros::zome;

#[zome]
mod cogov {
	use hdk::holochain_core_types::{
		entry::Entry,
		dna::entry_types::Sharing,
	};
	use hdk::holochain_persistence_api::{
		cas::content::Address
	};
	use hdk::prelude::{ValidatingEntryType, ZomeApiResult};

	use crate::collective::{
		Collective,
		commit_collective as commit_collective__impl,
		get_collective as get_collective__impl,
		CollectiveParams, CollectivePayload,
	};
	use crate::leger::Ledger;
	use crate::proposal::{Proposal, commit_proposal as commit_proposal__impl, ProposalParams, ProposalPayload};

	// collective
	#[entry_def]
	fn collective_def() -> ValidatingEntryType {
		entry!(
			name: "collective",
			description: "A cogov collective",
			sharing: Sharing::Public,
			validation_package: || {
				hdk::ValidationPackageDefinition::Entry
			},
			validation: | _validation_data: hdk::EntryValidationData<Collective>| {
				Ok(())
			},
			links: [
				to!(
					"ledger",
					link_type: "collective_ledger",
					validation_package: || {
						hdk::ValidationPackageDefinition::Entry
					},
					validation: |_validation_data: hdk::LinkValidationData| {
						Ok(())
					}
				)
			]
    )
	}

	// ledger
	#[entry_def]
	fn ledger_def() -> ValidatingEntryType {
		entry!(
			name: "ledger",
			description: "A cogov collective ledger",
			sharing: Sharing::Public,
			validation_package: || {
				hdk::ValidationPackageDefinition::Entry
			},
			validation: | _validation_data: hdk::EntryValidationData<Ledger>| {
				Ok(())
			}
    )
	}

	#[entry_def]
	fn proposal_def() -> ValidatingEntryType {
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

	#[init]
	fn init() -> Result<(), ()> {
		Ok(())
	}

	#[validate_agent]
	pub fn validate_agent(validation_data: hdk::EntryValidationData<AgentId>) -> Result<(), ()> {
		Ok(())
	}

	#[zome_fn("hc_public")]
	pub fn get_entry(address: Address) -> ZomeApiResult<Option<Entry>> {
		hdk::get_entry(&address)
	}

	#[zome_fn("hc_public")]
	pub fn test(collective: CollectiveParams) -> ZomeApiResult<Collective> {
		let collective2 = Collective {
			name: collective.name,
		};
		Ok(collective2)
	}

	#[zome_fn("hc_public")]
	// curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "cogov", "function": "commit_collective", "args": { "collective": { "name": "Collective 0" } } }}' http://127.0.0.1:8888
	pub fn commit_collective(collective: CollectiveParams) -> ZomeApiResult<CollectivePayload> {
		let (collective_address, _collective_entry, collective2) = commit_collective__impl(
			Collective {
				name: collective.name,
			})?;
		Ok(CollectivePayload {
			collective_address,
			collective: collective2,
		})
	}

	#[zome_fn("hc_public")]
	// curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "cogov", "function": "get_collective", "args": { "collective_address": "addr" } }}' http://127.0.0.1:8888
	pub fn get_collective(collective_address: Address) -> ZomeApiResult<CollectivePayload> {
		let collective_address__ = collective_address.clone();
		let collective = get_collective__impl(collective_address__)?;
		Ok(CollectivePayload {
			collective_address,
			collective,
		})
	}

	#[zome_fn("hc_public")]
	pub fn commit_proposal(proposal: ProposalParams) -> ZomeApiResult<ProposalPayload> {
		let (proposal_address, _proposal_entry, proposal2) = commit_proposal__impl(Proposal {
			name: proposal.name,
			content: proposal.content,
		})?;
		Ok(ProposalPayload {
			proposal_address,
			proposal: proposal2,
		})
	}
}
