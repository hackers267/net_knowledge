# Webpack

webpack是前端构建中优秀的打包工具。

## 配置

webpack配置中最主要的配置有下面几部分：

- entry [入口配置](./entry.md)
- output[输出配置](./output.md)
- module(模块配置)
- plugins(插件配置)
- mode(模式配置)

## 脑图

```mermaid
mindmap
  webpack
    entry("entry(入口)")
      单入口
      多入口
    output-输出
      path
      filename
      clean
    module-模块
      rules
        loader
          常用loader
          自定义loader
          loader分类
    plugins-插件
      常用插件
      自定义插件
        插件的生命周期
    mode-模式
      开发模式-development
      生产样式-production
    开发服务器-devServer
      host
      port
      open
      hot
```

## 配置示例

开发配置

```js
  
```

生产配置

```js
  
```
