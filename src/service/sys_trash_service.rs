use std::fmt::{Debug};
use rbatis::executor::Executor;
use rbatis::intercept::{Intercept, ResultType};
use crate::domain::table::SysTrash;
use crate::pool;
use rbatis::object_id::ObjectId;
use rbatis::rbdc::datetime::DateTime;
use rbatis::rbdc::db::ExecResult;
use rbatis::rbdc::Error;
use rbs::Value;
use serde::Serialize;
use sqlparser::ast::Statement;
use sqlparser::parser::Parser;
use sqlparser::dialect::GenericDialect;

/// A trash can service that can recycle data. Retrieve the data, display the trash can data
#[derive(Debug)]
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
    ) -> Result<bool, Error> {
        if sql.starts_with("delete from ") {
            let dialect = GenericDialect {}; // or AnsiDialect
            let v: Vec<Statement> = Parser::parse_sql(&dialect, &sql.clone()).map_err(|e| Error::from(e.to_string()))?;
            if v.len() <= 0 {
                return Err(Error::from("sql is empty"));
            }
            let table = match v.get(0).unwrap() {
                Statement::Delete {
                    tables:_,
                    from,
                    using: _,
                    selection: _,
                    returning: _, } => {
                    let mut data = "".to_string();
                    for x in from {
                        let x_str = &format!("{}", x);
                        data.push_str(x_str.as_str());
                    }
                    data
                }
                _ => {
                    "".to_string()
                }
            };
            if table.is_empty() {
                return Err(Error::from(format!("sql={} table_name is empty", sql)));
            }
            let new_sql = sql.clone().replace(&format!("delete from {}", table), &format!("select * from {}", table));
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
        Ok(true)
    }
}