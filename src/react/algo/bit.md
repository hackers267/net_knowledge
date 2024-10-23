{{#title React-位算法}}
# 位算法

位算法是React中使用最多的算法之一。在React中使用位算法不仅仅是为了提高React的性能，还可以利用位的一些操作特性来简化部分运算。

## 概念

位运算是非常底层的算法，是指在计算机科学中，利用位级操作(如与，或，非，异或等)来解决问题的算法。位算法通常在底层编程、性能优化、加密算法和数据压缩等领域中非常有用。

## 具体操作

位运算 | 用法 | 描述
---|---|---
按位与(`&`) | `a & b` | 对于每一个比特位，两个操作数都为1时，结果为1，否则为0
按位或(`\|`) | `a \| b` | 对于每一个比特位，两个操作数都为0时，结果为0，否则为1
按位异或(`^`) | `a ^ b` | 对于每一个比特位，两个操作数相同时，结果为0，否则为1
按位非(`~`) | `a ~ b` | 反转操作数的比特位，即0变1，1变0
左移(`<<`) | `a << b` | 将a的二进制形式向左移b(<32)个比特位，右边用0填充
有符号右移(`>>`) | `a >> b` | 将a的二进制形式向右移b(<32)个比特位，丢弃被移除的位，左侧以最高位填充
无符号右移(`>>>`) | `a >>> b` | 将a的二进制形式向右移b(<32)个比特位，丢弃被移除的位，并用0在左侧填充

位运算的示例这里就不展示了，这里添加一些常用的用法：


### 位清除

使用与运算清除特定位。

```js
const a = 0b1011; // 11
const b = a & ~(1<<1); // 清除第二位
console.log("0b"+b.toString(2).padStart(8,0)); // 0b00001001, 即9
```

### 位设置

使用或运算设置特定位。

```js
const a = 0b1011; // 11
const b = a | (1 << 2); // 设置第三位
console.log("0b"+ b.toString(2).padStart(8,0)); // 0b00001111, 即15
```

### 位翻转

使用异或运算来翻转特定位。

```js
const a = 0b1011; // 11
const b = a ^ 1; // 翻转最低位
console.log("0b"+ b.toString(2).padStart(8,0)); // 0b00001010, 即10
```

### 位测试

使用与运算测试特定位。

```js
const a = 0b1011; // 11
const is_set = (a & (1<<1)) != 0; // 测试第二位是否为1
console.log(is_set); // true
```

## 位掩码

掩码(Mask)在计算机科学和数字电子学中是一种常用工具，用于从一组数据中选择、过滤或修改特定的位。掩码本身是一个二进制数，其中某些位被设置为1，用于指示哪些位是重要的或需要被操作，而其它位被设置为0，表示不重要或不需要被操作的位。

在编程中，位掩码通常用于通过位与(AND)操作来提取、设置或清除特定位。

### 位提取

利用与运算

```js
const a = 0b1101; // 13
const mask = 0b0101; // 用于提取第一位和第三位
const result = a & mask;
console.log("0b"+result.toString(2).padStart(8,0)); // 0b00000101; 即5
```

### 位设置

利用或运算

```js
const a = 0b1101; // 13
const mask = 0b0010; // 设置第二位
const result = a | mask;
console.log("0b"+ result.to_string(2).padStart(8,0)); // 0b00001111; 即15
```

### 清除位

利用与运算和非运算

```js
const a = 0b1101; // 13
const mask = 0b0001; // 清除第一位
const result = a & ~mask;
console.log("0b"+ result.toString(2).padStart(8,0)); // 0b00001100; 即12
```

## 应用

枚举属性：

通过位移的方式，定义一些枚举常量：

```js
const A = 1 << 0; // 0b00000001;
const B = 1 << 1; // 0b00000010;
const C = 1 << 2; // 0b00000100;
```

1. 属性增加`|`

`ABC = A|B|C`

2. 属性删除`& ~`

`AB = ABC & ~C`

3. 属性比较
  1. AB当中包含B:`AB & B === B`
  2. AB当中不包含C:`AB & C === 0`
  3. A和B相等:`A === B`

```js
const A = 1 << 0; // 0b00000001
const B = 1 << 1; // 0b00000010
const C = 1 << 2; // 0b00000100

// 增加属性
const ABC = A | B | C; // 0b00000111
// 删除属性
const AB = ABC & ~C; // 0b00000011

// 属性比较
// 1. AB当中包含B
console.log((AB & B) === B); // true
// 2. AB当中不包含C
console.log((AB & C) === 0); // true
// 3. A和B相等
console.log(A === B); // false
```

### React中的优先级管理

从React17开始，优先级就是使用二进制进行表示的。其巧妙的应用有以下几个:

分离最高优先级(`getHighestPriorityLane`):

```js
function getHighestPriorityLane(lanes: Lanes) {
  return lanes & -lanes;
}
```

通过lanes & -lanes可以分离出所有比特位中最右边的 1.

分离最低优先级(`getLowestPriorityLane`):

```js
function getLowestPriorityLane(lanes: Lanes): Lane {
  // This finds the most significant non-zero bit.
  const index = 31 - clz32(lanes);
  return index < 0 ? NoLanes : 1 << index;
}

```

clz32(lanes)返回一个数字在转换成 32 无符号整形数字的二进制形式后, 前导 0 的个数

- 假设 lanes(InputDiscreteLanes) = 0b0000000000000000000000000011000
- 那么 clz32(lanes) = 27, 由于 InputDiscreteLanes 在源码中被书写成了 31 位, 虽然在字面上前导 0 是 26 个, 但是转成标准 32 位后是 27 个
- index = 31 - clz32(lanes) = 4
- 最后 1 << index = 0b0000000000000000000000000010000
- 相比最初的 InputDiscreteLanes, 分离出来了最左边的1
通过 lanes 的定义, 数字越小的优先级越高, 所以此方法可以获取最低优先级的 lane

## 总结

我们看到，在React当中，官方实现中巧妙的运用位运算，不仅仅提升了React的性能，还使得优先级的处理更加直接和方便。所以，学好算法，不仅仅提升我们的能力，在一些情况下，还可以使得我们对代码的处理更加的优雅。
