{{#title RxJS-combineLatest操作符}}
# combineLatest

组合多个Observable以创建一个Observable,其值是根据其**每个**输入Observable的**最新值**计算得出的。

> 每当**任何**输入Observable发出一个值时，它都会使用来自**所有**输入的**最新值**某一个公式进行求值，然后发出该公式的输出。

![combineLatest](./combineLatest.png "combineLatest弹珠图")

`combineLatest`会组合来自observables**数组**中传递的**所有**Observables的值。这是通过按顺序订阅每位Observable来完成的，并且每当有任何Observable发出时，都会从每位Observable中收集一个包含最新值的数组。因此，如果将n个Observables传递给combineLatest操作符，则返回的Observable将始终发出一个包含n个值的数组，其顺序与传递的Observables的顺序相对应。

combineLatest的静态版本接受一个Observables数组。传递一个空数组将导致Observable立即完成。

为了确保输出数组始终具有相同的长度,combineLatest实际上会等待所有输入Observable**至少发出一次**，然后开始发出结果。这意味着如果某些Observable开始发出之前已经发出了值，那么除了最后一个值之外的所有值都会丢失。另一方面，如果某个Observable没有发出值但完成了，则结果Observable将在同一时刻完成而不发出任何内容，因为现在已不可能再将完成的Observable中的值包含在结果数组中。此外，如果某些输入Observable没有发出任何值并且永远不会完成，combineLatest也将永远不会发出值并且永远不会完成，因为它会一直等待所有流发出一些值。

如果至少一个Observable被传递给combineLatest并且所有传递的Observable都发出了一些东西，那么当所有组合流都已完成时，生成的Observable将完成。因此，即使想地Observable完成，combineLatest的结果仍然会在其它Observable完成时发出值。如果是一个已完成的Observable，从现在开始，它的值将永远是最后一个发出的值。另一方面，如果有任何Observable报错，combineLatest也会立即报错，并且所有其它Observable都将退订。

重点内容如下:

- **最新值**
- **任何Observable发出值都会产生值**  
- **每位Observable至少发出一个值**

## 示例

组合Observables数组

```javascript
import { of, delay, startWith, combineLatest } from 'rxjs';

const observables = [1,5,10].map(
  n => of(n).pipe(
    delay(n*1000),
    startWith(0)
  )
);
const combined = combineLatest(observables);
combined.subscribe(value => console.log(value));
```

组合Observables的字典

```javascript
import { of, delay, startWith, combineLatest } from 'rxjs';

const observables = {
  a: of(1).pipe(delay(1000),startWith(0)),
  b: of(5).pipe(delay(1000),startWith(0)),
  c: of(10).pipe(delay(1000),startWith(0)),
};
const combined = combineLatest(observables);
combined.subscribe(value=>console.log(value));
```
