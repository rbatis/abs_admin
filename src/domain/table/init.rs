use rbatis::RBatis;
use rbatis::rbdc::DateTime;
use crate::domain::table::LoginCheck::PasswordCheck;
use crate::domain::table::{SysPermission, SysRole, SysRolePermission, SysUser, SysUserRole};

pub async fn sync_tables_data(rb: &RBatis) {
    let conn = rb.acquire().await.expect("init data fail");
    if let Ok(v) = SysUser::select_by_column(&conn, "id", "1").await {
        if !v.is_empty() {
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