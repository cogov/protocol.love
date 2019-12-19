use time::{Tm, Timespec};
use hdk::holochain_json_api::{
	json::JsonString,
	error::JsonError,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Ledger {
	pub name: String,
	pub created_at_sec: i64,
}

impl Ledger {
	#[allow(dead_code)]
	fn created_at(&self) -> Tm {
		time::at(Timespec::new(self.created_at_sec, 0))
	}
}

impl Default for Ledger {
	fn default() -> Self {
		Ledger {
			name: "unnamed ledger".to_string(),
			created_at_sec: time::now_utc().to_timespec().sec,
		}
	}
}
