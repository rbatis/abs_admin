use crate::domain::table::SysDict;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysDictVO {
    #[serde(flatten)]
    pub inner: SysDict,
}

impl From<SysDict> for SysDictVO {
    fn from(arg: SysDict) -> Self {
        Self { inner: arg }
    }
}

impl SysDictVO {}
