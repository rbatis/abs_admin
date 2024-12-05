use crate::domain::table::{RbacPermission, RbacRole};
use crate::context::CONTEXT;
use std::collections::HashMap;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysPermissionVO {
    pub id: Option<String>,
    //father id(can empty)
    pub parent_id: Option<String>,
    pub name: Option<String>,
    //permission
    pub permission: Option<String>,
    //menu path
    pub path: Option<String>,
    pub create_date: Option<String>,
    pub childs: Option<Vec<SysPermissionVO>>,
}

impl From<RbacPermission> for SysPermissionVO {
    fn from(arg: RbacPermission) -> Self {
        Self {
            id: arg.id,
            parent_id: arg.parent_id,
            name: arg.name,
            permission: arg.permission,
            path: arg.path,
            create_date: arg
                .create_date
                .map(|v| v.format(&CONTEXT.config.datetime_format)),
            childs: None,
        }
    }
}

impl SysPermissionVO {
    pub fn get_father_id(&self) -> &Option<String> {
        &self.parent_id
    }

    pub fn set_childs_recursive(&mut self, all_record: &HashMap<String, Self>) {
        let mut childs: Option<Vec<Self>> = None;
        if self.id.is_some() {
            for (_key, x) in all_record {
                if x.get_father_id().is_some() && self.id.eq(&x.get_father_id()) {
                    let mut item = x.clone();
                    item.set_childs_recursive(all_record);
                    match &mut childs {
                        Some(childs) => {
                            childs.push(item);
                        }
                        None => {
                            let vec = vec![item];
                            childs = Some(vec);
                        }
                    }
                }
            }
        }
        if childs.is_some() {
            self.childs = Some(childs.unwrap());
        }
    }
}



#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysRoleVO {
    pub id: Option<String>,
    pub name: Option<String>,
    //father id(can empty)
    pub parent_id: Option<String>,
    pub create_date: Option<String>,
    pub resources: Vec<SysPermissionVO>,
    pub childs: Option<Vec<SysRoleVO>>,
    pub resource_ids: Vec<String>,
}

impl From<RbacRole> for SysRoleVO {
    fn from(arg: RbacRole) -> Self {
        Self {
            id: arg.id,
            name: arg.name,
            parent_id: arg.parent_id,
            create_date: arg
                .create_date
                .map(|v| v.format(&CONTEXT.config.datetime_format)),
            resources: vec![],
            childs: None,
            resource_ids: vec![],
        }
    }
}

impl SysRoleVO {
    pub fn from_option(arg: Option<RbacRole>) -> Option<SysRoleVO> {
        match arg {
            Some(arg) => Some(SysRoleVO {
                id: arg.id,
                name: arg.name,
                parent_id: arg.parent_id,
                create_date: arg
                    .create_date
                    .map(|v| v.format(&CONTEXT.config.datetime_format)),
                resources: vec![],
                childs: None,
                resource_ids: vec![],
            }),
            _ => None,
        }
    }
}
