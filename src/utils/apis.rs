/// Utilities to query the IGDB API to look for games in their game library.
///
/// The GOG API documentation lives here:
pub mod igdb {
    #![allow(unused_variables, dead_code)]

    use ron::de::from_reader;
    use serde::{Deserialize, Serialize};
    use serenity::futures::lock::Mutex;
    use std::{fs::File, path::PathBuf};
    use lazy_static::lazy_static;

    // Storage for the login token
    lazy_static!(
        // Client ID (not the secret)
        static ref CLIENT_ID: Mutex<String> = Mutex::new("".to_owned());

        // IGDB Token infos
        static ref TOKEN: Mutex<String> = Mutex::new("".to_owned());
        static ref EXPIRES_IN: Mutex<i32> = Mutex::new(0 as i32);
        static ref TOKEN_TYPE: Mutex<String> = Mutex::new("".to_owned());
    );

    /// URLs to use to query the GOG API
    mod endpoints {
        /// Authentication routes
        pub mod auth {
            /// The URL to call to log into the IGDB API
            /// 
            /// Method: GET
            /// 
            /// #### Parameters (all REQUIRED):
            ///
            /// 
            /// client_id (str)
            /// 
            /// client_secret (str)
            /// 
            /// grant_type (str) and must be set to "client_credentials"
            pub const URL: &'static str = "https://id.twitch.tv/oauth2/token";
        }

        pub mod search {
            /// IGDB Search API
            /// 
            /// Method: POST
            /// 
            /// Parameters: See https://api-docs.igdb.com/?java#search
            pub const SEARCH_GAME: &'static str = "https://api.igdb.com/v4/games/";
        }
    }

    #[derive(Debug, Deserialize, Clone)]
    /// Data read from your igdb.ron file
    pub struct IGDBSecret {
        client_id: String,
        client_secret: String,
        #[serde(skip)]
        grant_type: String,
    }

    impl Default for IGDBSecret {
        fn default() -> Self {
            Self {
                client_id: "".to_owned(),
                client_secret: "".to_owned(),
                // Never changes
                grant_type: "client_credentials".to_owned(),
            }
        }
    }

    #[derive(Debug, Deserialize)]
    struct IGDBTokenInfo {
        access_token: String,
        expires_in: i32,
        token_type: String,
    }

    /// Reads your IGDB secrets from the data/igdb.ron file.
    /// 
    /// Also stores your client id (not client secret), in memory for quick access.
    /// 
    /// See data/dummy_igdb.ron for an example.
    pub async fn read_secrets_from_file() -> Result<IGDBSecret, Box<dyn std::error::Error>> {
        let path: PathBuf = PathBuf::from("data/igdb.ron");
        let file: File = File::open(path).expect("Cannot open file data/igdb.ron");
        
        let read: IGDBSecret = from_reader(file)?;
        *CLIENT_ID.lock().await = read.client_id.clone();

        Ok(read)
    }

    /// Returns a token
    pub async fn log_into_igdb() -> Result<(), Box<dyn std::error::Error>> {
        // Do the reqwest
        let client: reqwest::Client = reqwest::Client::new();
        let response = client.get(endpoints::auth::URL)
            .send()
            .await?; // Returns a reqwest error if something bad happens
        // Parse the response JSON into a plain old structure
        let token_infos: IGDBTokenInfo = response.json::<IGDBTokenInfo>().await?;
        // Store the token infos in memory
        // Might consider simply write these to a file, but then it would be readable by anyone (encrypt?)
        *TOKEN.lock().await = token_infos.access_token.clone();
        *EXPIRES_IN.lock().await = token_infos.expires_in;
        *TOKEN_TYPE.lock().await = token_infos.token_type.clone();

        Ok(())
    }

    #[derive(Debug, Deserialize, Clone)]
    pub struct IGDBGameSearchResponseData {
        found: Vec<IGDBGameBasic>,
    }

    #[derive(Debug, Deserialize, Clone)]
    pub struct IGDBGameBasic {
        id: u32,
        name: String,
        platforms: Vec<IGDBPlatformBasic>,
    }

    #[derive(Debug, Deserialize, Clone)]
    pub struct IGDBPlatformBasic {
        id: u32,
        name: String,
    }

    /// Pushes a query to the IGDB API
    /// 
    /// Returns 
    pub async fn query_game_by_name(game_name: String) -> Result<IGDBGameSearchResponseData, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let client_id: String = CLIENT_ID.lock().await.clone();
        let token: String = format!("{} {}", TOKEN_TYPE.lock().await.clone(), TOKEN.lock().await.clone());
        let response = client.post(endpoints::search::SEARCH_GAME)
            .header("Client-ID", &client_id)
            .header("Authorization", &token)
            .body(format!("search {};\nfields name,platforms.name", game_name))
            .send()
            .await?;
        let parsed_response = response.json::<IGDBGameSearchResponseData>().await?;
        Ok(parsed_response)
    }
}
