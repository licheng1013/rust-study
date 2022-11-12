# 目录

## 介绍
- 开始rust学习
- 2022/10/22
- Actix框架的基本使用
- 本项目包含了大部分示例，请放心食用！

## Actix
- 官网: [https://actix.rs/](https://actix.rs/)
- github: [https://github.com/actix/actix-web](https://github.com/actix/actix-web)


## 学习
- 2022/10/29
- 路由->ok
- Mysql->ok
- Redis->ok
- web中间件编写->ok
- 文件上传->ok
- 请求传参—>ok
- 路径传参->ok
- 多模块设计->ok
- 列表->ok
- map->ok
- 闭包->ok
- 线程->ok
- 多态->ok


## 工作空间
- 项目运行的路径 -> 项目根目录
- 创建包的两种形式！
- bin -> 运行程序
- lib -> 依赖库

## 打包
- 获取可以打包的列表
- rustc --print target-list
- rustup target add x86_64-unknown-linux-musl


### Linux
- 请在Linux上打包问题少。
- 默认打包本平台: cargo build --release
- cargo build --release --target=x86_64-unknown-linux-musl


### 切换环境
- 切换到尝新环境
- rustup override set nightly
- 切换到稳定环境
- rustup override set stable

### 工具
- 修复代码
- cargo fix