pub mod bencher;
pub mod password_encoder;

///快速取 源对象数组的 属性类型数组Vec，
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
macro_rules! to_field_vec {
    ($vec_ref:expr,$field_name:ident) => {{
        let mut ids = vec![];
        for item in $vec_ref {
            ids.push(item.$field_name.clone().unwrap_or("".to_string()));
        }
        ids
    }};
}
