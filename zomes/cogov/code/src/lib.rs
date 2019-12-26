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
		CollectiveParams,
	};
	use crate::leger::Ledger;
	use crate::proposal::{Proposal, commit_proposal as commit_proposal__impl};

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
			}
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
	pub fn commit_collective(collective: CollectiveParams) -> ZomeApiResult<Address> {
		let (_collective_entry, collective_address) = commit_collective__impl(
			Collective {
				name: collective.name,
			})?;
		Ok(collective_address)
	}

	#[zome_fn("hc_public")]
	pub fn commit_proposal(name: String, content: String) -> ZomeApiResult<Address> {
		commit_proposal__impl(Proposal {
			name,
			content,
		})
	}
}
