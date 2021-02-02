# abs_admin  语言后台管理模板项目。
*  Rust语言坚如磐石，高性能，无内存泄漏，协程竞争，坚如磐石
*  DDD领域驱动设计,Mysql,Redis，通用中间件和数据库，通用企业级框架选型
*  [rbatis-orm](https://github.com/rbatis/rbatis) 和Mybatis-Plus一样的好用，简洁，易扩展的ORM框架
*  [fast_log](https://github.com/rbatis/fast_log) 超快速异步日志框架，支持zip压缩，切割
*  [actix-web](https://actix.rs/) 常年屠榜web框架压测网站的框架
*  前后端分离,基于 [Vue-JS](https://cn.vuejs.org/) +[Vue-AntDesign](https://www.antdv.com/docs/vue/introduce-cn/) + [Vue-AntDesign-Pro](https://pro.antdv.com/)
*  自带JwtToken鉴权登陆，图形验证码登陆，二维码扫码登陆,基础权限管理

# 开发进度
*  (前端-进度5%) 已完成登陆，部分权限模块对接, 路由对接(待开始)，菜单对接(待开始)
*  (后端-进度60%) 部分登陆接口，二维码接口，图形验证码接口，短信登陆模板，部分角色接口，部分用户接口，部分权限接口


# 此项目存在的意义
* 高性能，快如C++, 超低内存占用，支持廉价服务器
* 稳定，部署无忧，无内存泄漏，无闪退
* 开箱即用

# 快速安装教程
* 前端项目链接 https://github.com/rbatis/abs_admin_vue
* 1.docker命令快速启动redis和mysql(用户名root密码123456)。生产docker可以建议部署http服务，原则上生产环境不建议用docker部署数据库
```cmd
docker run -it -d --name redis -p 6379:6379 redis
docker run -d -p 3306:3306 -e MYSQL_ROOT_PASSWORD=123456 --name mysql -e TZ=Asia/Shanghai mysql:5.7
```
* 2.使用MysqlWorkBench或Navicat等工具 导入database.sql脚本到Mysql数据库（mysql用户名密码root  123456）（redis无密码）中

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


