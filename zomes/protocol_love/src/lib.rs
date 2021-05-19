// use hdk::holochain_core_types::entry::Entry;
// use hdk::holochain_persistence_api::{
// 	cas::content::EntryHash
// };
// use hdk::prelude::*;
// use crate::collective::{CollectivePeoplePayload};
// use crate::proposal::{ProposalParams, ProposalPayload};
// use crate::action::ActionsPayload;
pub mod action;
pub mod collective;
pub mod ledger;
pub mod person;
pub mod proposal;
pub mod utils;
// collective
// #[init]
// fn init() -> ExternResult<()> {
//   Ok(())
// }
//#[hdk_extern]
// pub fn get_entry(address: EntryHash) -> ExternResult<Option<Entry>> {
//   hdk::get_entry(&address)
// }
// #[hdk_extern]
// pub fn get_collective_people(
//   collective_entry_hash: EntryHash
// ) -> ExternResult<CollectivePeoplePayload> {
//   crate::collective::get_collective_people(collective_entry_hash)
// }
// // curl -X POST -H "Content-Type: application/json" -d '{"id": "0", "jsonrpc": "2.0", "method": "call", "params": {"instance_id": "test-instance", "zome": "protocol-love", "function": "get_collective", "args": { "collective_entry_hash": "addr" } }}' http://127.0.0.1:8888
// #[hdk_extern]
// pub fn get_actions(collective_entry_hash: EntryHash) -> ExternResult<ActionsPayload> {
//   crate::action::get_actions(collective_entry_hash)
// }
// #[hdk_extern]
// pub fn create_proposal(proposal: ProposalParams) -> ZomeApiResult<ProposalPayload> {
//   crate::proposal::create_proposal(proposal)
// }
