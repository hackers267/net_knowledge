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

## 自定义指令

