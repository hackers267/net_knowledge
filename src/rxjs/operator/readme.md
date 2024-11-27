{{#title RxJS-操作符}}

# 操作符

RxJS的操作符是最有用的。操作符能让你以声明方式轻松组合复杂异步代码的基本构造块。

## 什么是操作符

操作符是函数。有两种操作符：

- 联结操作符
- 创建操作符

可以联入管道的操作符是可以使用语法`observableInstance.pipe(operator())`联入Observables管道的类型。其中包括`filter`和`mergeMap`。调用时，它们不会更改现有的Observable实例。相反，它们返回一个新的Observable，其订阅逻辑是基于第一个Observable的。

> 可联入管道的操作符是一个以Observable作为输入并返回另一个Observable的函数。这是一个纯操作:之前的Observable保持不变.

创建操作符是另一种操作符，可以作为独立函数调用以创建新的Observable。

## 高阶Observables

Observables最常发出的是普通值，如字符串和数字，但令人惊讶的是，它还经常需要处理Observables的Observables，即所谓的高阶Observables。如果你使用高阶Observable,通常需要进行展平处理:(以某种方式)将高阶Observable转换为普通Observable。

## 操作符的分类

有很多用于不同用途的操作符，它们可以分类为:创建、转换、过滤、联结、多播、错误处理、实用工具等。

### 过滤操作符

在rxjs中，过滤操作符可以分为多个类别，其中有一类操作符是基于时间的操作符。下面是主要的基于时间的过滤操作符:

- auditTime
- debounceTime
- sampleTime
- throttleTime
- audit
- debounce
- sample
- throttle`

具体内容可以参考[基于时间的过滤操作符](./filterByTime.md)

除了上面的这类基于时间的操作符，还有一些简易的过滤操作符。下面是主要的简易操作符:

- first
- last
- elementAt
- ignoreElements
