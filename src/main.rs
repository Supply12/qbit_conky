use qbit::{Api, Credentials};

#[tokio::main]
async fn main() {
    let credentials = Credentials::new("admin", "p");
    let client = Api::new_login("http://localhost:4568", credentials)
        .await
        .unwrap();
    let torrents = client.torrents(None).await.unwrap();

    // let host_credentials = Credentials::new("admin", "Qcug~z6t3'?DC#+"); 
    // let host_client = Api::new_login("http://192.168.0.25:8080", host_credentials)
    //     .await
    //     .unwrap();
    // let host_torrents = host_client.torrents(None).await.unwrap();	

    for torrent in torrents {
		let name = torrent.name;
		let short_name: String = name.chars().take(12).collect();
		let percent_progress = (torrent.progress * 100.0).round() as i32;
        println!(" ${{color 8e8e8e}}{:?} ${{color white}}{}% ${{execbar 'echo {}'}}", short_name, percent_progress, percent_progress);
    }
    // for torrent in host_torrents {
	// 	let name = torrent.name;
	// 	let short_name: String = name.chars().take(12).collect();
	// 	let percent_progress = (torrent.progress * 100.0).round() as i32;
    //     println!("{:?} {}% ${{execbar 'echo {}'}}", short_name, percent_progress, percent_progress);
    // }
}