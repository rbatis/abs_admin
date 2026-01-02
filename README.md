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
| Dictionary management | √ |
| Captcha | √ |
| File upload/download | √ |
| Storage (Local/OSS) | √ |
| Cache (Memory/Redis) | √ |

## Quick Start

```bash
git clone https://github.com/rbatis/abs_admin.git
cd abs_admin
cargo run
```

Open: http://localhost:8000

Frontend source: https://github.com/rbatis/abs_admin_vue

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
| 字典管理 | √ |
| 图片验证码 | √ |
| 文件上传/下载 | √ |
| 存储服务（本地/OSS） | √ |
| 缓存服务（内存/Redis） | √ |

## 快速开始

```bash
git clone https://github.com/rbatis/abs_admin.git
cd abs_admin
cargo run
```

访问: http://localhost:8000

前端源码: https://github.com/rbatis/abs_admin_vue

## 配置

编辑 `application.json5` 配置数据库连接等参数。

默认账号: `admin` / `123456`

## 测试 API

导入 `postman.json` 到 Postman 进行接口测试。

## License

Apache-2.0
