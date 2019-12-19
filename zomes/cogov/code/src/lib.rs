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

mod collective;
mod leger;
mod proposal;

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

	use crate::collective::{Collective, commit_collective as commit_collective__impl};
	use crate::leger::Ledger;
	use crate::proposal::{Proposal, commit_proposal as commit_proposal__impl};

	#[init]
	fn init() -> Result<(), ()> {
		Ok(())
	}

	#[validate_agent]
	pub fn validate_agent(validation_data: hdk::EntryValidationData<AgentId>) -> Result<(), ()> {
		Ok(())
	}

	#[zome_fn("hc_public")]
	fn get_entry(address: Address) -> ZomeApiResult<Option<Entry>> {
		hdk::get_entry(&address)
	}

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

	#[zome_fn("hc_public")]
	fn commit_collective(collective: Collective) -> ZomeApiResult<Address> {
		commit_collective__impl(collective)
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

	#[zome_fn("hc_public")]
	pub fn commit_proposal(proposal: Proposal) -> ZomeApiResult<Address> {
		commit_proposal__impl(proposal)
	}
}
