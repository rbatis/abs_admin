use crate::domain::table::SysTrash;
use crate::pool;
use rbatis::object_id::ObjectId;
use rbatis::rbdc::datetime::DateTime;
use rbatis::rbdc::Error;
use serde::Serialize;

/// A trash can service that can recycle data. Retrieve the data, display the trash can data
pub struct SysTrashService {}

impl SysTrashService {
    pub async fn add<T>(&self, table_name: &str, args: &[T]) -> Result<u64, Error>
    where
        T: Serialize,
    {
        if args.is_empty() {
            return Ok(0);
        }
        //copy data to trash
        let mut trashes = Vec::with_capacity(args.len());
        for x in args {
            trashes.push(SysTrash {
                id: Some(ObjectId::new().to_string().into()),
                table_name: Some(table_name.to_string()),
                data: Some(serde_json::to_string(x).unwrap_or_default()),
                create_date: Some(DateTime::now()),
            });
        }
        Ok(SysTrash::insert_batch(pool!(), &trashes, 20)
            .await?
            .rows_affected)
    }
}
