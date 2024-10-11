# loader

loader和plugins是webpack中最重要的一部分，其中loader是用来处理各种资源的，比如处理css样式，处理图片，处理字体文件等等。由于需要处理的资源很多，社区中为我们提供了很多常用的loader。此外，我们也可以自定义自己的loader，来实现自己对资源的处理。

## 常用loader

### 处理样式的loader

- less-loader
- sass-loader
- css-loader
- style-loader
- stylus-loader

## loader分类

loader可以分为:

- 同步loader
- 异步loader
- raw loader
- pitch loader

### 同步loader

### 异步loader

### raw loader

### picth loader

## loader的执行顺序

在配置loader的时候，我们可以配置一个叫`enforce`的字段，这个字段的值可以为*pre*和*post*。这个字段会影响到loader的执行顺序。

为了下面的讲解方便，我们约定如下:

`enforce`值为*pre*的字段称之为*pre-loader*，值为*post*的字段为*post-loader*，而没有定义`enforce`字段的值为*normal-loader*.

而`inline-loader`是一种特殊的*loader*，其应用在*import*或*require*的行内,如下所示:

```js
import myModule from 'raw-loader!my-module';
```

上面的示例中的*raw-loader*就是一个*inline-loader*;该loader应用于模块*my-module*。更多关于*inline-loader*的介绍请参考[inline-loader详情](#inline-loader)。

正常情况下的执行顺序是这样的：

```mermaid
flowchart LR
  pre([pre-loader]) --> normal([normal-loader]) --> inline([inline-loader]) --> post([post-loader])
```

对于picth loader的执行顺序则是这样的:

```mermaid
flowchart LR
  post([post-loader]) --> inline([inline-loader]) --> normal([normal-loader]) --> pre([pre-loader])
```

对于`enforce`值相同的loader，其执行顺序是 *从右到左*、*从下到上*。

### inline-loader

`inline-loader`是一种特殊的*loader*，其应用在*import*或*require*的行内,如下所示:

```js
import myModule from 'raw-loader!my-module';
```

上面的示例中的*raw-loader*就是一个*inline-loader*;该loader应用于模块*my-module*。如果想要在一个*import*或*require*行内对模块应用多个*loader*，可以使用 *!* 分隔不同的loader:

```js
import myModule from 'last-loader!middle-loader!first-loader!my-module';
```

在上面的代码中，`my-module`应用了三个不同的行内loader，其执行顺序如下：

```mermaid
flowchart LR
  first(["first-loader"]) --> middle(["middle-loader"]) --> last(["last-loader"])
```

即其依然适用于*先下后上*，*先右后左*的执行顺序。

在*inline-loader*中，可以通过下面的几种方式来阻断一些*loader*的执行：

添加 *!* 阻断*normal-loader*的执行:

```js
import { a } from '!./file1.js'; // 阻断 normal-loader的执行
```

添加 *-!* 阻断*pre-loader*和*normal-loader*的执行:

```js
import { a } from '-!./file1.js'; // 阻断 normal-loader和pre-loader的执行
```

添加 *!!* 阻断 *pre-loader*,*normal-loader*和*post-loader*的执行:

```js
import { a } from '!!./file1.js'; // 阻断pre-loader,normal-loader和post-loader的执行
```
