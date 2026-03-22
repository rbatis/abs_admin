use crate::domain::dto::rbac::PermissionPageDTO;
use rbatis::rbdc::DateTime;
use rbatis::table_sync::ColumnMapper;
use rbatis::{RBatis, crud, html_sql};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use rbatis::executor::Executor;

///Permission Resource Table
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Hash, Eq, PartialEq)]
pub struct RbacPermission {
    pub id: Option<String>,
    pub name: Option<String>,
    //permission
    pub permission: Option<String>,
    //menu path
    pub path: Option<String>,
    pub create_date: Option<DateTime>,
}

crud!(RbacPermission {});

#[html_sql("src/domain/table/rbac.html")]
impl RbacPermission {
    pub async fn select_page(conn: &dyn Executor,request:&rbatis::PageRequest,dto: &PermissionPageDTO) -> rbatis::Result<Page<RbacPermission>> { impled!() }
    pub async fn select_by_permission_or_name(conn: &dyn Executor,permission:&str,name:&str) -> rbatis::Result<Vec<RbacPermission>> { impled!() }
    pub async fn select_by_parent_id_null(conn: &dyn Executor) -> rbatis::Result<Vec<RbacPermission>> { impled!() }
}

///RoleTable
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RbacRole {
    pub id: Option<String>,
    pub name: Option<String>,
    pub create_date: Option<DateTime>,
}

crud!(RbacRole {});

#[html_sql("src/domain/table/rbac.html")]
impl RbacRole {
    pub async fn select_page_by_name(conn: &dyn Executor, page_request: &dyn PageRequest, name: &str) -> rbatis::Result<Page<RbacRole>> { impled!() }
}

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
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Eq, PartialEq, Hash)]
pub struct RbacUserRole {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub role_id: Option<String>,
    pub create_date: Option<DateTime>,
}
crud!(RbacUserRole {});

pub async fn sync_tables(conn: &dyn Executor, mapper: &dyn ColumnMapper) {
    // RBAC permission
    let table = RbacPermission {
        id: Some(Default::default()),
        name: Some(Default::default()),
        permission: Some(Default::default()),
        path: Some(Default::default()),
        create_date: Some(Default::default()),
    };
    let _ = RBatis::sync(conn, mapper, &table, "rbac_permission").await;
    let table = RbacRole {
        id: Some(Default::default()),
        name: Some(Default::default()),
        create_date: Some(Default::default()),
    };
    let _ = RBatis::sync(conn, mapper, &table, "rbac_role").await;
    let table = RbacRolePermission {
        id: Some(Default::default()),
        role_id: Some(Default::default()),
        permission_id: Some(Default::default()),
        create_date: Some(Default::default()),
    };
    let _ = RBatis::sync(conn, mapper, &table, "rbac_role_permission").await;
    let table = RbacUserRole {
        id: Some(Default::default()),
        user_id: Some(Default::default()),
        role_id: Some(Default::default()),
        create_date: Some(Default::default()),
    };
    let _ = RBatis::sync(conn, mapper, &table, "rbac_user_role").await;
    // RBAC permission end
}

pub trait IntoMap<K: Eq + Hash, V>: Sized + IntoIterator<Item = V> {
    fn into_map(self, id_fn: fn(&V) -> K) -> HashMap<K, V> {
        let mut map = HashMap::new();
        for item in self {
            map.insert(id_fn(&item), item);
        }
        map
    }
}

pub trait IntoMapVec<K: Eq + Hash + Clone, V: Eq + Hash>: Sized + IntoIterator<Item = V> {
    fn into_map(self, id_fn: fn(&V) -> K) -> HashMap<K, Vec<V>> {
        let mut map = HashMap::new();
        for item in self {
            let key = id_fn(&item);
            if !map.contains_key(&key) {
                map.insert(key.clone(), HashSet::new());
            }
            if let Some(v) = map.get_mut(&key) {
                v.insert(item);
            }
        }
        let mut map2 = HashMap::with_capacity(map.len());
        for (k, v) in map {
            let vec = v.into_iter().collect::<Vec<V>>();
            map2.insert(k, vec);
        }
        map2
    }
}

impl IntoMap<String, RbacRole> for Vec<RbacRole> {}
impl IntoMap<String, RbacPermission> for Vec<RbacPermission> {}
impl IntoMapVec<String, RbacRolePermission> for Vec<RbacRolePermission> {}

impl IntoMapVec<String, RbacUserRole> for Vec<RbacUserRole> {}
