use log::LevelFilter;
use rbatis::dark_std::defer;
use rbatis::intercept_log::LogInterceptor;
use rbatis::RBatis;
use rbatis::rbdc::DateTime;
use rbatis::table_sync::{ColumnMapper, MssqlTableMapper, MysqlTableMapper, PGTableMapper, SqliteTableMapper};
use crate::domain::table::LoginCheck::PasswordCheck;
use crate::domain::table::{SysDict, SysPermission, SysRole, SysRolePermission, SysTrash, SysUser, SysUserRole};

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
    let table = SysPermission {
        id: Some(Default::default()),
        parent_id: Some(Default::default()),
        name: Some(Default::default()),
        permission: Some(Default::default()),
        path: Some(Default::default()),
        create_date: Some(Default::default()),
    };
    let _ = RBatis::sync(&conn, mapper, &table, "sys_permission").await;
    let table = SysRole {
        id: Some(Default::default()),
        parent_id: Some(Default::default()),
        name: Some(Default::default()),
        create_date: Some(Default::default()),
    };
    let _ = RBatis::sync(&conn, mapper, &table, "sys_role").await;
    let table = SysRolePermission {
        id: Some(Default::default()),
        role_id: Some(Default::default()),
        permission_id: Some(Default::default()),
        create_date: Some(Default::default()),
    };
    let _ = RBatis::sync(&conn, mapper, &table, "sys_role_permission").await;
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
    let table = SysUserRole {
        id: Some(Default::default()),
        user_id: Some(Default::default()),
        role_id: Some(Default::default()),
        create_date: Some(Default::default()),
    };
    let _ = RBatis::sync(&conn, mapper, &table, "sys_user_role").await;
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

    let _ = SysRole::insert(
        &conn,
        &SysRole {
            id: Some(1.to_string()),
            name: Some("admin".to_string()),
            parent_id: None,
            create_date: Some(DateTime::now()),
        },
    )
        .await;

    let _ = SysUserRole::insert(
        &conn,
        &SysUserRole {
            id: Some(1.to_string()),
            user_id: Some(1.to_string()),
            role_id: Some(1.to_string()),
            create_date: Some(DateTime::now()),
        },
    )
        .await;

    let sys_permissions = vec![
        SysPermission {
            id: Some(1.to_string()),
            parent_id: None,
            name: Some("首页".to_string()),
            permission: Some("/".to_string()),
            path: Some("/".to_string()),
            create_date: Some(DateTime::now()),
        },
        SysPermission {
            id: Some(9.to_string()),
            parent_id: None,
            name: Some("user".to_string()),
            permission: Some("user".to_string()),
            path: Some("user".to_string()),
            create_date: Some(DateTime::now()),
        },
        SysPermission {
            id: Some(10.to_string()),
            parent_id: None,
            name: Some("setting".to_string()),
            permission: Some("setting".to_string()),
            path: Some("setting".to_string()),
            create_date: Some(DateTime::now()),
        },
    ];

    let mut index = 1;
    for permission in sys_permissions {
        let _ = SysPermission::insert(&conn, &permission).await;
        let role_permission = SysRolePermission {
            id: Some(index.to_string()),
            role_id: Some(1.to_string()),
            permission_id: permission.id.clone(),
            create_date: Some(DateTime::now()),
        };
        let _ = SysRolePermission::insert(&conn, &role_permission).await;
        index += 1;
    }
}