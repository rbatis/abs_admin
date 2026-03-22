use crate::domain::dto::DictPageDTO;
use rbatis::rbdc::DateTime;
use rbatis::{crud, html_sql};

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

#[html_sql("src/domain/table/sys_dict.html")]
impl SysDict {
    pub async fn select_page(conn: &dyn rbatis::Executor, page_request: &rbatis::PageRequest, dto: &DictPageDTO) -> rbatis::Result<Page<SysDict>> { impled!() }
}
