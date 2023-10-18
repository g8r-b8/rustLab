// cargo run "Blushakurx"
use std::env;
use reqwest;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ExternalUrls {
    spotify: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Artist {
    name: String,
    external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]
struct Album {
    name: String,
    artists: Vec<Artist>,
    external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]
struct Track {
    name: String,
    href: String,
    popularity: u32,
    album: Album,
    external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]
struct Items<T> {
    items: Vec<T>,
}
#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    tracks: Items<Track>,
}

fn print_tracks(tracks: Vec<&Track>) {
    for track in tracks {
        println!("🔥 {}", track.name);
        println!("💿 {}", track.album.name);
        println!(
            "🕺 {}",
            track
                .album
                .artists
                .iter()
                .map(|artist| artist.name.to_string())
                .collect::<String>()
        );
        println!("🌎 {}", track.external_urls.spotify);
        println!("---------")
    }
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let search_query = &args[1];
    let url = format!(
        "https://api.spotify.com/v1/search?q={query}&type=track,artist",
        query = search_query
    );
    //let api_key = "64734676ade144f886839334bde1b4a1";
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(AUTHORIZATION, "Bearer BQDLxlm61u1WSkM68NmWCqL7DS0u_oNB16Ai__h0YKZL8eEAlQ84ZF858vpoLWqYsNWKJw_q3xvx3D-_Wd2pOk_0G5CGAMUlCTRoJznx9N4KRn23nDs")
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        // confirm the request using send()
        .send()
        .await
        // the rest is the same!
        .unwrap();
    match response.status() {
        reqwest::StatusCode::OK => {
            // on success, parse our JSON to an APIResponse
            match response.json::<APIResponse>().await {
                //Ok(parsed) => println!("Success! {:?}", parsed),
                Ok(parsed) => print_tracks(parsed.tracks.items.iter().collect()),
                Err(_) => println!("Hm, the response didn't match the shape we expected."),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token");
        }
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    };
    //println!("{:?}", response);
}
