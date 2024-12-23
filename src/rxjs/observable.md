{{#title RxJS-Observable}}

# Observable

Observable是个**多值**的惰性**Push**集合。

 &nbsp; | 单值 | 多值
---|---|---
拉 | Function | Iterator 
推 | Promise | Observable 

## 拉取和推送

**拉取(Pull)** 和 **推送(Push)** 是两种不同的协议，用于描述**生产者(Producer)**如何与数据 **消费者(Consumer)** 通信。

&nbsp; | 生产者 | 消费者
---|---|---
拉取 | **被动**: 在请求时产生数据 | **主动**:决定何时请求数据
推送 | **主动**: 按照自己的节奏生成数据 | **被动**: 对接收到的数据做出响应 

什么是拉取?在拉取体系中，由消费者确定何时从数据生产者接收数据。而生产本身不知道数据何时交付给消费者。

每个JavaScript函数都是一个拉取系统。该函数是数据的生产者，调用该函数的代码通过从其调用中"拉取"出 **单个** 返回值来使用它。

ES2015引用了生成器函数和迭代器，它是另一种类型的拉取体系。调用`iterator.next()`的代码是消费者，它从迭代器(生产者)"拉取"出 **多** 个值。

什么是推送?在推送体系中，由生产者决定何时向消费者发送数据。而消费者不知道何时会收到该数据。

Promise是当前JavaScript中最常见的推送体系类型。Promise(生产者)向已注册的回调(消费者)传递解析后的值，但与函数不同的是，Promise负责准确确定该值何时"推送"到回调。

RxJS引入了Observables，一个新的JavaScript推送体系。Observable是多个值的生产者，并将它们"推送"给Observer(消费者).

- **函数**是一种惰性求值的计算，它在调用时同步返回单个值。
- **生成器**是一种惰性求值的计算，它在迭代时同步返回零到(可能)无限个值。
- **Promise**是一种可能会(也可能不会)最终返回单个值的计算
- **Observable**是一种惰性求值的计算，从它被调用的那一刻起，它可以同步或异步返回零个到(可能)无限个值。
