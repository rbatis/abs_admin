use rbatis::rbdc::DateTime;
use crate::domain::table::*;
crud!(SysTrash {});
impl_delete!(SysTrash{ delete_by_day_before(before:DateTime) => "` where create_date < #{before}`"});
