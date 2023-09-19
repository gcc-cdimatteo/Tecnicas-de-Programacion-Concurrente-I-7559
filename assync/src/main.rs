use async_std::task;
use futures::future::join_all;
use std::collections::HashMap;

use rspotify::{
    model::{ArtistId, FullTracks, Market, PlayableItem, PlaylistId},
    prelude::BaseClient,
    ClientCredsSpotify, Credentials,
};

async fn get_full_tracks_by_artist(spotify: &ClientCredsSpotify, id: &ArtistId<'_>) -> FullTracks {
    // println!("EMPIEZA PARA: {:?}", id);
    let full_track = FullTracks {
        tracks: spotify
            .artist_top_tracks(
                id.clone(),
                Some(Market::Country(rspotify::model::Country::Argentina)),
            )
            .await
            .unwrap(),
    };
    // println!("TERMINA PARA: {:?}", id);
    full_track
}

async fn async_main() {
    let creds = Credentials::new(
        "210c4166b7ae423ab7dfcd4362659ff6",
        "f38ab310eee84c0fb1092ef4274a10d6",
    );
    let spotify = ClientCredsSpotify::new(creds);
    spotify.request_token().await.unwrap();

    let playlist_uri = PlaylistId::from_id("1JkjsP4R7jTh6jPVzlsfP7").unwrap();
    let playlist = spotify.playlist(playlist_uri, None, None).await.unwrap();
    let mut artists_id: HashMap<ArtistId<'_>, u32> = HashMap::new();

    // Recorro una playlist y me quedo con el id de los artistsas
    for item in playlist.tracks.items.iter() {
        let playable_item = item.clone().track.unwrap();
        match playable_item {
            PlayableItem::Track(val) => {
                let full_track_artists = val.artists;
                let artist_id = full_track_artists.first().unwrap().id.as_ref().unwrap();
                if !artists_id.contains_key(artist_id) {
                    artists_id.insert(artist_id.clone(), 0);
                }
            }
            PlayableItem::Episode(_) => (),
        }
    }

    // Por cada artista me quedo con los top 10 tracks
    let mut top_tracks: HashMap<ArtistId<'_>, Vec<String>> = HashMap::new();

    let full_tracks: Vec<_> = artists_id
        .iter()
        .map(|id| get_full_tracks_by_artist(&spotify, id.0))
        .collect();

    // Aca esta la parte async
    let result = join_all(full_tracks).await;

    result.iter().for_each(|tracks| {
        top_tracks.insert(
            tracks
                .tracks
                .first()
                .unwrap()
                .artists
                .first()
                .unwrap()
                .id
                .as_ref()
                .unwrap()
                .to_owned(),
            tracks.tracks.iter().map(|t| t.name.to_owned()).collect(),
        );
    });

    println!("{:#?}", top_tracks);
}

fn main() {
    let _response = task::block_on(async_main());
}
