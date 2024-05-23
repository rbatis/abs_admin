# 发现的问题

- 角色菜单缓存没有更新
- 登陆多次查询
/admin/sys_login
`select * from sys_user  where account = ?` ["00000000000"]
select * from sys_user_role 有2次查询
find_user_permission 和 find_user_role 相同功能
/admin/sys_user_info
`select * from sys_user  where id = ?` ["1"]
select * from sys_user_role 有2次查询
- 多次token验证

# 更新内容
- ServiceContext 改为Default注入, service 实例 Default::default()
- fast_log 找到输出文件名和行号的办法, 并修改日志格式
- 密码加密bcrypt