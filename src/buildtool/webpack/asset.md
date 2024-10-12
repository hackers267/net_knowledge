# 静态资源

在webpack中静态资源包括但不限于以下几种类型：

- 图片
- 视频
- 音频
- 字体
- 文本等

在webpack中，我们对于这样资源的处理会有一些区别。在通常情况下，我们只需要把*视频*,*音频*,*字体*和*文本*等直接复制到目标目录即可。但对于图片，我们可能需要对一些小图片进行*data*编码，使之内嵌到html或css文件中，以减少网络请求。

对于这类静态资源的处理，在webpack5中，为我们提供了以下几种类型：

- [asset/resource](#assetresource)
- [asset/source](#assetsource)
- [asset/inline](#assetinline)
- [asset](#asset)
- javascript/auto

在这几种类型之中，*javascript/auto*类型是一个特殊的类型。其作用是阻止webpack默认对静态资源的处理。比如，当你不想使用webpack5对静态资源的处理方法，而使用一些自定义或都是其它的loader来处理静态资源的时候，你就可以把当前模块的*type*类型指定为*javascript/auto*类型。示例如下:

```js
module.exports = {
  // ...
  module: {
    rules: {
      test: /\.(png|jpg|gif)$/i,
      type: 'javascript/auto',
      use: {
        loader: 'url-loader',
        options: {
          limit: 8 * 1024
        }
      }
    }
  }
}
```

## asset/resource

先看示例代码：

```js
const path = require('path');

module.exports = {
  // 入口配置
  entry: './src/main.js',
  // 输出配置
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: '[name].js'
  },
  module: {
    rules: [
      {
        test: /\.png/i,
        type: 'asset/resource'
      }
    ]
  }
}
```

```js
import mainImage from './images/main.png';

img.src = mainImage; // /dist/151cfcfa1bd74779aadb.png
```

在上面的代码中，webpack会把png图片复制到目标目录并进行重命名。然后把对应的png图片的目标地址转为字符串赋值给`img.src`。

## asset/source

## asset/inline

## asset
