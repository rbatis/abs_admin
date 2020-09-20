# abs_admin  语言后台管理模板项目。 此版本接口火热进行中，前端模块开发进行中
*  rust  common admin server
*  基于Rust语言，急速开发，高性能，无内存泄漏，协程竞争，坚如磐石
*  DDD领域驱动设计，基于后台开发最常见领域驱动设计
*  基于ActixWeb，屠榜国外web框架压测网站的框架，Tokio,常用的serde json
*  基于Mysql,Redis，最通用的中间件和数据库
*  基于VueJs，最简单通用低成本的前端解决方案

# 此项目存在的意义
* 快速开发迭代无忧，后台系统几乎是每个应用必须的。rust的优势即 编写完毕，无bug，无内存溢出，泄漏
* 复杂高并发无忧，例如某公司开发某ERP项目，如果是用Java之类的语言，水平参差不齐的公司越到后期代码量越大，复杂业务也导致内存暴涨，业务越复杂，性能越差。而使用rust起码保证性能不掉，内存没问题
* 易上手，为初次使用Rbatis和actix_web的用户提供学习案例
* 符合传统Java后台设计模式，习惯。同时还保持安全+高性能

# 快速安装教程
* 使用docker启动redis和mysql(用户名root密码123456)
```cmd
docker run -it -d --name redis -p 6379:6379 redis
docker run -d -p 3306:3306 -e MYSQL_ROOT_PASSWORD=123456 --name mysql -e TZ=Asia/Shanghai mysql:5.7
```
* 使用mysql workbench或者navcat导入database.sql脚本到mysql

* PostMan  请求脚本导入
```cmd
打开postman,导入 postman.json
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
* JWT token Auth(基于JWT token的权限鉴权)
* Role,User,Reource（角色，用户，权限）


