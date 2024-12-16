use crate::domain::dto::DictPageDTO;
use rbatis::executor::Executor;
use rbatis::rbdc::DateTime;
use rbatis::{crud, html_sql, Page, PageRequest};

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
impl SysDict {
    #[html_sql("src/domain/table/dict.html")]
    pub async fn select_page(
        rb: &dyn Executor,
        page_request: &PageRequest,
        dto: &DictPageDTO,
    ) -> rbatis::Result<Page<SysDict>> {
        impled!()
    }
}
