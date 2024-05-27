# todo

- 登陆多次查询角色和权限,前端不需要再请求sys_user_info或者sys_login不需要返回角色和权限,只需要返回token
    /admin/sys_login
    /admin/sys_user_info

- 账号,角色,权限管理页面查询要优化,不用把所有关联都查询出来,前端可以在点开编辑的时候再查询关联数据

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
### 3
- 增加监控 axum-otel-metrics
- 修复查询用户get_user_info多次查询角色权限
- 修复token验证权限
- 修复查询角色没有用到缓存
### 4
- 优化Error
