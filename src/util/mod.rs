pub mod password_encoder;

///取源对象数组的String属性数组，
/// vec_ref: vec的引用，field: 结构体的field名称
///
/// for example:
///      struct SysUserRole{
///         pub role_id:String
///      }
///      let user_roles: Vec<SysUserRole>;
///      let role_ids = make_str_vec!(&user_roles,role_id); // Vec<String>
///
///
///
#[allow(unused_macros)]
#[macro_export]
macro_rules! make_string_vec {
        ($vec_ref:expr,$field:ident) => {
         {
            let mut ids = vec![];
            for item in $vec_ref {
              ids.push(item.$field.clone().unwrap_or("".to_string()));
            }
            ids
         }
    };
}