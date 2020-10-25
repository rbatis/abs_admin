# abs_admin  语言后台管理模板项目。 此版本接口火热进行中，前端模块开发进行中
*  Rust语言坚如磐石，高性能，无内存泄漏，协程竞争，坚如磐石
*  DDD领域驱动设计
*  ActixWeb，常年屠榜web框架压测网站的框架，Tokio,常用的serde json
*  Mysql,Redis，通用中间件和数据库
*  VueJs，最简单通用低成本的前端解决方案
*  基于[Rbatis](https://github.com/rbatis/rbatis)，和Mybatis-Plus一样的好用，简洁，易扩展的ORM框架
*  JWT Token用户登录模式

# 此项目存在的意义
* 高性能，快如C++, 高内存利用率，运行于廉价服务器环境
* 稳定，无内存泄漏
* 开箱即用

# 快速安装教程
* 1.docker（需安装docker）命令快速启动redis和mysql(用户名root密码123456)。生产docker可以建议部署http服务，原则上生产环境不建议用docker部署数据库
```cmd
docker run -it -d --name redis -p 6379:6379 redis
docker run -d -p 3306:3306 -e MYSQL_ROOT_PASSWORD=123456 --name mysql -e TZ=Asia/Shanghai mysql:5.7
```
* 2.使用MysqlWorkBench或Navcat等工具 导入database.sql脚本到Mysql数据库（mysql用户名密码root  123456）（redis无密码）中

* 3.安装打开PostMan ，导入postman.json到postman中即可使用写好的请求
```cmd
打开postman,导入 postman.json
```
* 4.使用Clion克隆导入abs_admin项目，命令行执行或者点开main.rs点击按钮运行
```cmd
cargo run
```


# module(模块)
* JWT token Auth(基于JWT token的权限鉴权)
* Role,User,Reource（角色，用户，权限）


