# FJCPC Course Parser - Rust Backend

福建船政交通职业学院课程解析器 - Rust 后端（使用 Actix-web 框架）

## 项目结构

```
backend/
├── src/
│   ├── controller/      # 控制器层（处理 HTTP 请求）
│   │   ├── course.rs
│   │   └── mod.rs
│   ├── parser/          # 解析器层（与外部 API 交互）
│   │   ├── auth.rs      # 认证相关
│   │   ├── course.rs    # 课程相关
│   │   ├── schedule.rs  # 课表相关
│   │   └── mod.rs
│   ├── routes/          # 路由层（URL 映射）
│   │   ├── course.rs
│   │   └── mod.rs
│   ├── services/        # 服务层（业务逻辑）
│   │   ├── course.rs
│   │   └── mod.rs
│   ├── utils/           # 工具模块
│   │   ├── config.rs    # 配置管理
│   │   ├── http.rs      # HTTP 客户端
│   │   ├── log.rs       # 日志工具
│   │   ├── response.rs  # 响应格式
│   │   ├── schedule.rs  # 课表工具
│   │   ├── simulator.rs # 浏览器模拟器（待完善）
│   │   └── mod.rs
│   ├── lib.rs           # 库入口
│   └── main.rs          # 程序入口
├── tests/               # 集成测试
│   ├── auth_test.rs     # 认证测试
│   ├── course_test.rs   # 课程测试
│   └── schedule_test.rs # 课表测试
├── .env.example         # 环境变量示例
├── Cargo.toml           # 项目配置
└── README.md            # 本文件
```

## 功能特性

- ✅ **高性能**: 使用 Rust + Actix-web，性能远超 Node.js 版本
- ✅ **类型安全**: Rust 的强类型系统确保代码质量
- ✅ **异步并发**: 使用 Tokio 异步运行时，支持高并发
- ✅ **IPv4 强制**: 解决学校 DNS 服务器的 AAAA 记录问题
- ✅ **认证机制**: 支持 Basic Auth 和 Bearer Token
- ⚠️ **浏览器模拟**: 简化版实现（待完善）

## 快速开始

### 1. 安装依赖

确保已安装 Rust 工具链：

```bash
# 安装 Rust（如果尚未安装）
# 访问 https://rustup.rs/ 下载安装

# 验证安装
rustc --version
cargo --version
```

### 2. 配置环境变量

复制 `.env.example` 为 `.env` 并填写配置：

```bash
cp .env.example .env
```

编辑 `.env` 文件：

```env
COLLEGE_APP_BASE_URL=https://czmobile.fjcpc.edu.cn
TEST_STUDENT_UCODE=your_test_student_ucode_here
```

### 3. 编译项目

```bash
# 开发模式编译
cargo build

# 生产模式编译（优化）
cargo build --release
```

### 4. 运行服务器

```bash
# 开发模式运行
cargo run

# 生产模式运行
cargo run --release
```

服务器将在 `http://127.0.0.1:8080` 启动。

### 5. 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test auth_test
cargo test schedule_test
cargo test course_test

# 显示测试输出
cargo test -- --nocapture
```

## API 端点

### GET /api/course

获取课程信息（当前为测试端点）

**响应示例**:
```json
{
  "code": 200,
  "status": "success",
  "data": {
    "ucode": "NOT_SET"
  },
  "message": "Success"
}
```

## 重要注释说明

### 1. 模拟器逻辑

位于 `src/utils/simulator.rs`，这个模拟器的逻辑要始终保留。虽然当前是简化版实现，但框架已经搭建好，未来可以完善。

**为什么需要模拟器？**
- 学校 API 可能随时变化
- 静态认证可能失效
- 需要动态捕获真实的认证信息

### 2. IPv4 强制

位于 `src/utils/http.rs`，强制使用 IPv4 解析 DNS。

**原因**: 船政那个神必 DNS 服务器加了 AAAA 却没法解析的问题 😅

### 3. 认证不确定性

位于 `src/parser/auth.rs`，Basic Auth 使用 "cat:cat"。

**注意**: 根据校内服务器数据推测，存在不确定性。如果失败会自动回退到浏览器模拟。

## 与 Node.js 版本的对比

| 特性 | Node.js 版本 | Rust 版本 |
|------|-------------|-----------|
| 性能 | 中等 | 高 |
| 内存占用 | 较高 | 低 |
| 类型安全 | 弱（TypeScript 可选） | 强 |
| 并发模型 | 事件循环 | 多线程 + 异步 |
| 浏览器模拟 | Puppeteer（完整） | chromiumoxide（简化） |
| 启动速度 | 快 | 中等 |
| 编译时间 | 无 | 较长 |

## 待完善功能

- [ ] 完善浏览器模拟器（chromiumoxide 或 fantoccini）
- [ ] 添加更多 API 端点
- [ ] 实现完整的课程查询功能
- [ ] 添加缓存机制
- [ ] 添加更多单元测试
- [ ] 性能基准测试

## 技术栈

- **Web 框架**: Actix-web 4.x
- **异步运行时**: Tokio 1.x
- **HTTP 客户端**: reqwest (rustls-tls)
- **序列化**: serde + serde_json
- **日志**: tracing + tracing-subscriber
- **错误处理**: anyhow + thiserror
- **浏览器自动化**: chromiumoxide (待完善)

## 开发建议

1. **使用 `cargo watch` 自动重新编译**:
   ```bash
   cargo install cargo-watch
   cargo watch -x run
   ```

2. **使用 `cargo clippy` 检查代码**:
   ```bash
   cargo clippy
   ```

3. **使用 `cargo fmt` 格式化代码**:
   ```bash
   cargo fmt
   ```

4. **查看依赖树**:
   ```bash
   cargo tree
   ```

## 许可证

与主项目保持一致

## 贡献

欢迎提交 Issue 和 Pull Request！

