use time::{Tm, Timespec};
use hdk::holochain_json_api::{
	json::JsonString,
	error::JsonError,
};

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
