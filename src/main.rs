use qbittorrent::Api;

#[tokio::main(flavor = "current_thread")]
async fn main() {
	let username = "admin";
	let password = "";
	let address = "http://localhost:4568";

	let api = Api::new(username, password, address).await.unwrap();

	let global_speeds = api.get_global_transfer_info().await.unwrap();
	println!("current download rate is at {} bytes / sec", global_speeds.dl_info_speed());

	let torrent_list = api.get_torrent_list().await.unwrap();
	println!("qbittorrent is managing {} torrents", torrent_list.len());
}