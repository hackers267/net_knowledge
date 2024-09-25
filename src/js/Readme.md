# Js相关知识和问题

### var,let,const之间有什么关系和区别

在Js中，var,let和const都是用于声明变量的方法。其中，let和const都是es6新提出的方法。其区别可以从以下几个方面进行分析：

- 作用域
- 暂时性死区[#1](#暂时性死区)
- 变量提升[#2](#变量提升)
- 变量声明


在Js中，没有let和const之前，只存在两个作用域：全局作用域和函数作用域。并不存在块级作用域。但let和const的出现使Js有了块级作用域。即在代码块中如果使用`var`声明变量，则可以在代码块外可以访问变量。而使用`let`和`const`则不可以。比如:

```js
{
  var a = 1;
  let b = 2;
  const c = 3;
}
// 此处可以访问a,访问b和c会报错: Uncaught ReferenceError: b is not defined
```

下面再来说说变量提升的问题：在Js中，存在两个变量提升的状况，其中一下是`var`声明的变量，另外一个就是`function`声明的函数。这里只说说`var`声明的变量，其它变量提升的情况可以参考[变量提升](#变量提升)的相关内容。

在下面的代码中，使用`var`声明一个变量`a`，但是在声明之前对`a`进行了访问。在这种情况下，Js并没有报错，而是输出`a`的值为`undefined`。说明这个时候，变量`a`已经存在，但还没有赋值。而在`var`声明之后，再次访问`a`，则输出结果为`3`，说明这个时候，a不仅仅被声明了，而且a还被赋值了。在Js中这种现象就是变量提升。

```js
console.log(a);
var a = 3;
console.log(a);
```

而与`var`的变量提升对应的则是`let`和`const`的暂时性死区。同样是上面的代码,把`var`的变量声明修改为`let`或者`const`，则运行程序就会报错：`Uncaught ReferenceError: b is not defined`; 因为在`let`声明变量之前，不能在对应的作用域中访问其声明的变量，一旦在这个区域中访问变量，程序就会报错。人们把在作用域开始前到`let`声明变量之间的这个区域称之为`暂时性死区`。而`var`和`let`,`const`之间如何的不同，原因是在Js中，顶层对象的属性和全局变量是等价的。而`var`声明的变量既是全局变量，又是顶层变量。而对于`var`来说，只存在全局作用域和函数作用域，这就导致了除非你是在函数内使用`var`声明了变量，否则，其会自动成为全局变量和顶层对象的属性。这就是说，全局变量和顶层对象的属性是同步文化的。

在浏览器中，因为顶层对象`window`是不可删除和重新赋值的。所以可以基本认为访问全局变量就是在访问顶层对象上的属性。而在`Node`中，由于顶层对象`global`可以被重新赋值，但是在`global`被重新赋值后，全局变量还是存在的，不会被`global`的重新赋值影响。所以在`node`中，访问全局对象并不能直接理解为访问顶层对象的属性。

因为对象`var`不存在块级作用域，所以在执行效果上，在一段代码外是不是加`{}`是没有什么影响的。但对于`let`和`const`来说就不一样。先来看看下面代码的执行结果:

```js
var a = 3;
let b = 3;
const c = 3;
{
  var a = 4;
  let b = 4;
  const c = 4;
  console.log(a); // 4
  console.log(b); // 4
  console.log(c); // 4
}
console.log(a); // 4
console.log(b); // 3
console.log(c); // 3
```

对于上面的执行结果，我们对于括号内的结果没有什么疑问。但是为什么在括号外，a是4，而b和c都是3呢？原因就是`var`没有块级作用域，而`let`和`const`有块级作用域。所以上面的代码和下面的代码是等价的:

```js
var a = 3;
let b = 3;
const c = 3;
var a = 4;
{
  let b = 4;
  const c = 4;
  console.log(a); // 4
  console.log(b); // 4
  console.log(c); // 4
}
console.log(a); // 4
console.log(b); // 3
console.log(c); // 3
```

我们通过这次的代码知道在访问a之前，a已经被重新声明和赋值为`4`，所以两次访问`a`结果都是`4`。而`b`和`c`都是访问的其作用域内对应的值。

在这里我们注意到，使用`var`可以对一个变量进行重新声明。但是，`let`和`const`就不可以，下面的代码就会报错:

```js
let a = 1;
let a = 2; //SyntaxError: Identifier 'a' has already been declared
const b = 1;
const b = 2; // SyntaxError: Identifier 'b' has already been declared
```

`const` 除了不能重新声明之外，还有一个需要注意的点就是：`const`声明变量之后，必须立即赋值(即声明和赋值必须在同一个访问中完成)。

如果使用`const`定义变量时不对交易进行赋值，则会出现`Uncaught SyntaxError: Missing initializer in const declaration`的错误。而且，`const`变量一旦声明，其值就不再改变(主要针对[基础类型](#基础类型)).

对于基础类型来说，使用`const`声明的变量就是常量。但对于复杂类型来说，`const`只能保证变量引用的内容地址不变，并不能保证其值不变。因为变量保存的只是一个引用。比如:

```js
const a = 1; // a 之后不能改变，因为a使用了const声明并是一个基础类型
const b = {}; // b 的地址不能改变，但其值可能改变，因为b虽然使用了const声明，但是保存的是一个引用
console.log(b); // {}
b.a = 3;
console.log(b); // { a : 3}
```

## 变量提升

TODO:

## 暂时性死区

TODO:

## 基础类型

在Js中，主要有下面几种基础类型：

- Boolean
- String
- Number
- Null
- Undefined
- Symbol

这几个类型本身都是比较简单的，其复杂性在于其类型转换，其中比较而言，简单的有`Boolean`,`Null`和`Undefined`，其次是`Number`和`String`,而`Symbol`则是在平时编程中比较少用到的。

### Boolean

在Js中，Boolean只有两个值`true`和`false`,但是在Js中，其他类型的值都可以转为布尔值。其中，只有以下的值转为`false`(通常称之为假值):

- ''(空字符串)
- 0
- +0
- -0
- NaN
- undefined
- null
- false

其它的值都为真值(转为布尔值时为`true`)

### Null

`null`在Js中表示一个对空对象的引用,其表示没有指向任何对象。

#### 类型转换

- ToString 'null'
- ToNumber 0
- ToBolean false

#### 比较

在使用`==`比较的时候，`null`只与`null`和`undefined`比较为`ture`，其它都为`false`。

## 类型转换

## 特殊值

在Js中，存在以下几个特殊值和特殊情况:

- null
- undefined
- NaN

### null

1. `null` 只和 `null` 本身严格相等
2. `null` 和 `null`  与 `undefined` 相等
3. `null` 的 `typeof` 的值为`object`.

### undefined

1. `undefined` 只和 `undefined` 本身严格相等
2. `undefined` 和 `null` 与 `Undefined` 相等
3. `undefined` 的 `typeof` 的值为 `undefined`

### NaN

1. NaN 不和任何值相等。包括其本身
2. `NaN` 的 `typeof` 的值为 `number`. 其是一个特殊的数值类型
3. 判断一个值是否为`NaN`,可以使用全局函数`isNaN`或是`Number`的方法`Number.isNaN`
