# 目录
## 介绍
- 2022/10/22
- Actix框架的基本使用

## Actix
- 官网: [https://actix.rs/](https://actix.rs/)
- github: [https://github.com/actix/actix-web](https://github.com/actix/actix-web)


## 学习
- 2022/10/28
- 路由->ok
- Mysql->ok
- Redis->ok
- 中间件->ok
- 文件上传->ok
- 请求传参—>ok
- 路径传参->ok

## 打包
- 获取可以打包的列表
- rustc --print target-list
- rustup target add x86_64-unknown-linux-musl


### Linux
- 请在Linux上打包问题少。
- 默认打包本平台: cargo build --release
- cargo build --release --target=x86_64-unknown-linux-musl