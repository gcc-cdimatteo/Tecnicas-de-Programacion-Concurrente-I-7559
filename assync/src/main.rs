use rspotify::model::*;
use rspotify::{
    model::{AlbumId, Country, FullTracks, Market, SearchTracks, SearchType},
    prelude::BaseClient,
    scopes, AuthCodeSpotify, ClientCredsSpotify, Credentials, OAuth,
};
use serde::de::DeserializeOwned;
use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};

#[tokio::main]
async fn main() {
    let creds = Credentials::new(
        "210c4166b7ae423ab7dfcd4362659ff6",
        "f38ab310eee84c0fb1092ef4274a10d6",
    );
    let spotify = ClientCredsSpotify::new(creds);
    spotify.request_token().await.unwrap();

    println!("Looking for Duki tracks...");
    let result = spotify
        .search(
            "duki",
            SearchType::Track,
            Some(Market::Country(Country::UnitedStates)),
            None,
            Some(10),
            None,
        )
        .await;

    let searched_tracks = match result {
        Ok(album) => {
            // println!("Searched track: {album:?}");
            album
        }
        Err(err) => {
            // println!("Search error! {err:?}");
            panic!()
        }
    };

    if let SearchResult::Tracks(full_tracks) = searched_tracks {
        // println!("{:?}", full_tracks);
        full_tracks
            .items
            .into_iter()
            .for_each(|t| println!("{:?}", t.name));
    }

    // serde_json::de::Deserializer::from_str()

    // let birdy_uri = AlbumId::from_uri("spotify:album:0sNOF9WDwhWunNAHPD3Baj").unwrap();
    // let albums = spotify.album(birdy_uri, None).await;
    // println!("Response: {albums:#?}");
}
