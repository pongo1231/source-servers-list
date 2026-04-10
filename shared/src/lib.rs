pub mod stream;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServerListing {
	#[serde(rename = "a")]
	pub id: u16,
	#[serde(rename = "b")]
	pub game: String,
	#[serde(rename = "c")]
	pub addr: String,
	#[serde(rename = "d")]
	pub status: ServerListingStatus,
	#[serde(rename = "e")]
	pub name: String,
	#[serde(rename = "f")]
	pub icon_name: Option<String>,
	#[serde(rename = "g")]
	pub info: Option<ServerListingInfo>,
	#[serde(skip)]
	pub rcon_password: Option<String>,
}

#[derive(Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServerListingInfo {
	#[serde(rename = "a")]
	pub map: String,
	#[serde(rename = "b")]
	pub active: u8,
	#[serde(rename = "c")]
	pub max: u8,
}

#[derive(Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServerListingStatus {
	#[default]
	#[serde(rename = "a")]
	Pending,
	#[serde(rename = "b")]
	Unreachable,
	#[serde(rename = "c")]
	Reachable,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum WSClientMsg {
	Invalid,
	ReqEntries,
	ReqPlayers(u16),
}

#[derive(Clone, Serialize, Deserialize)]
pub enum WSServerMsg {
	Invalid,
	ResEntries(Vec<ServerListing>),
	ResPlayers(u16, Vec<String>),
}
