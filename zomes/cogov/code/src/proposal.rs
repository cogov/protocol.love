use time::{Tm, Timespec};
use hdk::holochain_json_api::{
	json::JsonString,
	error::JsonError,
};

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
