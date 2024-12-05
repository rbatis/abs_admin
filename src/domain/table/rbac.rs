use rbatis::rbdc::DateTime;
use rbatis::{crud, impl_select, impl_select_page};
use crate::domain::dto::rbac::ResPageDTO;

///Permission Resource Table
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RbacPermission {
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

crud!(RbacPermission {});
impl_select_page!(RbacPermission{select_page(dto: &ResPageDTO) =>
    "`where 0 = 0 `
      if dto.name!=null && dto.name!= '':
         ` and name like #{'%'+dto.name+'%'}`
      ` and parent_id IS NULL`
      if !sql.contains('count'):
        ` order by create_date desc`"});
impl_select!(RbacPermission{select_by_permission_or_name(permission:&str,name:&str) => "`where permission = #{permission} or name = #{name}`"});
impl_select!(RbacPermission{select_by_parent_id_null()=>"`where parent_id IS NULL order by create_date desc`"});

///RoleTable
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RbacRole {
    pub id: Option<String>,
    pub name: Option<String>,
    //father id(can empty)
    pub parent_id: Option<String>,
    pub create_date: Option<DateTime>,
}

crud!(RbacRole {});
impl_select_page!(RbacRole{select_page_by_name(name:&str)=>
    "`where 0 = 0`
    if name != '':
      ` and name like #{'%'+name+'%'}`
    ` and parent_id IS NULL `
    if !sql.contains('count'):
     `order by create_date desc`"});

///Role Permission relational tables (relational tables do not use logical deletion)
#[derive(Clone, Debug, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub struct RbacRolePermission {
    pub id: Option<String>,
    pub role_id: Option<String>,
    pub permission_id: Option<String>,
    pub create_date: Option<DateTime>,
}
crud!(RbacRolePermission {});

///User role relationship tables (relational tables do not use logical deletion)
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RbacUserRole {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub role_id: Option<String>,
    pub create_date: Option<DateTime>,
}
crud!(RbacUserRole {});
