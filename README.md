# abs_admin  语言后台管理模板项目。
*  Rust语言坚如磐石，高性能，无内存泄漏，协程竞争，坚如磐石
*  DDD领域驱动设计,Mysql,Redis，通用中间件和数据库，通用企业级框架选型
*  [rbatis-orm](https://github.com/rbatis/rbatis) 和Mybatis-Plus一样的好用，简洁，易扩展的ORM框架
*  [fast_log](https://github.com/rbatis/fast_log) 超快速异步日志框架，支持zip压缩，切割
*  [actix-web](https://actix.rs/) 常年屠榜web框架压测网站的框架
*  前后端分离,基于 [Vue-JS](https://cn.vuejs.org/) +[Vue-AntDesign](https://www.antdv.com/docs/vue/introduce-cn/) + [Vue-AntDesign-Pro](https://pro.antdv.com/)
*  RBAC权限控制,自带JwtToken鉴权登陆，图形验证码登陆，二维码扫码登陆,基础权限管理

# 开发进度（包含前/后端）
* Web前端动态菜单(根据后台权限动态生成)（√）
* JWT Token登陆（账号密码登陆（√）图形验证码登陆（x）二维码扫码登陆（x）短信登陆（x））
* 设置/权限管理（父子级，权限可缓存redis）（√）
* 设置/角色管理（父子级+权限树）（√）
* 设置/后台账号管理（角色树）（x）
* 设置/键值对常量管理(x)
* JWT Token拦截器校验（x）


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
* 4.使用Clion克隆导入abs_admin项目，点开main.rs点击按钮运行.或执行命令:
```cmd
cargo update
cargo run
```


# module(模块)
* JWT token Auth(基于JWT token的权限鉴权)
* Role,User,Reource（角色，用户，权限）


