use std::fmt;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum NodeIdentityEqualityStrategy {
    #[serde(rename = "host")]
    Host,
    #[serde(rename = "public-key")]
    PublicKey,
}

impl fmt::Display for NodeIdentityEqualityStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
