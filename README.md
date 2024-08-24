# mini-redis-zh
mini-redis-zh源码中文注释

## 项目介绍
本项目是[mini-redis](https://github.com/JasonLai256/mini-redis)的中文注释版本，旨在帮助理解redis源码。

## 项目结构
本项目包含以下文件：
- `README.md`：项目介绍文件
- `main.go`：主程序文件，包含启动redis服务器的代码
- `server.go`：服务器文件，包含处理客户端请求的代码
- `client.go`：客户端文件，包含与redis服务器通信的代码
- `protocol.go`：协议文件，包含处理redis协议的代码

## 使用方法
1. 克隆项目到本地
2. 进入项目目录
3. 运行`go run main.go`启动redis服务器
4. 使用redis客户端连接到服务器，例如`redis-cli`或`telnet`

## 注意事项
- 本项目仅用于学习和研究目的，不应用于生产环境
- 本项目中的代码可能存在错误或不完整，请谨慎使用
