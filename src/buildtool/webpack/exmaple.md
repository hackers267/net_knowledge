# 配置示例

## 开发配置示例

```js
const path = require("path");

module.exports = {
  // 入口配置
  entry: "./src/main.js",
  // 输出配置
  output: {
    path: path.resolve(__dirname,'./dist'),
    filename: '[name].js',
    clean: true
  },
  // 模块配置
  module: {
    rules: [
      {
        test: /\.css$/,
        ues: ['style-loader','css-loader']
      },
      {
        test: /\.less$/,
        use: ['style-loader','css-loader','less-loader'],
      },
      {
        test: /\.s[ac]ss$/,
        use: ['style-loader','css-loader','sass-loader'],
      },
      {
        test: /\.styl$/,
        use: ['style-loader','css-loader','stylus-loader']
      },
      {
        test: /(png|gif|svg|jp[e]g)$/,
        type: 'asset',
        parser: {
          dataUrlCondition: {
            maxSize: 8 * 1024 // 8kb
          }
        }
      }
    ]
  }
}
```

## 生产配置示例
