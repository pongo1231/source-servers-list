use shared::ServerListing;

#[derive(Clone)]
pub enum UIMsg {
	Init,
	UpdateListing(ServerListing),
	UpdatePlayers(u16, Vec<String>),
}
