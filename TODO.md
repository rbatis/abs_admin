# 发现的问题

- 登陆多次查询角色和权限
    /admin/sys_login
    select * from sys_user_role 有2次查询
    find_user_permission 和 find_user_role 相同功能
    /admin/sys_user_info
    select * from sys_user_role 有2次查询
- 多次token验证
- 账号管理页面查询角色列表,要优化
- Error 字典太过繁琐,先获取错误信息,然后又获取错误码

# 更新内容
### 1
- ServiceContext 改为Default注入, service 实例 Default::default()
- fast_log 增加一个log_farmat,修改输出文件行号的格式,方便调试
- 密码加密bcrypt
### 2
- SysUserVO和SignInVO去掉password字段
- 更新用户表时,先删除角色再增加角色,改为先更新,再插入
- 修复登陆失败3次时间限制,如果再失败,不要清除次数
- 修复查询用户多次查询角色