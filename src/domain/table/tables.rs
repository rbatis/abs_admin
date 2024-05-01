use std::collections::HashMap;
use crate::domain::table::LoginCheck;
use crate::domain::table::LoginCheck::PasswordCheck;
use log::LevelFilter;
use rbatis::dark_std::defer;
use rbatis::intercept_log::LogInterceptor;
use rbatis::rbdc::DateTime;
use rbatis::table_sync::{
    ColumMapper, MssqlTableMapper, MysqlTableMapper, PGTableMapper, SqliteTableMapper,
};
use rbatis::RBatis;
use serde::{Deserialize, Serialize};

///Permission Resource Table
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysPermission {
    pub id: Option<String>,
    //father id(can empty)
    pub parent_id: Option<String>,
    pub name: Option<String>,
    //permission
    pub permission: Option<String>,
    //menu path
    pub path: Option<String>,
    pub create_date: Option<DateTime>,
}

crud!(SysPermission {});
impl_select_page!(SysPermission{select_page(dto: &crate::domain::dto::ResPageDTO) =>
    "`where 0 = 0 `
      if dto.name!=null && dto.name!= '':
         ` and name like #{'%'+dto.name+'%'}`
      ` and parent_id IS NULL`
      if !sql.contains('count'):
        ` order by create_date desc`"});
impl_select!(SysPermission{select_by_permission_or_name(permission:&str,name:&str) => "`where permission = #{permission} or name = #{name}`"});
impl_select!(SysPermission{select_by_parent_id_null()=>"`where parent_id IS NULL order by create_date desc`"});

///RoleTable
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysRole {
    pub id: Option<String>,
    pub name: Option<String>,
    //father id(can empty)
    pub parent_id: Option<String>,
    pub create_date: Option<DateTime>,
}

crud!(SysRole {});
impl_select_page!(SysRole{select_page_by_name(name:&str)=>
    "`where 0 = 0`
    if name != '':
      ` and name like #{'%'+name+'%'}`
    ` and parent_id IS NULL `
    if !sql.contains('count'):
     `order by create_date desc`"});

///Role Permission relational tables (relational tables do not use logical deletion)
#[derive(Clone, Debug, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub struct SysRolePermission {
    pub id: Option<String>,
    pub role_id: Option<String>,
    pub permission_id: Option<String>,
    pub create_date: Option<DateTime>,
}
crud!(SysRolePermission {});

///Background user table
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysUser {
    pub id: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub login_check: Option<LoginCheck>,
    pub state: Option<i32>,
    pub create_date: Option<DateTime>,
}

crud!(SysUser {});

impl_select_page!(SysUser{select_page(name:&str,account:&str)=>
    "`where 0 = 0`
    if name != '':
      ` and name like #{'%'+name+'%'}`
    if account != '':
      ` and account like #{'%'+account+'%'}`
    if !sql.contains('count'):
     ` order by create_date desc`"});


///User role relationship tables (relational tables do not use logical deletion)
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysUserRole {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub role_id: Option<String>,
    pub create_date: Option<DateTime>,
}
crud!(SysUserRole {});

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
impl_select_page!(SysDict{select_page(dto: &crate::domain::dto::DictPageDTO) =>
    "`where id!=''`
      if dto.code!=null:
         ` and code = #{dto.code}`
      if dto.name!=null:
         ` and name = #{dto.name}`
      if !sql.contains('count'):
         ` order by create_date `"});

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysTrash {
    pub id: Option<String>,
    pub table_name: Option<String>,
    pub data: Option<String>,
    pub create_date: Option<DateTime>,
}

crud!(SysTrash {});
impl_delete!(SysTrash{ delete_by_day_before(before:DateTime) => "` where create_date < #{before}`"});


#[derive(Serialize, Deserialize)]
pub struct Sms {
    pub account: String,
    pub args: HashMap<String, String>,
}


pub async fn sync_tables(rb: &RBatis) {
    //disable log
    let log_intercept = rb.get_intercept::<LogInterceptor>().unwrap();
    let level = log_intercept.get_level_filter().clone();
    log_intercept.set_level_filter(LevelFilter::Off);
    defer!(|| {
        log_intercept.set_level_filter(level);
    });
    let mapper = {
        match rb.driver_type().unwrap() {
            "sqlite" => &SqliteTableMapper {} as &dyn ColumMapper,
            "mssql" => &MssqlTableMapper {} as &dyn ColumMapper,
            "mysql" => &MysqlTableMapper {} as &dyn ColumMapper,
            "postgres" => &PGTableMapper {} as &dyn ColumMapper,
            _ => {
                panic!("not find driver mapper")
            }
        }
    };
    let conn = rb.acquire().await.expect("connection database fail");
    let table = SysPermission {
        id: Some("".to_string()),
        parent_id: Some("".to_string()),
        name: Some("".to_string()),
        permission: Some("".to_string()),
        path: Some("".to_string()),
        create_date: Some(DateTime::now()),
    };
    let _ = RBatis::sync(&conn, mapper, &table, "sys_permission").await;
    let table = SysRole {
        id: Some("".to_string()),
        parent_id: Some("".to_string()),
        name: Some("".to_string()),
        create_date: Some(DateTime::now()),
    };
    let _ = RBatis::sync(&conn, mapper, &table, "sys_role").await;
    let table = SysRolePermission {
        id: Some("".to_string()),
        role_id: Some("".to_string()),
        permission_id: Some("".to_string()),
        create_date: Some(DateTime::now()),
    };
    let _ = RBatis::sync(&conn, mapper, &table, "sys_role_permission").await;
    let table = SysUser {
        id: Some("".to_string()),
        account: Some("".to_string()),
        password: Some("".to_string()),
        name: Some("".to_string()),
        login_check: Some(LoginCheck::NoCheck),
        state: Some(0),
        create_date: Some(DateTime::now()),
    };
    let _ = RBatis::sync(&conn, mapper, &table, "sys_user").await;
    let table = SysUserRole {
        id: Some("".to_string()),
        user_id: Some("".to_string()),
        role_id: Some("".to_string()),
        create_date: Some(DateTime::now()),
    };
    let _ = RBatis::sync(&conn, mapper, &table, "sys_user_role").await;
    let table = SysDict {
        id: Some("".to_string()),
        name: Some("".to_string()),
        code: Some("".to_string()),
        state: Some(0),
        create_date: Some(DateTime::now()),
    };
    let _ = RBatis::sync(&conn, mapper, &table, "sys_dict").await;
    let table = SysTrash {
        id: Some("".to_string()),
        table_name: Some("".to_string()),
        data: Some("".to_string()),
        create_date: Some(DateTime::now()),
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
            name: Some("1".to_string()),
            permission: Some("/".to_string()),
            path: Some("/".to_string()),
            create_date: Some(DateTime::now()),
        },
        SysPermission {
            id: Some(2.to_string()),
            parent_id: None,
            name: Some("dashboard".to_string()),
            permission: Some("dashboard".to_string()),
            path: Some("dashboard".to_string()),
            create_date: Some(DateTime::now()),
        },
        SysPermission {
            id: Some(3.to_string()),
            parent_id: None,
            name: Some("首页".to_string()),
            permission: Some("/".to_string()),
            path: Some("".to_string()),
            create_date: Some(DateTime::now()),
        },
        SysPermission {
            id: Some(4.to_string()),
            parent_id: None,
            name: Some("form".to_string()),
            permission: Some("form".to_string()),
            path: Some("form".to_string()),
            create_date: Some(DateTime::now()),
        },
        SysPermission {
            id: Some(5.to_string()),
            parent_id: None,
            name: Some("table".to_string()),
            permission: Some("table".to_string()),
            path: Some("table".to_string()),
            create_date: Some(DateTime::now()),
        },
        SysPermission {
            id: Some(6.to_string()),
            parent_id: None,
            name: Some("profile".to_string()),
            permission: Some("profile".to_string()),
            path: Some("profile".to_string()),
            create_date: Some(DateTime::now()),
        },
        SysPermission {
            id: Some(7.to_string()),
            parent_id: None,
            name: Some("result".to_string()),
            permission: Some("result".to_string()),
            path: Some("result".to_string()),
            create_date: Some(DateTime::now()),
        },
        SysPermission {
            id: Some(8.to_string()),
            parent_id: None,
            name: Some("exception".to_string()),
            permission: Some("exception".to_string()),
            path: Some("exception".to_string()),
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
