use crate::domain::table::SysPermission;
use std::collections::HashMap;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysPermissionVO {
    #[serde(flatten)]
    pub inner: SysPermission,
    pub childs: Option<Vec<SysPermissionVO>>,
}

impl From<SysPermission> for SysPermissionVO {
    fn from(arg: SysPermission) -> Self {
        Self {
            inner: arg,
            childs: None,
        }
    }
}

impl SysPermissionVO {
    pub fn get_father_id(&self) -> &Option<String> {
        &self.inner.parent_id
    }

    pub fn set_childs_recursive(&mut self, all_record: &HashMap<String, Self>) {
        let mut childs: Option<Vec<Self>> = None;
        if self.inner.id.is_some() {
            for (_key, x) in all_record {
                if x.get_father_id().is_some() && self.inner.id.eq(&x.get_father_id()) {
                    let mut item = x.clone();
                    item.set_childs_recursive(all_record);
                    match &mut childs {
                        Some(childs) => {
                            childs.push(item);
                        }
                        None => {
                            let mut vec = vec![];
                            vec.push(item);
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
