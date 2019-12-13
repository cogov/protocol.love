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

use hdk_proc_macros::zome;

#[zome]
mod cogov {
	use hdk::holochain_core_types::{
		entry::Entry,
		dna::entry_types::Sharing,
	};
	use hdk::holochain_json_api::{
		json::JsonString,
		error::JsonError,
	};
	use hdk::holochain_persistence_api::{
		cas::content::Address
	};
	use hdk::prelude::{ValidatingEntryType, ZomeApiResult};

	use std::time::SystemTime;

	#[init]
	fn init() -> Result<(), ()> {
		Ok(())
	}

	#[validate_agent]
	pub fn validate_agent(validation_data: EntryValidationData<AgentId>) -> Result<(), ()> {
		Ok(())
	}

	#[entry_def]
	fn my_entry_def() -> ValidatingEntryType {
		entry!(
        name: "my_entry",
        description: "this is a same entry definition",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<MyEntry>| {
            Ok(())
        }
    )
	}

	#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
	pub struct MyEntry {
		content: String,
	}

	#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
	pub struct Collective {
		name: String,
		created: SystemTime,
	}

	impl Default for Collective {
		fn default() -> Self {
			Collective {
				name: "unnamed collective".to_string(),
				created: SystemTime::now(),
			}
		}
	}

	#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
	pub struct Ledger {
		name: String,
		created: SystemTime,
	}

	impl Default for Ledger {
		fn default() -> Self {
			Ledger {
				name: "unnamed ledger".to_string(),
				created: SystemTime::now(),
			}
		}
	}

	#[zome_fn("hc_public")]
	fn create_my_entry(entry: MyEntry) -> ZomeApiResult<Address> {
		let entry = Entry::App("my_entry".into(), entry.into());
		let address = hdk::commit_entry(&entry)?;
		Ok(address)
	}

	#[zome_fn("hc_public")]
	fn get_entry(address: Address) -> ZomeApiResult<Option<Entry>> {
		hdk::get_entry(&address)
	}


	#[entry_def]
	fn collective_def() -> ValidatingEntryType {
		entry!(
        name: "collective",
        description: "A cogov collective",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<MyEntry>| {
            Ok(())
        }
    )
	}

	#[zome_fn("hc_public")]
	fn commit_collective(collective: Collective) -> ZomeApiResult<Address> {
		let ledger_name = format!("Primary Ledger for {}", collective.name).to_string();
		let collective_entry = Entry::App("collective".into(), collective.into());
		let collective_address = hdk::commit_entry(&collective_entry)?;
		let ledger = Ledger {
			name: ledger_name,
			..Default::default()
		};
		commit_ledger(ledger)?;
		Ok(collective_address)
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
        validation: | _validation_data: hdk::EntryValidationData<MyEntry>| {
            Ok(())
        }
    )
	}

	fn commit_ledger(ledger: Ledger) -> ZomeApiResult<Address> {
		let collective_entry = Entry::App("collective".into(), ledger.into());
		let collective_address = hdk::commit_entry(&collective_entry)?;
		Ok(collective_address)
	}
}
