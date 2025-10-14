# 使用 Rust（Actix-web）重写后端的实施计划

本文档面向 AI/开发者执行，给出分阶段、可操作的迁移步骤与注意事项。目标是在不打断现有 Node 后端可用性的前提下，完成核心能力的 Rust 化，并逐步替换现有服务。

## 目标与范围
- 使用 Actix-web 构建新的后端服务。
- 模块化目录：`services`、`utils`、`parser`（迁移现有 `backend/services` 的三个文件至此）、`routes`、`controller`。
- 与现有 Node 后端并行共存（新工程默认放置在 `backend-rs/`，不影响 `backend/`）。
- 保留功能等价：认证、学年/学期/周课程获取、批量课程汇总、固定日程信息导出。
- 兼容 Windows/PowerShell 开发环境；命令不使用 `&&` 串联。

## 目录结构（Rust 新后端）
```
backend-rs/
  Cargo.toml
  src/
    main.rs
    routes/
      mod.rs
      course.rs
    controller/
      mod.rs
      course.rs
    services/
      mod.rs
      course.rs
    parser/
      mod.rs
      auth.rs          # 对应 JS: authService
      schedule.rs      # 对应 JS: scheduleService
      course.rs        # 对应 JS: courseService（批量汇总）
    utils/
      mod.rs
      config.rs
      http.rs          # reqwest 客户端、IPv4 优先策略
      log.rs           # tracing 封装
      response.rs      # 统一响应模型（success/error）
      schedule.rs      # 固定日程（移植 JS 的 utils/schedule.js 内容）
```

> 说明：
> - `parser` 放置“面向外部系统的数据拉取与解析”能力，即原 `backend/services/*.js` 的核心逻辑。
> - `services` 面向业务编排（聚合 parser 输出、并发处理、重试策略等）。
> - `controller` 处理请求入参校验、调用 `services`、输出统一响应。
> - `routes` 仅进行 URL 与 `controller` 绑定。
> - `utils` 放置通用设施：配置、HTTP 客户端、日志、响应模型、常量等。

## 依赖建议（仅文档，实际请按需执行）
在 PowerShell 中执行（不实际运行，仅供实施时参考）：
- 初始化工程
  - `cargo new backend-rs --bin`
- 基础依赖
  - `cargo add actix-web actix-cors`
  - `cargo add serde serde_json --features serde/derive`
  - `cargo add tokio --features macros,rt-multi-thread`
  - `cargo add reqwest --features json,brotli,gzip,deflate,rustls-tls`
  - `cargo add tracing tracing-subscriber`
  - `cargo add anyhow thiserror`
  - `cargo add dotenvy`
- 可选（Puppeteer 替代）
  - `cargo add chromiumoxide --features enable-gen`
  - 可选替代：`cargo add fantoccini`（需要系统安装 ChromeDriver/GeckoDriver）

> 说明：选择 `rustls-tls` 以避免 Windows OpenSSL 依赖问题。

## 环境变量映射
- `FJCPC_APP_BASE_URL` → Rust: `FJCPC_APP_BASE_URL`（默认 `https://app.fjcpc.edu.cn`）
- `TEST_STUDENT_UCODE` → Rust: `TEST_STUDENT_UCODE`
- 可选：`CHROME_PATH`（若使用 `chromiumoxide` 并需要指定浏览器路径）

## Puppeteer 在 Rust 的替代方案
- 首选：`chromiumoxide`（基于 Chrome DevTools Protocol，支持监听网络请求事件，便于截获 Authorization 头）。
- 备选：`fantoccini`（通过 WebDriver；需外部 `chromedriver/geckodriver`）。
- 其他：社区 `playwright-rust`，生态相对不如上两者稳定。

对本项目的需求（打开目标页面、监听请求、捕获 `Authorization` 头），`chromiumoxide` 更直接：
1) 启动无头浏览器；2) 监听 `Network.requestWillBeSent`；3) 命中包含 `Authorization` 的请求头保存；4) 关闭浏览器并返回。

## 实施阶段与步骤

### 阶段 0：准备与并行目录
- 在仓库根目录创建 `backend-rs/` 工程（保留现有 `backend/`）。
- 创建 `docs/`（当前文件所在目录）。

### 阶段 1：工程初始化与基础骨架
1) 生成项目并添加依赖（见“依赖建议”）。
2) 在 `src/` 下创建目录骨架（见“目录结构”）。
3) `main.rs`：
   - 初始化日志（tracing）。
   - 读取环境变量（dotenvy）。
   - 注册路由模块与 CORS。

### 阶段 2：utils 模块
- `config.rs`：
  - 结构体 `AppConfig { college_app_base_url, test_student_ucode, prefer_ipv4 }`。
  - 从 env 读取，提供默认值。
- `http.rs`：
  - 基于 `reqwest::Client` 构建器封装。
  - 预留“IPv4 优先”策略（MVP 可先使用系统默认解析；如需强制 IPv4，可增加自定义解析/直连 IP + Host 头策略，后续迭代）。
- `response.rs`：
  - 定义 `ApiResponse<T> { code, status, data, message }`，提供 `success()` / `error()` 工具函数。
- `log.rs`：
  - `tracing_subscriber` 控制台输出；如需文件落盘，后续增加 `tracing-appender`。
- `schedule.rs`：
  - 迁移 `backend/utils/schedule.js` 的常量为 Rust 常量/结构体。

### 阶段 3：parser 模块（核心能力迁移）
- `auth.rs`（对应 JS: `authService.js`）
  - `get_basic_auth()`：先实现静态 `cat:cat` 的 Basic（与 JS 保持一致，标注“不确定性”）。
  - `get_server_basic_auth(raw_ucode)`：使用浏览器模拟（见下“浏览器模拟”小节）。
  - `get_user_info(raw_ucode)`：
    - 构造 `GET /gateway/auth/oauth/token`，Headers: `Authorization: Basic ...`，Params: `{ ucode: HUA_TENG-<raw>, state:1, grant_type: 'ucode', scope:'server' }`。
    - 401 时调用 `get_server_basic_auth` 重试一次。
    - 返回 `{ access_token, refresh_token, user_info(username/phone/nickName) }`。
- `schedule.rs`（对应 JS: `scheduleService.js`）
  - `get_school_year(user_token)`：映射 `getXn` → 按 JS 逻辑格式化。
  - `get_semester(user_token, school_year, semester)`：映射 `getSemesterbyXn`。
  - `get_week_course(user_token, student_id, start_date)`：映射 `getListByNoWeek2` 并 `parse_course_string`。
  - `parse_course_string(s: &str) -> Option<CourseInfo>`：按 JS 的 `split('|')` 规则解析。
  - `get_school_schedule()`：返回 `utils::schedule` 常量。
- `course.rs`（对应 JS: `courseService.js`）
  - `get_all_courses(user_token, student_id, semester_weeks)`：并发拉取所有周课程，`FuturesUnordered` 聚合，返回 `HashMap<u32, Vec<...>>`。

#### 浏览器模拟（替代 Puppeteer）
- 基于 `chromiumoxide`：
  - 启动浏览器（headless）。
  - 开启网络监听，捕获包含 `Authorization` 的请求头；识别 `Basic` 与 `Bearer` 并缓存。
  - 访问 `${college_app_base_url}/czmobile/mytimetableIndexNew?uid=<ucode>`，等待网络空闲。
  - 关闭浏览器，返回 `{ basic_auth, bearer_auth }`。

### 阶段 4：services 模块（编排）
- `services/course.rs`：基于 `parser::course` 与 `parser::schedule`，对外提供便捷调用（参数校验、日志记录、错误语义封装）。

### 阶段 5：controller 与 routes
- `controller/course.rs`：处理 `GET /api/course`（当前 Node 仅返回 `TEST_STUDENT_UCODE`，MVP 同步之，以便先跑通链路）。
- `routes/course.rs`：注册路由到 Actix。
- 统一错误处理中间件（可在 `main.rs` 或 `utils` 中封装）。

### 阶段 6：运行与最小验证（MVP）
- 运行（PowerShell）：
  - `cd backend-rs`
  - `cargo run`
- 验证：
  - `GET http://127.0.0.1:8080/api/course` 应返回 `{ code, status:"success", data:{ ucode: <env> }, message }`。
- 后续将 `controller/course.rs` 接入 `services/course.rs`，完成等价功能返回。

## 与现有 JS 的功能映射
- `backend/services/authService.js` → `parser/auth.rs`
- `backend/services/scheduleService.js` → `parser/schedule.rs`
- `backend/services/courseService.js` → `parser/course.rs`（批量课程）与 `services/course.rs`（编排）
- `backend/utils/*` → `utils/*`（分别迁移：config、response、schedule、log、api(HTTP 客户端)）
- `backend/routes/courseRouter.js` → `routes/course.rs` + `controller/course.rs`

## 错误处理与重试
- 与 JS 一致：401 触发一次“浏览器模拟”重试（仅 `auth` 流程）。
- 其他错误：直接返回错误；在 `services` 层进行语义化包装，日志按级别记录。

## 性能与并发建议
- `get_all_courses` 使用 `FuturesUnordered` 或 `join_all` 并发拉取；限制并发数（如 `tokio::sync::Semaphore`）防止过载。
- 复用 `reqwest::Client`，避免连接反复建立。

## 风险与注意事项
- `get_basic_auth()` 的静态值存在不确定性；已保留“浏览器模拟”兜底。
- IPv4 强制策略：MVP 阶段先使用默认解析；如遇 AAAA 解析问题，再切换到“直连 IP + Host 头”或自定义解析（后续 issue 跟进）。
- Windows 下浏览器路径：必要时通过 `CHROME_PATH` 指定。
- 使用最新稳定 crate 版本；若出现“方法即将弃用”提示，优先调整到推荐 API。

## 里程碑
1) MVP 可跑通：项目启动 + `/api/course` 回固定 env 值。
2) 接入 `parser/auth`、`parser/schedule`，完成真实接口拉取。
3) 并发优化与错误语义化，替换 Node 端同等接口。
4) 移除 Node 后端或保留为兼容备用。

---

### 附：关键函数签名（建议）
```rust
// utils/response.rs
pub struct ApiResponse<T> { pub code: u16, pub status: String, pub data: T, pub message: String }
```

```rust
// parser/auth.rs
pub async fn get_basic_auth() -> String;
pub async fn get_server_basic_auth(raw_ucode: Option<String>) -> anyhow::Result<String>;
pub async fn get_user_info(raw_ucode: &str) -> anyhow::Result<UserInfo>;
```

```rust
// parser/schedule.rs
pub async fn get_school_year(token: &str) -> anyhow::Result<Vec<SchoolYear>>;
pub async fn get_semester(token: &str, year: &str, sem: &str) -> anyhow::Result<Vec<WeekInfo>>;
pub async fn get_week_course(token: &str, student_id: &str, start_date: &str) -> anyhow::Result<Vec<DayCourse>>;
```

```rust
// parser/course.rs
pub async fn get_all_courses(token: &str, student_id: &str, weeks: &[WeekInfo]) -> anyhow::Result<HashMap<u32, Vec<DayCourse>>>;
```

