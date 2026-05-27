# StudnetNetPass

一键登录/登出校园网认证系统。

## 功能

- 自动探测校园网 portal 认证页面
- 一键登录认证
- 一键注销下线
- 会话信息自动保存，无需重复输入凭证

## 工作原理

```
用户执行 login
  │
  ├─ 1. 连接 portal 服务器
  ├─ 2. 获取重定向 URL，解析认证参数
  │    (wlanuserip, wlanacname, mac, nasip, ssid, t, url ...)
  ├─ 3. 从 config.toml 读取学号密码
  ├─ 4. POST 登录请求到 /zportal/login/do
  ├─ 5. 服务器返回 Set-Cookie (JSESSIONID + userIndex)
  └─ 6. 将会话信息保存到 .session 文件

用户执行 logout
  │
  ├─ 1. 读取 .session 文件（Cookie + 用户参数）
  ├─ 2. POST 登出请求到 /zportal/logout
  └─ 3. 删除 .session 文件
```

## 使用方法

### 1. 配置

重命名 `config.toml.example` 为 `config.toml`，填写学号和密码（配置文件需要和工具在同一目录下）：

```toml
[credentials]
username = "202501236"
password = "12345678"
```

### 2. 登录

```bash
studnetnetpass login
```

正常输出：
```
正在登录中......
会话信息已保存，可执行 logout 下线
登录成功! {"message":"","nextPage":"goToAuthResult","result":"success"}
```

### 3. 登出

```bash
studnetnetpass logout
```

正常输出：
```
正在注销下线......
已下线!
```

### 调试模式

```bash
RUST_LOG=debug studnetnetpass login
RUST_LOG=debug studnetnetpass logout
```

## 构建

```bash
# 开发构建
cargo build

# 发布构建（体积优化）
cargo build --release
```

## 网络环境

校园网 portal 服务器为内网地址，认证协议基于 HTTP 表单提交。

## 免责声明

本工具仅供学习和研究校园网认证机制使用。使用者应遵守所在校园网的网络使用规定，合理使用网络资源。作者不对因使用本工具而产生的任何后果承担责任，包括但不限于账号封禁、网络中断或其他损失。

使用本工具即表示您理解并同意以上条款。

## 免费上网实现原理

### 服务端搭建

- https://github.com/xtls/xray-core#others-that-support-vless-xtls-reality-xudp-plux

### 客户端环境

- https://github.com/xtls/xray-core#gui-clients

![pic](./resources/workflow.png)
