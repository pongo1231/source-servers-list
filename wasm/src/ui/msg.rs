use shared::ServerListing;

#[derive(Clone)]
pub enum UIMsg {
	WSInit,
	UpdateListing(ServerListing),
	UpdatePlayers(u16, Vec<String>),
}
