use rbatis::{crud, htmlsql};
use rbatis::rbdc::DateTime;
use rbatis::executor::Executor;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysTrash {
    pub id: Option<String>,
    pub table_name: Option<String>,
    pub data: Option<String>,
    pub create_date: Option<DateTime>,
}

crud!(SysTrash {});
impl SysTrash {
    htmlsql!(delete_by_day_before(rb:&dyn Executor, before:DateTime) -> rbatis::rbdc::db::ExecResult => "src/domain/table/sys_trash.html");
}
