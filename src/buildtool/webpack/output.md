# 输出配置

webpack中的输出配置是由`output`字段配置的,其是一个object。其中我们经常配置的字段值有两个：

- [path](#path)
- [filename](#filename)

## path

*path* 字段决定打包后文件的存放路径，其值是一个绝对路径。打包后通过的资源都存放于该路径下。其配置如下:

```js
const path = require('path');

module.exports = {
  // 入口文件
  entry: './src/main.js',
  // 输出配置
  output: {
    // 输出的基准路径
    path: path.resolve(__dirname, 'dist'),
  }
}
```

在上面的示例代码中，在使用webpack打包后，所有的资源都是存放于项目的`dist`目录中。

## filename

*filename* 是webpack打包*javascript及其相关资源* 后生成的文件的文件名称，对于单入口文件，我们可以直接指定是文件名称即可。如:

```js
const path = require('path');

module.exports = {
  // 入口配置
  entry: './src/main.js',
  // 输出配置
  output: {
    // 基准路径
    path: path.resolve(__dirname, 'dist'),
    // 文件名称
    filename: 'main.js'
  }
  // 其它必要配置
}
```

在上面的示例中，我们直接指定了输出的文件的名称为`main.js`。

对于多入口文件，我们是不能直接指定输出的文件名称的，因为如果我们直接指定为文件名称，由于只有一个指定的名称，后打包的文件后覆盖之前打包的文件，会导致文件丢失。所以，对于多入口配置，我们需要使用webpack为我们提供的一些[名称变量](#名称变量)来为输出文件命名。示例如下:

```js
const path = require('path');

module.exports = {
  // 入口配置
  entry: {
    main: './src/main.js',
    app: './src/app.js',
  },
  // 输出配置
  output: {
    // 输出文件的基准路径
    path: path.resolve(__dirname, 'dist'),
    // 输出文件的文件名
    filename: '[name].js'
  }
  // 其它必要配置
}
```

## clean

输出配置中的 *clean* 选项用于决定在生成新的输出文件前是可清空之前的文件或如何清空之前的文件。其接收一个布尔值或一个对象: `boolean | {dry?: boolean, keep?: RegExp | string| ((filename:string)=>boolean)}`

示例如下：

当 *clean* 的值为`true`时，webpack会清空输出目录的所有的已有文件。

```js
module.exports = {
  // ...
  output: {
    clean: true, // 在生成新的输出文件前，清空之前生成的文件
  }
}
```

当 *clean* 的值为 `{dry: true}`时，webpack不会清空之前生成的文件，但是会在控制台中输出哪些文件应该移除

```js
module.exports = {
  // ..
  output: {
    clean: {
      dry: true // 在控制台中输出应移除的文件而不清空目录中的文件
    }
  }
}
```

当 *clean* 的值为 `{ keep: RegExt | string | ((filename:string) => boolean) }`时,webpack会根据配置的保留某些结果而移除其他的内容。

```js
module.exports = {
  // ...
  output: {
    clean: {
      keep: 'index.html', // 保留输出目录中的index.html文件，移除其它文件
    }
  }
}
```

```js
module.exports = {
  // ...
  output: {
    clean: {
      keep: /\.ico$/, // 保留输出目录中的以 *.icon*为扩展名的文件，移除其它文件
    }
  }
}
```

```js
module.exports = {
  // ...
  output: {
    clean: {
      keep(filename) {
        return filename.endsWith('.icon'); // 保留输出目录中的以 *.icon*为扩展名的文件，移除其它文件
      }
    }
  }
}
```

在示例中我们可以得出：

- 当 *keep* 为字符串时，webpack会保留与字符串完全匹配的文件
- 当 *keep* 为正则表达式时,webpack会保留符合正则表达式的文件
- 当 *keep* 为一个断言函数时，webpack会保留断言函数为真的文件

## 名称变量
