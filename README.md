# abs_admin
*  rust  common admin server
*  rust语言后台管理

# dep crate(依赖库)
* rbatis 1.3.6
* actix-web 2.0
* tokio 0.2
* serde 1.0
* redis-rs 0.16.0

# database（数据库）
* mysql

# web view manager(web可视化管理)
* vuejs(基于Vuejs)
* element ui(饿了么UI)

# module(模块)
* JWT token based auth(基于JWT token的权限鉴权)
* Role,User,Reource（角色，用户，权限）


# sh(脚本)
```cmd
docker run -it -d --name redis -p 6379:6379 redis
```

# PostMan  请求脚本导入
```cmd
打开postman,导入 abs_admini.postman_collection.json
```
