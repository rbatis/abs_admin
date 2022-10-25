/// auth dto
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct SysAuthDTO {
    pub access_token: String,
    pub path: String,
}
