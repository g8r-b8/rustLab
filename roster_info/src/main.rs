// cargo run "MIA"
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

pub fn minify(string: &str){
    for line in string.split('\n'){
        print!("{}                                                              ", line);
    }
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let search_query = &args[1];
    let url = format!(
        "https://www.basketball-reference.com/teams/{query}/2024.html",
        query = search_query
    );
    let response = reqwest::get(url)
        .await
        .unwrap();
        //.text()
        //.await;
    match response.status(){
        reqwest::StatusCode::OK => {
            let text = response.text().await.unwrap();
            let text = &text.replace("\n","\n \n");
            //println!("{:?}", &text);
            minify(text);
            //let split_text = text.split("\n");
            //println!("{:?}", split_text);
            //minify(&response.text().await.unwrap())
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token");
        }
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    }
    //println!("{:?}", response);
    // TODO: parse the data with serde, set up the structs to be what we're looking for
    /*
    match response.status() {
        reqwest::StatusCode::OK => {
            // on success, parse our JSON to an APIResponse
            match response.json::<APIResponse>().await {
                Ok(parsed) => println!("Success! {:?}", parsed),
                //Ok(parsed) => print_tracks(parsed.tracks.items.iter().collect()),
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
    */
}
