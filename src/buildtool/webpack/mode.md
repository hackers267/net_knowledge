# 模式

使用 *mode*来指明我们是在什么模式下开发的。以便让webpack启用对应的优化方式。

*mode* 一般分为以下三种:

- development
- production
- none

我们常用的是*development* 和 *production* 这两种，一个是在我们开发时候使用的模式*development*，一个是我们进行生产打包的时候使用原模式*production*。当我们使用*development*时，webpack的打包速度要快很多，而且默认情况下不会对资源进行压缩处理。而当我们使用*production* 进行打开时，webpack会默认对js和html等资源进行压缩处理。
