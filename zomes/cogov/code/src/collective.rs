use std::borrow::Borrow;
use time::{Tm, Timespec};
use hdk::holochain_json_api::{
	json::JsonString,
	error::JsonError,
};
use crate::leger::create_collective_ledger;
use holochain_wasm_utils::holochain_persistence_api::cas::content::Address;
use hdk::prelude::ZomeApiResult;
use holochain_wasm_utils::holochain_core_types::entry::Entry;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Collective {
	pub name: String,
	pub created_at_sec: i64,
}

impl Collective {
	#[allow(dead_code)]
	fn created_at(&self) -> Tm {
		time::at(Timespec::new(self.created_at_sec, 0))
	}
}

impl Default for Collective {
	fn default() -> Self {
		Collective {
			name: "unnamed collective".to_string(),
			created_at_sec: time::now_utc().to_timespec().sec,
		}
	}
}

pub fn commit_collective(collective: Collective) -> ZomeApiResult<Address> {
	let collective_entry = Entry::App("collective".into(), collective.borrow().into());
	let collective_address = hdk::commit_entry(&collective_entry)?;
	create_collective_ledger(&collective, &collective_address)?;
	Ok(collective_address)
}
