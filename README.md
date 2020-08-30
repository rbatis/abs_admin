# abs_admin
*  rust  common admin server
*  rust语言后台管理模板项目，为了急速开发而生

# 此项目存在的意义
* 解决快速开发项目，后台系统几乎是每个应用必须的。rust的优势即 编写完毕，无bug，无内存溢出
* 解决大型复杂项目，例如某公司开发某ERP项目，如果是用Java之类的语言，水平参差不齐的公司越到后期代码量越大，业务越复杂，性能越差。而使用rust起码保证性能不掉
* 为初次使用Rbatis和actix_web的用户提供学习案例

# 快速安装教程
* 使用docker启动mysql和redis
```cmd
docker run -it -d --name redis -p 6379:6379 redis
docker run -p 3306:3306 --name mysql -e MYSQL_ROOT_PASSWORD=123456 -d mysql:5.7
```
* 使用mysql workbench或者navcat导入database.sql脚本到mysql

* PostMan  请求脚本导入
```cmd
打开postman,导入 abs_admini.postman_collection.json
```
* 使用Clion导入abs_admin运行或者命令行执行
```cmd
cargo run
```


# dep crate(依赖库)
* rbatis
* actix-web
* tokio
* serde
* redis-rs

# database（数据库）
* mysql

# web view manager(web后台页面js框架)
* vuejs(基于Vuejs)
* element ui(饿了么UI)

# module(模块)
* JWT token based auth(基于JWT token的权限鉴权)
* Role,User,Reource（角色，用户，权限）


