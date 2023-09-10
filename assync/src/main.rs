use rspotify::{
    model::{AlbumId, Country, Market},
    prelude::BaseClient,
    scopes, AuthCodeSpotify, ClientCredsSpotify, Credentials, OAuth,
};

#[tokio::main]
async fn main() {
    let creds = Credentials::new(
        "210c4166b7ae423ab7dfcd4362659ff6",
        "f38ab310eee84c0fb1092ef4274a10d6",
    );
    let spotify = ClientCredsSpotify::new(creds);
    spotify.request_token().await.unwrap();
    let birdy_uri = AlbumId::from_uri("spotify:album:0sNOF9WDwhWunNAHPD3Baj").unwrap();
    let albums = spotify.album(birdy_uri, None).await;
    println!("Response: {albums:#?}");
}
