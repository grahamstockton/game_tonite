cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use anyhow::Result;
        use reqwest;
        use std::sync::Arc;
        use http::{HeaderMap, HeaderValue};
        use std::time::Duration;
        use tokio::time::Instant;

        const BASE_URL: &str = "https://api.igdb.com/v4";
        const CLIENT_HEADER: &str = "Client-ID";
        const AUTHORIZATION_HEADER: &str = "Authorization";
        const GET_GAMES_QUERY: &str = "fields name,cover; sort name asc; limit 500;";
        const BEARER_TOKEN_PREFIX: &str = "Bearer ";
    }
}

#[cfg(feature = "ssr")]
enum RequestType {
    Games,
    Covers,
}

#[cfg(feature = "ssr")]
impl RequestType {
    fn as_str(&self) -> &'static str {
        match self {
            RequestType::Games => "games",
            RequestType::Covers => "covers",
        }
    }
}

// access token for igdb api. will expire, so needs to be refreshed
#[cfg(feature = "ssr")]
pub struct AppAccessToken {
    access_token: String,
    expire_time: Instant,
    client_id: String,
    secret_key: String,
    reqwest: Arc<reqwest::Client>,
}

// wrapper for access token. uses refreshing logic due to timeout.
#[cfg(feature = "ssr")]
#[derive(serde::Deserialize)]
struct AccessTokenResponse {
    access_token: String,
    expires_in: u64,
}

#[cfg(feature = "ssr")]
impl AppAccessToken {
    // initialize and get token
    async fn new(reqwest: Arc<reqwest::Client>, client_id: String, secret_key: String) -> Self {
        let t = get_access_token(reqwest.clone(), &client_id, &secret_key).await;
        match t {
            Ok(r) => AppAccessToken {
                access_token: r.access_token,
                expire_time: generate_expiry(r.expires_in),
                client_id: client_id,
                secret_key: secret_key,
                reqwest: reqwest,
            },
            Err(e) => panic!("unable to get initial igdb access token: {:?}", e),
        }
    }

    // get token. if expired, refresh
    async fn get(mut self) -> Result<String> {
        if Instant::now() > self.expire_time {
            let res =
                get_access_token(self.reqwest.clone(), &self.client_id, &self.secret_key).await?;
            self = AppAccessToken {
                access_token: res.access_token,
                expire_time: generate_expiry(res.expires_in),
                ..self
            }
        }

        Ok(self.access_token.clone())
    }
}

// make the api call to get access token
#[cfg(feature = "ssr")]
async fn get_access_token(
    reqwest: Arc<reqwest::Client>,
    client_id: &str,
    secret_key: &str,
) -> Result<AccessTokenResponse> {
    let response = reqwest
        .post(format_access_url(&client_id, &secret_key))
        .send()
        .await?;
    let json = response.json().await?;
    Ok(json)
}

// convert remaining time to an expiration time Instant
#[cfg(feature = "ssr")]
fn generate_expiry(remaining: u64) -> Instant {
    Instant::now()
        .checked_add(Duration::from_secs(remaining))
        .unwrap()
}

#[cfg(feature = "ssr")]
fn format_access_url(client_id: &str, secret_key: &str) -> String {
    return [
        "https://id.twitch.tv/oauth2/token?client_id=",
        client_id,
        "&client_secret=",
        secret_key,
        "&grant_type=client_credentials",
    ]
    .join("");
}

// client for interacting with the international games database api
#[cfg(feature = "ssr")]
pub struct IgdbClient {
    pub reqwest: Arc<reqwest::Client>,
    pub client_id: String,
    pub secret_key: String,
    pub access_token: AppAccessToken,
}

#[cfg(feature = "ssr")]
impl IgdbClient {
    pub async fn new(reqwest: Arc<reqwest::Client>, client_id: String, secret_key: String) -> Self {
        IgdbClient {
            reqwest: reqwest.clone(),
            client_id: client_id.clone(),
            secret_key: secret_key.clone(),
            access_token: AppAccessToken::new(reqwest, client_id, secret_key).await,
        }
    }

    // get a list of all games
    pub async fn get_games(&self) {
        let mut headers = HeaderMap::new();
        headers.insert(
            CLIENT_HEADER,
            HeaderValue::from_str(self.client_id.as_str()).unwrap(),
        );
        headers.insert(
            AUTHORIZATION_HEADER,
            HeaderValue::from_str(&[BEARER_TOKEN_PREFIX, &self.access_token.access_token].join(""))
                .unwrap(),
        );

        let res = self
            .reqwest
            .post(get_post_url(RequestType::Games))
            .headers(headers)
            .body(GET_GAMES_QUERY)
            .send()
            .await
            .unwrap();

        // TODO: parse this into a batch process, deserialize
        // TODO: filter out all the games we aren't gonna play
        println!("{:?}", res.text().await.unwrap());
    }
}

#[cfg(feature = "ssr")]
fn get_post_url(t: RequestType) -> String {
    format!("{}/{}", BASE_URL.to_string(), t.as_str().to_owned())
}

#[cfg(feature = "ssr")]
pub struct GetGamesResponse {}

#[cfg(feature = "ssr")]
pub struct GetCoverResponse {}
