pub mod bencher;
pub mod client;
pub mod password_encoder;
pub mod string;

/// 取目标Vec的成员属性vec集合
/// vec_ref: vec的引用，field_name: 结构体的field名称
///
/// for example:
///      struct SysUserRole{
///         pub role_id:String
///      }
///      let user_roles: Vec<SysUserRole>;
///      let role_ids = make_field_vec!(&user_roles,role_id); // role_ids: Vec<String>
///
///
///
#[allow(unused_macros)]
#[macro_export]
macro_rules! make_field_vec {
    ($vec_ref:expr,$field_name:ident) => {{
        let mut ids = vec![];
        for item in $vec_ref {
            match &item.$field_name {
                std::option::Option::Some(v) => {
                    ids.push(v.clone());
                }
                _ => {}
            }
        }
        ids
    }};
}

/// 取目标Vec的成员属性hashmap集合
/// vec_ref: vec的引用，field_name: 结构体的field名称
///
/// for example:
///      struct SysUserRole{
///         pub role_id:String
///      }
///      let user_roles: Vec<SysUserRole>;
///      let role_ids = make_field_map!(&user_roles,role_id); // role_ids: HashMap<String,SysUserRole>
///
///
///
#[allow(unused_macros)]
#[macro_export]
macro_rules! make_field_map {
    ($vec_ref:expr,$field_name:ident) => {{
        let mut ids = std::collections::HashMap::new();
        for item in $vec_ref {
            match &item.$field_name {
                std::option::Option::Some(v) => {
                    ids.insert(v.clone(),item.clone());
                }
                _ => {}
            }
        }
        ids
    }};
}