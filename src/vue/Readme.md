# Vue

# 单文件组件

在使用Vue构建前端页面的时候，有一种方式叫做单非年组件的方式，英文名叫*Single File Component*，简称为*SFC*.通过单文件组件的方式，我们可以把一个组件相关的`html`模板，`CSS`样式和`JavaScript`逻辑文件放在同一个文件中。这种做法可以最大程度的做到*高内聚，低耦合*的设计要求。

## html模板

Vue的html模板部分是基于html创建的。其中主要的有以下几点内容:

- 属性绑定
- 方法绑定
- form表单的双向绑定

### 属性绑定

在vue中的一般属性和html中的属性绑定是一样的。但如果你想绑定一些动态属性，就需要使用`v-bind`来绑定属性了。比如下面的代码：

```html
<div v-bind:id="dynamicId"></div>
```

上面代码中`div`元素的`id`值为`dynamicId`的值，其值为`Js`逻辑部分中`data`定义的`dynamicId`的值。

### 插值

在Vue中，我们可以使用 *{{}}* 包含需要的变量就可以。如下面的代码：

```html
<span>Message: {{msg}}</span>
```

### 使用JavaScript表达式

在Vue中，我们可以在所有的数据绑定的地方使用*JavaScript表达式*。比如:

```vue
{{ number + 1 }}  
{{ ok ? "Yes" : "No" }}
{{ message.split("").reverse().join("") }}
<div :id='`list-${id}`'></div>
```

### 指令

指令是Vue中带有前缀的特殊的一种属性。具体内容可以参考[指令](./directive.md);

## CSS样式

Vue的的CSS样式和一般的CSS样式在本质上并没有什么不同。但在单文件组件的设置的样式，只能作用下当前的单文件组件。

## Js逻辑

TODO
