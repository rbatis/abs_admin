use crate::pool;
use crate::context::CONTEXT;
use parking_lot::Mutex;
use rbatis::executor::Executor;
use rbatis::intercept::{Intercept, ResultType};
use rbatis::object_id::ObjectId;
use rbatis::rbdc::db::ExecResult;
use rbatis::rbdc::DateTime;
use rbatis::rbdc::Error;
use rbs::Value;
use serde::Serialize;
use sqlparser::ast::{FromTable, Statement};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use std::fmt::Debug;
use std::time::Duration;
use rbatis::async_trait;
use crate::domain::table::sys_trash::SysTrash;

/// A trash can service that can recycle data. Retrieve the data, display the trash can data
#[derive(Debug)]
pub struct SysTrashService {
    pub recycle_date: Mutex<DateTime>,
}

impl SysTrashService {
    pub fn new() -> Self {
        Self {
            recycle_date: Mutex::new(DateTime::now()),
        }
    }
    pub async fn add<T>(&self, table_name: &str, args: &[T]) -> Result<u64, Error>
    where
        T: Serialize,
    {
        if args.is_empty() {
            return Ok(0);
        }
        let now = DateTime::now();
        //copy data to trash
        let mut trashes = Vec::with_capacity(args.len());
        for x in args {
            trashes.push(SysTrash {
                id: Some(ObjectId::new().to_string().into()),
                table_name: Some(table_name.to_string()),
                data: Some(serde_json::to_string(x).unwrap_or_default()),
                create_date: Some(now.clone()),
            });
        }
        let r = SysTrash::insert_batch(pool!(), &trashes, 20)
            .await?
            .rows_affected;
        let diff = now.clone().0 - self.recycle_date.lock().0.clone();
        if diff > Duration::from_secs(24 * 3600) {
            *self.recycle_date.lock() = now.clone();
            let _ = self.recycle().await;
        }
        Ok(r)
    }

    //recycle trash older than `trash_recycle_days`
    pub async fn recycle(&self) -> Result<u64, Error> {
        let before = DateTime::now().0.sub(Duration::from_secs(
            CONTEXT.config.trash_recycle_days * 24 * 3600,
        ));
        let r = SysTrash::delete_by_day_before(pool!(), DateTime(before)).await?;
        Ok(r.rows_affected)
    }
}

/// delete sql => select sql=> insert to Trash => delete sql
#[async_trait]
impl Intercept for SysTrashService {
    async fn before(
        &self,
        _task_id: i64,
        rb: &dyn Executor,
        sql: &mut String,
        args: &mut Vec<Value>,
        _result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
    ) -> Result<Option<bool>, Error> {
        if sql.starts_with("delete from ") {
            let dialect = GenericDialect {}; // or AnsiDialect
            let mut v: Vec<Statement> = Parser::parse_sql(&dialect, &sql.clone())
                .map_err(|e| Error::from(e.to_string()))?;
            if v.len() <= 0 {
                return Err(Error::from("sql is empty"));
            }
            let table = match v.remove(0) {
                Statement::Delete { from, .. } => {
                    let mut data = "".to_string();
                    match from {
                        FromTable::WithFromKeyword(v) => {
                            for x in v {
                                let x_str = &format!("{}", x);
                                data.push_str(x_str.as_str());
                            }
                        }
                        FromTable::WithoutKeyword(v) => {
                            for x in v {
                                let x_str = &format!("{}", x);
                                data.push_str(x_str.as_str());
                            }
                        }
                    }
                    data
                }
                _ => "".to_string(),
            };
            if table.is_empty() {
                return Err(Error::from(format!("sql={} table_name is empty", sql)));
            }
            if table.eq("sys_trash") {
                return Ok(Some(true));
            }
            let new_sql = sql.clone().replace(
                &format!("delete from {}", table),
                &format!("select * from {}", table),
            );
            let data = rb.query(&new_sql, args.clone()).await?;
            match data {
                Value::Array(arr) => {
                    self.add(&table, &arr).await?;
                }
                _ => {
                    return Err(Error::from(format!("data={} not array", data)));
                }
            }
        }
        Ok(Some(true))
    }
}
