use rspotify::{prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth, Config};
use std::env;
use tokio::main;

#[main]
async fn main() {
    // Setup credentials (you'll need to add your own client_id, client_secret, and redirect_uri here)
    let creds = Credentials::new("your_client_id", "your_client_secret");

    // OAuth config
    let oauth = OAuth {
        redirect_uri: "your_redirect_uri".to_string(),
        scopes: scopes!("user-library-read", "playlist-read-private", "playlist-modify-public"),
        ..Default::default()
    };

    // Create the Spotify client
    let mut spotify = AuthCodeSpotify::with_config(creds, oauth, Config::default());

    // Start the authorization process (user logs in via a browser)
    let auth_url = spotify.get_authorize_url(false).unwrap();
    println!("Authorize the app here: {}", auth_url);

    // The user will provide the authorization code from the URL after logging in
    println!("Enter the authorization code:");
    let mut code = String::new();
    std::io::stdin().read_line(&mut code).unwrap();
    let code = code.trim();

    // Request the access token using the authorization code
    spotify.request_token(code).await.unwrap();

    // Fetch the current user's playlists (just as an example)
    let playlists = spotify.current_user_playlists(None, None).await.unwrap();
    for playlist in playlists.items {
        println!("Playlist: {}", playlist.name);
    }

    // Now we can fetch playlists, tracks, etc. using the `spotify` instance
}
