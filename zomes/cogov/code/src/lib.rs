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
pub mod leger;
pub mod proposal;

use hdk_proc_macros::zome;
use std::borrow::Borrow;

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
	use holochain_wasm_utils::holochain_core_types::link::LinkMatch;

	use crate::collective::{
		Collective,
		commit_collective,
		CollectiveParams,
		CollectivePayload,
	};
	use crate::leger::Ledger;
	use crate::proposal::{
		Proposal,
		commit_proposal,
		ProposalParams,
		ProposalPayload
	};
	use crate::action::{
		Action,
		ActionsPayload,
		ActionStatus,
		ActionIntent,
	};

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
					"action",
					link_type: "collective_action",
					validation_package: || {
						hdk::ValidationPackageDefinition::Entry
					},
					validation: |_validation_data: hdk::LinkValidationData| {
						Ok(())
					}
				),
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

	#[entry_def]
	fn action_def() -> ValidatingEntryType {
		entry!(
			name: "action",
			description: "A cogov collective action",
			sharing: Sharing::Public,
			validation_package: || {
				hdk::ValidationPackageDefinition::Entry
			},
			validation: | _validation_data: hdk::EntryValidationData<Action>| {
				Ok(())
			},
			links: [
				from!(
					"collective",
					link_type: "action_collective",
					validation_package: || {
						hdk::ValidationPackageDefinition::Entry
					},
					validation: |_validation_data: hdk::LinkValidationData| {
						Ok(())
					}
				),
				to!(
					"action",
					link_type: "child_action",
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
	// curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "cogov", "function": "commit_collective", "args": { "collective": { "name": "Collective 0" } } }}' http://127.0.0.1:8888
	pub fn create_collective(collective: CollectiveParams) -> ZomeApiResult<CollectivePayload> {
		let (collective_address, _collective_entry, collective__) = commit_collective(
			Collective {
				name: collective.name,
			})?;
		let action = Action {
			op: "create_collective".into(),
			status: ActionStatus::Executed,
			data: (&collective__).into(),
			tag: "".into(),
			action_intent: ActionIntent::SystemAutomatic,
		};
		let action_entry = Entry::App("action".into(), action.borrow().into());
		let action_address = hdk::commit_entry(&action_entry)?;
		hdk::link_entries(
			&collective_address,
			&action_address,
			"collective_action",
			"create_collective",
		)?;
		Ok(CollectivePayload {
			collective_address,
			collective: collective__,
		})
	}

	#[zome_fn("hc_public")]
	// curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "cogov", "function": "get_collective", "args": { "collective_address": "addr" } }}' http://127.0.0.1:8888
	pub fn get_actions(collective_address: Address) -> ZomeApiResult<ActionsPayload> {
		let actions = hdk::utils::get_links_and_load_type(
			&collective_address,
			LinkMatch::Exactly("collective_action"),
			LinkMatch::Any,
		)?;
		Ok(ActionsPayload {
			collective_address,
			actions,
		})
	}

	#[zome_fn("hc_public")]
	// curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "cogov", "function": "get_collective", "args": { "collective_address": "addr" } }}' http://127.0.0.1:8888
	pub fn get_collective(collective_address: Address) -> ZomeApiResult<CollectivePayload> {
		let collective_address__ = collective_address.clone();
		let collective = hdk::utils::get_as_type(collective_address__)?;
		Ok(CollectivePayload {
			collective_address,
			collective,
		})
	}

	#[zome_fn("hc_public")]
	pub fn create_proposal(proposal: ProposalParams) -> ZomeApiResult<ProposalPayload> {
		let (proposal_address, _proposal_entry, proposal2) = commit_proposal(Proposal {
			name: proposal.name,
			content: proposal.content,
		})?;
		Ok(ProposalPayload {
			proposal_address,
			proposal: proposal2,
		})
	}
}
