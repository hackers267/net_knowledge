{{#title RxJS操作符-基于时间的过滤操作符}}

# 基于时间的过滤操作符

在RxJS中，有几个基于时间的过滤操作符，它们分别是`auditTime`,`debounceTime`,`sampleTime`,`throttleTime`。同时还有`audit`,`debounce`,`sample`和`throttle`四个更通用的过滤操作符。

我们在这里把这几个操作符分别两组：`debounceTime`,`throttleTime`和`auditTime`,`sampleTime`。

第一组分别是防抖和节流。第二组是审核和取样。

先来看看使用频率上比较高的防抖和节流操作符。

## 防抖操作符debounceTime

### 简介

`debounceTime`操作符，它用于对源Observable发出的值进行时间上的过滤。具体来说，当源Observable连续发出多个值时，只有在经过指定的时间间隔内**没有新的值发出**时，才会将**最后**发出的那个值向下传递。

### 背景

在很多实际应用场景中，例如用户在输入框中输入内容时，可能会快速连续地敲击键盘输入字符，这会导致频繁地触发事件并发出数据。但对于一些应用逻辑，比如实时搜索功能，并不需要对每一个输入字符都立即进行处理，而是希望在用户停止一小段时间后(比如几百毫秒)，再根据用户最终输入的完整内容进行搜索操作。`debounceTime`操作符就可以很好地解决这类问题，避免不必要的频繁处理，优化性能并提供更符合用户实际需求的响应。

### 示例

```javascript
import { fromEvent, debounceTime } from "rxjs";

// 获取页面上的输入框元素
const input = document.getElementById("myInput");
// 将输入框的输入事件转换为Observable
const input$ = fromEvent(input, "input");

input$
  .pipe(
    debounceTime(500), // 指定时间间隔为500毫秒，即500毫秒内没有新输入事件，传递当前输入值
  )
  .subscribe((event) => {
    console.log("经过debounceTime处理后的输入值为:", event.target.value);
  });
```

在上述示例中:

1. 首先通过`fromEvent`函数将输入框的`input`事件转换为一个Observable。
2. 然后使用`debounceTime`操作符对这个Observable进行处理，设置时间间隔为500毫秒。
3. 最后订阅处理后的Observable，当满足`debounceTime`的条件时，就会在控制台打印出输入框的值。

### 应用场景

- **实时搜索功能**: 如前面提到的，当用户输入搜索关键词时，避免对每个字符都立即进行搜索，而是等待用户暂停输入一段时间后，根据最终输入的完整关键词进行搜索，提升性能并提供更准确的搜索结果。
- **窗口大小调整事件处理**: 当用户调整浏览器窗口大小时，可能会连续触发多次窗口大小调整事件。使用`debounceTime`可以在用户停止调整窗口大小一段时间后，再根据最终的窗口大小进行相关布局或数据处理操作，避免频繁地重新计算和调整布局。

## 节流操作符throttleTime

### 简介

`throttleTime`的作用是在指定的时间间隔内，只允许源Observable发出的**第一个值**通过，而在该时间间隔内后续发出的值都会被忽略，直到下一个时间间隔开始，才有可能再次让第一个发出的值通过。

### 背景

在一些场景中，例如用户频繁点击按钮执行某个操作(如提交表单、触发某个特效等),可能会由于误操作或快速多次点击导致不必要的多次执行相同操作，这可能会引发上一些问题，比如重复提交表单数据等。`throttleTime`操作符可以用来限制这种操作，确保在一定时间内只有第一次点击等操作被执行，从而避免重复执行带来的不良后果。

### 示例

```js
import { fromEvent, throttleTime } from "rxjs";

// 获取页面上的按钮元素
const button = document.getElementById("myButton");
// 将按钮的点击事件转换为Observable
const buttonClick$ = fromEvent(button, "click");
buttonClick$
  .pipe(
    throttleTime(500), // 指定时间间隔为500毫秒，每500毫秒只允许第一个点击事件通过
  )
  .subscribe(() => {
    console.log("经过throttleTime处理后的点击事件");
  });
```

在上述示例中：

1. 首先通过`fromEvent`函数将按钮的`click`事件转换为一个Observable。
2. 然后使用`throttleTime`操作符对这个Observable进行处理，设置时间间隔为500毫秒。
3. 最后订阅处理后的Observable，当满足`throttleTime`的条件时，就会在控制台打印出相关信息，表示点击事件经过了`throttleTime`处理。

### 应用场景

- **按钮防重复点击**: 在用户点击按钮执行诸如提交表单、保存数据、触发某个重要操作等功能时，使用`throttleTime`可以确保在一定时间间隔内(比如1秒)只有第一次点击被执行，防止用户快速多次点击导致的重复操作问题。
- **滚动事件处理**: 当页面流动时，可能会频繁触发滚动事件。通过`throttleTime`可以限制在一定时间间隔内只处理第一次触发的滚动事件，避免在滚动过程中因频繁处理滚动事件而导致性能下降或执行不必要的操作(如频繁加载更多数据等)。

### 和防抖的区别

- **防抖(debounce)**: 就好比你在等电梯，电梯门要关上的时候，只要有人在关门的那一小会儿时间里又按了开门按钮，电梯门就会重新打开，然后又得等一会儿才会再次准备关门，而且这期间要是还有人按开门按钮，那就继续等。防抖就是这样，当一个事件(比如按电梯开门按钮就好比是某个函数被触发)频繁发生时，它会等这个事件在一段时间内不再发生了，才会真正去执行对应的操作(就像电梯门真正关上，也就是函数真正执行)。
- **节流(throttle)**: 这就好比坐过山车，每隔几分钟就会有一趟车出发。不管这几分钟来了多少人想坐过山车，只有等到规定的时间间隔到了，第一拨人才能上车出发，后面来的人就得等下一个时间间隔了。节流就是在一个事件频繁触发的时候，它会按照固定的时间间隔，只让第一个触发的事件通过去执行对应的操作，在这个时间间隔里后面再触发的事件就不管了，得等下一个时间间隔开始，再看第一个触发的事件能不能执行。

## 审核操作符auditTime

### 简介

`auditTime`会在指定的时间间隔结束时，发出在该时间间隔内源Observable最后发出的那个值(如果在该时间间隔内有值发出的话)。也就是说，不管之后有没有新值发出，只要**时间间隔一到**，就会把该间隔内**最后**出现的值传递给下游的Observer。

### 背景

在实际应用场景中，有时候我们并不想对源Observable发出的每一个值都立即进行处理，而是希望按照一定的时间周期来获取每个周期内的**最后情况**。。比如在监控某个数据变化的场景中，可能数据会频繁变动，但我们只想每隔一段时间了解一下这段时间内最后呈现的数据状态，`auditTime`操作符就可以满足这种需求，它能够帮助我们在时间维度上对数据进行一种"阶段性总结"，避免过于频繁地处理每一个即时数据，从而优化处理流程和节省资源。

### 示例

```js
import { interval, auditTime } from "rxjs";

const source$ = interval(200);

source$.pipe(auditTime(500)).subscribe((value) => {
  console.log("经过auditTime处理的值:", value);
});
```

在上述示例中:

1. 首先通过`interval`函数创建了一个每隔200毫秒就发出一个值的Observable(这里发出的值是从0开始依次递增的整数)。
2. 然后使用`auditTime`操作符对这个Observable进行处理，设置时间间隔为500毫秒。这意味着每500毫秒，`auditTime`操作符会查看在这500毫秒内`interval`发出的最后一个值，并将其传递给下游的订阅者。
3. 最后通过订阅处理后的Observable，在控制台打印出`auditTime`处理后的值。

### 应用场景

- **数据监控和统计**: 在监控服务器性能指标(如CPU使用率、内存占用等)时，这些指标数据可能会频繁更新。使用`auditTime`可以每隔一段时间(如每隔5分钟)获取该时间段内最后记录的指标值，以便进行阶段性的分析和统计，而不需要对每一次的指标变动都立即处理。

- **用户行为分析**: 例如在分析用户在网页上的鼠标轨迹时，鼠标位置可能会不断变化。通过`auditTime`可以每隔一定时间(如每隔10秒)获取该时间段内最后记录的鼠标位置，从而了解用户在不同时间段内的大致行为走向，为用户行为分析提供便利。

### 和节流的不同

- `throttleTime`是在每个设定的时间间隔内，只让**第一个**发出的值通过去执行，重点在每个时间间隔的**开头**那个值。
- `auditTime`是到了设定的时间间隔结束的时候，只取这个时间间隔内Observable **最后**发出的那个值来处理，重点在每个时间间隔的**结尾**那个值。

## 采样操作符sampleTime

### 简介

`sampleTime`按照固定的时间间隔对源Observable进行采样，在每个**采样时刻**，将源Observable当明正在发出的值(如果有的话)传递给下游。简单来说，就是不管Observable发出的值是否频繁，它都会在固定的时间点去"抓取"当时的值并传递下去。

### 背景

在一些场景中，我们需要定期获取某个持续变化的数据源的值，以便进行后续的分析、展示或其他处理。例如，在实时图表绘制中，我们需要每隔一段时间获取当前的数据值来更新图表，展示数据的动态变化过程。`sampleTime`操作符就能满足这种按固定时间间隔获取值的需求，使得我们可以有序地从持续变化的数据流中获取所需的值进行相关操作。

### 示例

```js
import { interval, sampleTime } from "rxjs";

const source$ = interval(200);

source$
  .pipe(
    sampleTime(500), // 每500毫秒采样一次interval的值
  )
  .subscribe((value) => {
    console.log("经过sampleTime处理的值：", value);
  });
```

在上述示例中:

1. 首先利用`interval`函数创建了一个每隔200毫秒发出一个值的Observable(发出的值同样是从0开始依次递增的整数)。
2. 然后运用`sampleTime`操作符对这个Observable进行处理，设置时间间隔为500毫秒。这表示每500毫秒，`sampleTime`操作符会在该时刻去查看`interval`发出的值，并将其传递给下游的订阅者。
3. 最后通过订阅处理后的Observable，在控制台打印出经过`sampleTime`处理后的值。

### 应用场景

- **实时图表绘制**: 如前面提到的，在绘制股票价格走势、气温变化等实时图表时，需要每隔一段时间(如每隔1分钟)获取当前的数据值来更新图表，展示数据的动态变化过程。`sampleTime`操作符就可以很好地满足这种需求，确保图表能够及时、准确地反映数据的实际变化情况。
- **传感器数据采集**: 在工业控制领域，对于各种传感器(如温度传感器、压力传感器等)所采集的数据，通常需要按照固定的时间间隔(如每隔5秒)进行采集并传输到监控系统进行分析处理。`sampleTime`操作符可以用来在固定时间间隔内采集传感器当时发出的值，满足数据采集和传输的要求。

### 和auditTime的区别

- `auditTime`是等到每个时间间隔结束的时候，去看这个时间间隔内源Observable最后发出的那个值，然后把这个值拿去处理，重点在每个时间间隔**结尾**那个值。
- `sampleTime`是按照固定的时间间隔，到那个时间点了就去看源Observable当时正在发出的值，然后把这个值拿去处理，重点在每个时间间隔那个点儿上源Observable**正在**发出的值。

在我们了解了`auditTime`,`debounceTime`,`sampleTim`和`throttleTime`四个操作符之后，其同样的逻辑也适用于`audit`,`debounce`,`sample`和`throttleTime`四个操作符，主要的区别在于其参数不同，前四个的参数都是固定的时间参数，后四个的参数是依据时间发出值的Observable.

## 弹珠图

![auditTime.png](./auditTime.png "auditTime")

<center><b>auditTime弹珠图</b></center>

![sampleTime.png](./sampleTime.png "sampleTime")

<center><b>sampleTime弹珠图</b></center>

![debounceTime.png](./debounceTime.png "debounceTime")

<center><b>debounceTime弹珠图</b></center>

![throttleTime.png](./throttleTime.png "throttleTime")

<center><b>throttleTime弹珠图</b></center>

![audit.svg](./audit.svg "audit")

<center><b>audit弹珠图</b></center>

![sample.png](./sample.png "sample")

<center><b>sample弹珠图</b></center>

![debounce.svg](./debounce.svg "debounce")

<center><b>debounce弹珠图</b></center>

![throttle.svg](./throttle.svg "throttle")

<center><b>throttle弹珠图</b></center>
