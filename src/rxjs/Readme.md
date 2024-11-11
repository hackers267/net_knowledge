{{#title 解决JS异步的另一个选择-RxJS}}
# RxJs

## 简介

RxJS是一个使用可观察序列编写异步和基于事件的程序的库。它提供了一种核心类型，即Observable，一些周边类型(Observer,Scheduler,Subjects)和类似于Array方法(map,filter,reduce,every等)的操作符，以便将异步事件作为集合进行处理。

> 可以将RxJS视为处理事件的Lodash。

RxJS中解决异步事件管理的基本概念有:

- **Observable(可观察者)** : 表示未来(future)值或事件的可调用集合的概念。
- **Observer(观察者)** : 是一个回调集合，它知道如何监听Observable传来的值。
- **Subscription(订阅)** : 表示Observable的一次执行，主要用于取消执行。
- **Operator(操作符)** : 是纯函数，可以使用map,filter,concat,reduce等操作来以函数式编程风格处理集合。
- **Subject(主体)** : 相当于一个EventEmitter,也是将一个值或事件多播到多个Observers的唯一方式
- **Scheduler(调度器)** : 是控制并发的集中化调度器，允许我们在计算发生时进行协调，例如setTimeout或requestAnimationFrame或其它。

## 特性

- Purity(纯净)

RxJS的强大之处在于它能够使用纯函数生成值。这意味着代码不太容易出错。

- Flow(流动)

RxJS有一系列的操作符，可以控制事件如何在observables中流动

- Values(值)

可以通过observables你駏的值进行转换
