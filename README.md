![demo](demo1.jpg)

# abs_admin

Enterprise-level Admin Backend Solution built with [Rust](https://www.rust-lang.org/)

[中文文档](#中文文档)

## Features

- Rock solid (Rust), high performance, no GC, no memory leaks
- DDD (Domain Driven Design) architecture
- Frontend-backend separation
- RBAC (5-tables) permission control
- JWT authentication

## Tech Stack

**Backend:**
- [axum](https://github.com/tokio-rs/axum) - Web framework
- [rbatis](https://github.com/rbatis/rbatis) - ORM framework
- [fast_log](https://github.com/rbatis/fast_log) - Async logging
- Database: SQLite (default) / MySQL / PostgreSQL
- Cache: Redis (optional)

**Frontend:**
- [Vue.js](https://cn.vuejs.org/) + [Ant Design Vue](https://www.antdv.com/)
- Repo: https://github.com/rbatis/abs_admin_vue

## Progress

| Feature | Status |
| --- | --- |
| Dynamic menu | √ |
| JWT auth | √ |
| Permission management | √ |
| Role management | √ |
| Account management | √ |

## Quick Start

**Start dependencies (optional):**
```bash
# Redis
docker run -d --name redis -p 6379:6379 redis

# MySQL (optional, SQLite used by default)
docker run -d -p 3306:3306 -e MYSQL_ROOT_PASSWORD=123456 --name mysql -e TZ=Asia/Shanghai mysql:5.7
```

**Start backend:**
```bash
cargo run
```

**Start frontend:**
```bash
git clone https://github.com/rbatis/abs_admin_vue
cd abs_admin_vue
yarn install
yarn serve
```

Open: http://localhost:8001

## Configuration

Edit `application.json5` for database connection and other settings.

Default account: `admin` / `123456`

## Test API

Import `postman.json` to Postman for API testing.

## License

Apache-2.0

---

# 中文文档

[Rust](https://www.rust-lang.org/) 企业级一站式后台管理系统

## 特性

- 坚如磐石（Rust 语言），高性能，无 GC，无内存泄漏
- DDD 领域驱动设计
- 前后端分离
- RBAC (5表) 权限控制
- JWT 认证

## 技术栈

**后端:**
- [axum](https://github.com/tokio-rs/axum) - Web 框架
- [rbatis](https://github.com/rbatis/rbatis) - ORM 框架
- [fast_log](https://github.com/rbatis/fast_log) - 异步日志
- 数据库: SQLite (默认) / MySQL / PostgreSQL
- 缓存: Redis (可选)

**前端:**
- [Vue.js](https://cn.vuejs.org/) + [Ant Design Vue](https://www.antdv.com/)
- 仓库: https://github.com/rbatis/abs_admin_vue

## 功能进度

| 功能 | 状态 |
| --- | --- |
| 动态菜单 | √ |
| JWT 认证 | √ |
| 权限管理 | √ |
| 角色管理 | √ |
| 账号管理 | √ |

## 快速开始

**启动依赖服务（可选）:**
```bash
# Redis
docker run -d --name redis -p 6379:6379 redis

# MySQL（可选，默认使用 SQLite）
docker run -d -p 3306:3306 -e MYSQL_ROOT_PASSWORD=123456 --name mysql -e TZ=Asia/Shanghai mysql:5.7
```

**启动后端:**
```bash
cargo run
```

**启动前端:**
```bash
git clone https://github.com/rbatis/abs_admin_vue
cd abs_admin_vue
yarn install
yarn serve
```

访问: http://localhost:8001

## 配置

编辑 `application.json5` 配置数据库连接等参数。

默认账号: `admin` / `123456`

## 测试 API

导入 `postman.json` 到 Postman 进行接口测试。

## License

Apache-2.0
