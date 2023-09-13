use std::collections::HashMap;
use async_std::task;

use rspotify::{
    model::{PlaylistId, PlayableItem, ArtistId, FullTracks, Market},
    prelude::BaseClient,
    ClientCredsSpotify, Credentials,
};

#[tokio::main]
async fn main() {
    let creds = Credentials::new(
        "210c4166b7ae423ab7dfcd4362659ff6",
        "f38ab310eee84c0fb1092ef4274a10d6",
    );
    let spotify = ClientCredsSpotify::new(creds);
    spotify.request_token().await.unwrap();

    let playlist_uri = PlaylistId::from_id("1JkjsP4R7jTh6jPVzlsfP7").unwrap();
    let playlist = spotify.playlist(playlist_uri, None, None).await.unwrap();
    let mut artists_id: HashMap<ArtistId, u32> = HashMap::new();

    for item in playlist.tracks.items.iter() {
        let playable_item = item.clone().track.unwrap();
        match playable_item {
            PlayableItem::Track(val) => {
                let full_track_artists = val.artists;
                let artist_id = full_track_artists.first().unwrap().id.as_ref().unwrap();
                if !artists_id.contains_key(artist_id) {
                    artists_id.insert(artist_id.clone(), 0);
                }
            },
            PlayableItem::Episode(_) =>  (),
        }
    }

    let mut top_tracks: HashMap<ArtistId, FullTracks> = HashMap::new();

    artists_id.into_iter().for_each(|id| {
        let full_tracks = task::block_on(get_full_tracks_by_artist(&spotify, &id.0));
        top_tracks.insert(id.0, full_tracks);
    });


    // let artists_id_copy = artists_id.clone();

    // for (artist_id, _followers) in artists_id_copy.iter() {
    //     // let artist_uri = ArtistId::from_id(artist_id.to_string());
    //     let artist = spotify.artist(artist_id.clone()).await.unwrap();
    //     let follows = artist.followers.total;
    //     artists_id.insert(artist_id.clone(), follows);
    //     println!("({:?},{:?})", artist_id,artists_id.get(artist_id));
    // }
}

async fn get_full_tracks_by_artist(spotify: &ClientCredsSpotify, id: &ArtistId<'_>) -> FullTracks {
    let result = FullTracks {
        tracks: spotify.artist_top_tracks(id.clone(), Some(Market::Country(rspotify::model::Country::Argentina))).await.unwrap()
    };
    result
}

    // let artists_id_c = Arc::new(artists_id);

    // thread::spawn(move || {
    //     artists_id.into_iter().for_each(|id| {
    //         let full_tracks = task::block_on(get_full_tracks_by_artist(&spotify, &id.0));
    //         top_tracks.insert(id.0, full_tracks);
    //     });
    // });

    // thread::spawn(move || {
    //     artists_id.into_iter().for_each(|id| {
    //         let full_tracks = task::block_on(get_full_tracks_by_artist(&spotify, &id.0));
    //         top_tracks.insert(id.0, full_tracks);
    //     });
    // });