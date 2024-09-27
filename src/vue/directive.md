# 指令

指令是带有`v-`前缀的特殊属性。Vue提供了许多[内置指令](#内置指令)，比如:`v-bind`。

指令的期望值是一个JavaScript表达式。一个指令的任务是在其表达式的值变化时响应式的更新DOM.

除了使用[内置指令](#内置指令)外，我们还可以[自定义指令](#自定义指令)

## 内置指令

Vue中有下面的一些内置指令:

- [v-text](#v-text)
- [v-html](#v-html)
- [v-show](#v-show)
- [v-if](#v-if)
- [v-else](#v-else)
- [v-else-if](#v-else-if)
- [v-for](#v-for)
- [v-on](#v-on)
- [v-bind](#v-bind)
- [v-model](#v-model)
- [v-slot](#v-slot)
- [v-pre](#v-pre)
- [v-once](#v-once)
- [v-memo](#v-memo)
- [v-cloak](#v-cloak)

### v-text

v-text是Vue中比较简单的一个指令。也是使用频率比较低的一个指令。因为它的作用和模板内的插值作用是一样的。都是把一个文本内容绑定到一个元素上。

比如:

```html
<span v-text="msg"></span>
<!-- 等价于 -->
<span>{{ msg }}</span>  
```

### v-html

v-html也是Vue中比较简单的一个指令。其的使用频率也并不高。因为它用于把原始的Html绑定到元素上。而直接绑定原始html到元素上，有引用`XSS攻击`的风险。示例如下:

```html
<div v-html="html"></div>
```

### v-show

v-show是在Vue中使用频率最高的一个属性之一。其作用为基于表达式的真假值，来改变元素的可见性。如

```html
<div v-show="display">hello</div>
```

v-show的改变元素的可见性是通过其`display`属性来控制的。其值的可见性为假值时，其`display`值为`none`。并且可见性改变时，其触发过滤效果。

我们需要注意的是，虽然`v-show`可以接收任何类型的值，但从最佳实践来说，`v-show`直接接收布尔值还是最好的。

### v-if,v-else,v-else-if

`v-if`,`v-else`和`v-else-if`是Vue中的条件渲染指令。其中`v-if`和`v-else-if`都接收一个真假值。根据这个真假值来判断是否渲染元素或片段。我们在这里需要知道的是,*v-else*和*v-else-if*不能单独使用，必须配合*v-if*一起使用。

和`v-show`一样，当条件改变时，会触发过渡效果。但是和`v-show`不一样的是，`v-show`是通过元素的CSS迎性`display`来控制元素是否显示。而`v-if`则是通过条件的真假来对元素进行重构或者销毁。其成本要远远比`v-show`高。基于这个原因，在一般情况下，如果一个元素会在显示和隐藏之间反复切换，那么`v-show`就是最好的选择。但如何一个元素在某个条件触发后就被销毁不再出现，那么更好的选择就是使用`v-if`。

在同一个元素上使用`v-if`和`v-for`时，`v-if`具有更高的优先级。但同时在一个元素上使用`v-if`和`v-for`往往会引起歧义。所以，并不推荐在同一个元素上同时使用`v-if`和`v-for`指令。

### v-for

在Vue中，`v-for`用于渲染列表。其指令值必须使用特殊语法`alias in expression`。如

```html
<div v-for="item in items">
  {{item.text}}
</div> 
```

同时，使用`v-for`的时候可以使用索引或索引别名，如何是对象，则是键值。如下:

```html
<div v-for="(item, index) in items"></div>
<div v-for="(value, key) in object"></div>
<div v-for="(value, name, index) in object"></div>
```

在通常的情况下，我们在使用`v-for`时，都需要使用绑定一个`key`属性。这个`key`需要绑定一个唯一值。以标记这个元素的唯一性，有利于提高性能。

### v-on

v-on用于绑定事件监听器。详情请见[事件监听](./event.md)

### v-bind

v-bind用于绑定属性。详情详见[属性绑定](./attr.md)

### v-model

v-model用于表单或元素上创建数据的双向绑定。详情请见[双向绑定](./model.md)

### v-slot

TODO:

### v-pre

TODO:

### v-once

TODO:

### v-memo

TODO:

### v-cloak

TODO:

## 自定义指令

