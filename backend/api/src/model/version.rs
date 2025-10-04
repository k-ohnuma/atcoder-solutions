use serde::Serialize;

#[derive(Serialize)]
pub struct VersionResponse {
    version: String
}

impl From<String> for VersionResponse {
    fn from(value: String) -> Self {
        VersionResponse {
            version: value
        }
    }
}
impl From<&str> for VersionResponse {
    fn from(value: &str) -> Self {
        VersionResponse {
            version: value.to_owned()
        }
    }
}
