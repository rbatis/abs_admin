use crate::context::CONTEXT;
use crate::domain::table::rbac::{RbacPermission, RbacRole};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RbacPermissionVO {
    pub id: Option<String>,
    pub name: Option<String>,
    //permission
    pub permission: Option<String>,
    //menu path
    pub path: Option<String>,
    pub create_date: Option<String>,
}

impl From<RbacPermission> for RbacPermissionVO {
    fn from(arg: RbacPermission) -> Self {
        Self {
            id: arg.id,
            name: arg.name,
            permission: arg.permission,
            path: arg.path,
            create_date: arg
                .create_date
                .map(|v| v.format(&CONTEXT.config.datetime_format)),
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysRoleVO {
    pub id: Option<String>,
    pub name: Option<String>,
    pub create_date: Option<String>,
    pub permissions: Vec<RbacPermissionVO>,
    pub permission_ids: Vec<String>,
}

impl SysRoleVO {
    pub fn set_permissions(&mut self, data: Vec<RbacPermissionVO>) {
        self.permissions = data;
        self.permission_ids = self
            .permissions
            .iter()
            .map(|v| v.id.clone().unwrap_or_default())
            .collect();
    }
}

impl From<RbacRole> for SysRoleVO {
    fn from(arg: RbacRole) -> Self {
        Self {
            id: arg.id,
            name: arg.name,
            create_date: arg
                .create_date
                .map(|v| v.format(&CONTEXT.config.datetime_format)),
            permissions: vec![],
            permission_ids: vec![],
        }
    }
}

impl SysRoleVO {
    pub fn from_option(arg: Option<RbacRole>) -> Option<SysRoleVO> {
        match arg {
            Some(arg) => Some(SysRoleVO {
                id: arg.id,
                name: arg.name,
                create_date: arg
                    .create_date
                    .map(|v| v.format(&CONTEXT.config.datetime_format)),
                permissions: vec![],
                permission_ids: vec![],
            }),
            _ => None,
        }
    }
}
