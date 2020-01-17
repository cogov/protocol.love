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

pub mod action;
pub mod collective;
pub mod ledger;
pub mod proposal;

use hdk_proc_macros::zome;
//use std::borrow::Borrow;

#[zome]
mod cogov {
	use hdk::holochain_core_types::entry::Entry;
	use hdk::holochain_persistence_api::{
		cas::content::Address
	};
	use hdk::prelude::{ValidatingEntryType, ZomeApiResult};

	use crate::collective::{CollectivePayload, CollectiveParams};
	use crate::proposal::{ProposalParams, ProposalPayload};
	use crate::action::ActionsPayload;

	// collective
	#[entry_def]
	fn collective_def() -> ValidatingEntryType {
		crate::collective::collective_def()
	}

	#[entry_def]
	fn action_def() -> ValidatingEntryType {
		crate::action::action_def()
	}

	#[entry_def]
	fn ledger_def() -> ValidatingEntryType {
		crate::ledger::ledger_def()
	}

	#[entry_def]
	fn proposal_def() -> ValidatingEntryType {
		crate::proposal::proposal_def()
	}

	#[init]
	fn init() -> ZomeApiResult<()> {
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
	pub fn create_collective(collective: CollectiveParams) -> ZomeApiResult<CollectivePayload> {
		crate::collective::create_collective(collective)
	}

	// curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "cogov", "function": "get_collective", "args": { "collective_address": "addr" } }}' http://127.0.0.1:8888
	#[zome_fn("hc_public")]
	pub fn get_actions(collective_address: Address) -> ZomeApiResult<ActionsPayload> {
		crate::action::get_actions(collective_address)
	}

	// curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "cogov", "function": "get_collective", "args": { "collective_address": "addr" } }}' http://127.0.0.1:8888
	#[zome_fn("hc_public")]
	pub fn get_collective(collective_address: Address) -> ZomeApiResult<CollectivePayload> {
		crate::collective::get_collective(collective_address)
	}

	#[zome_fn("hc_public")]
	pub fn create_proposal(proposal: ProposalParams) -> ZomeApiResult<ProposalPayload> {
		crate::proposal::create_proposal(proposal)
	}
}
