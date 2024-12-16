use rbatis::{crud, impl_select_page};
use rbatis::rbdc::DateTime;

///dictionary table
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysDict {
    pub id: Option<String>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub state: Option<i32>,
    pub create_date: Option<DateTime>,
}

crud!(SysDict {});
impl_select_page!(SysDict{select_page(dto: &crate::domain::dto::DictPageDTO) =>
    "`where id!=''`
      if dto.code!=null:
         ` and code = #{dto.code}`
      if dto.name!=null:
         ` and name = #{dto.name}`
      if do_count == false:
         ` order by create_date `"});