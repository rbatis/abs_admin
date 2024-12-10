use log::LevelFilter;
use rbatis::dark_std::defer;
use rbatis::intercept_log::LogInterceptor;
use rbatis::RBatis;
use rbatis::rbdc::DateTime;
use rbatis::table_sync::{ColumnMapper, MssqlTableMapper, MysqlTableMapper, PGTableMapper, SqliteTableMapper};
use crate::domain::table::LoginCheck::PasswordCheck;
use crate::domain::table::rbac::{RbacPermission, RbacRole, RbacRolePermission, RbacUserRole};
use crate::domain::table::{rbac, SysDict, SysTrash, SysUser};

pub async fn sync_tables(rb: &RBatis) {
    //disable log
    let log_intercept = rb.get_intercept::<LogInterceptor>().unwrap();
    let level = log_intercept.get_level_filter().clone();
    log_intercept.set_level_filter(LevelFilter::Off);
    defer!(|| {
        log_intercept.set_level_filter(level);
    });
    let mapper: &dyn ColumnMapper = {
        match rb.driver_type().unwrap() {
            "sqlite" => &SqliteTableMapper {},
            "mssql" => &MssqlTableMapper {},
            "mysql" => &MysqlTableMapper {},
            "postgres" => &PGTableMapper {},
            _ => {
                panic!("not find driver mapper")
            }
        }
    };
    let conn = rb.acquire().await.expect("connection database fail");
    // RBAC permission
    crate::domain::table::rbac::sync_tables(&conn,mapper).await;
    
    let table = SysUser {
        id: Some(Default::default()),
        account: Some(Default::default()),
        password: Some(Default::default()),
        name: Some(Default::default()),
        login_check: Some(Default::default()),
        state: Some(Default::default()),
        create_date: Some(Default::default()),
    };
    let _ = RBatis::sync(&conn, mapper, &table, "sys_user").await;
    let table = SysDict {
        id: Some(Default::default()),
        name: Some(Default::default()),
        code: Some(Default::default()),
        state: Some(Default::default()),
        create_date: Some(Default::default()),
    };
    let _ = RBatis::sync(&conn, mapper, &table, "sys_dict").await;
    let table = SysTrash {
        id: Some(Default::default()),
        table_name: Some(Default::default()),
        data: Some(Default::default()),
        create_date: Some(Default::default()),
    };
    let _ = RBatis::sync(&conn, mapper, &table, "sys_trash").await;

    let _ = rbac::sync_tables(&conn, mapper).await;
}


pub async fn sync_tables_data(rb: &RBatis) {
    let conn = rb.acquire().await.expect("init data fail");
    if let Ok(v) = SysUser::select_by_column(&conn, "id", "1").await {
        if v.len() > 0 {
            //if user exists,return
            return;
        }
    };
    let _ = SysUser::insert(
        &conn,
        &SysUser {
            id: Some("1".to_string()),
            account: Some("00000000000".to_string()),
            password: Some("e10adc3949ba59abbe56e057f20f883e".to_string()),
            name: Some("admin".to_string()),
            login_check: Some(PasswordCheck),
            state: Some(1),
            create_date: Some(DateTime::now()),
        },
    )
        .await;

    let _ = RbacRole::insert(
        &conn,
        &RbacRole {
            id: Some(1.to_string()),
            name: Some("admin".to_string()),
            create_date: Some(DateTime::now()),
        },
    )
        .await;

    let _ = RbacUserRole::insert(
        &conn,
        &RbacUserRole {
            id: Some(1.to_string()),
            user_id: Some(1.to_string()),
            role_id: Some(1.to_string()),
            create_date: Some(DateTime::now()),
        },
    )
        .await;

    let sys_permissions = vec![
        RbacPermission {
            id: Some(1.to_string()),
            name: Some("首页".to_string()),
            permission: Some("/".to_string()),
            path: Some("/".to_string()),
            create_date: Some(DateTime::now()),
        },
        RbacPermission {
            id: Some(9.to_string()),
            name: Some("user".to_string()),
            permission: Some("user".to_string()),
            path: Some("user".to_string()),
            create_date: Some(DateTime::now()),
        },
        RbacPermission {
            id: Some(10.to_string()),
            name: Some("setting".to_string()),
            permission: Some("setting".to_string()),
            path: Some("setting".to_string()),
            create_date: Some(DateTime::now()),
        },
    ];

    let mut index = 1;
    for permission in sys_permissions {
        let _ = RbacPermission::insert(&conn, &permission).await;
        let role_permission = RbacRolePermission {
            id: Some(index.to_string()),
            role_id: Some(1.to_string()),
            permission_id: permission.id.clone(),
            create_date: Some(DateTime::now()),
        };
        let _ = RbacRolePermission::insert(&conn, &role_permission).await;
        index += 1;
    }
}