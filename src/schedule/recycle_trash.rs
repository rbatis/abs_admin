use std::time::Duration;
use crate::service::CONTEXT;

pub async fn spawn_trash_schedule() {
    tokio::spawn(async move {
        loop {
            let r = CONTEXT.sys_trash_service.recycle().await;
            if r.is_err() {
                log::error!("spawn_trash_schedule fail={}",r.err().unwrap());
            }
            tokio::time::sleep(Duration::from(Duration::from_secs(60))).await;
        }
    });
}