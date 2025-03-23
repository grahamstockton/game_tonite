use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD, Engine as _};

#[derive(Clone)]
pub struct UrlParams {
    server_id: String,
    user_id: String,
}

impl UrlParams {
    /**
     * Converts url string from base64 (server_id:user_id) to server_id and user_id
     */
    pub fn decode_url(url: String) -> Result<Self> {
        let st = String::from_utf8(STANDARD.decode(url)?)?;
        let mut s = st.split(":");
        Ok(Self {
            server_id: s.next().context("params decode failed")?.to_string(),
            user_id: s.next().context("params decode failed")?.to_string(),
        })
    }

    pub fn get_server_id(&self) -> String {
        self.server_id.clone()
    }

    pub fn get_user_id(&self) -> String {
        self.user_id.clone()
    }
}
