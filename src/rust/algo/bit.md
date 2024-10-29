{{#title Rust-位运算}}
# 位运算

## 概念

位运算是非常底层的算法，是指在计算机科学中，利用位级操作(如与，或，非，异或等)来解决问题的算法。位算法通常在底层编程、性能优化、加密算法和数据压缩等领域中非常有用。

## 具体操作

位运算 | 用法 | 描述
---|---|---
按位与(`&`) | `a & b` | 对于每一个比特位，两个操作数都为1时，结果为1，否则为0
按位或(`\|`) | `a \| b` | 对于每一个比特位，两个操作数都为0时，结果为0，否则为1
按位异或(`^`) | `a ^ b` | 对于每一个比特位，两个操作数相同时，结果为0，否则为1
按位非(`!`) | `a ! b` | 反转操作数的比特位，即0变1，1变0
左移(`<<`) | `a << b` | 将a的二进制形式向左移b(<32)个比特位，右边用0填充
右移(`>>`) | `a >> b` | 将a的二进制形式向右移b(<32)个比特位，丢弃被移除的位，左侧以最高位填充

## 示例

### 与运算(AND)

与运算特性：两个位都为1时，结果为1，否则为0;

```rust
let a = 0b1011; // 11
let b = 0b1101; // 13
let c = a & b;
println!("0b{:08b}", c); // 0b00001001, 即9
```

### 或运算(OR)

或运算特性：两个位都为0时，结果为0，否则为1;

```rust
let a = 0b1011; // 11
let b = 0b1101; // 13
let c = a | b;
println!("0b{:08b}", c); // 0b00001111，即15
```

### 非运算(NOT)

非运算特性：反转所有位，将1变0，0变1;

```rust
let a = 0b1011; // 11
let b = !a;
println!("0b{:032b}",b); // 0b1111_1111_1111_1111_1111_1111_1111_0100
```

### 异或运算(XOR)

异或运算特性：两个位相同结果为0，否则为1;

```rust
let a = 0b1011; // 11
let b = 0b1101; // 13
let c = a ^ b;
println!("0b{:08b}",c); // 0b00000110，即6 
```

### 左移运算

左移运算特性:将位向左移动指定的位数，相当于乘以2的n次方。

```rust
let a = 0b1011; // 11
let b = a << 2;
println!("0b{:08b}",b); // 0b00101100, 即44
```

### 右移运算

右移运算特性：将位向右移动指定的位数，相当于除以2的n次方。对于无符号数，右边会填充0；对于有符号数，会填充符号位。

```rust
let a = 0b1011; // 11
let b = a >> 1;
println!("0b{:08b}", b); // 0b00000101, 即5
let c: i32 = -13;
let d = c >> 1;
println!("0b{:08b}", d); // 0b1111_1111_1111_1111_1111_1111_1111_1101,即 -7
```

### 位清除

使用与运算清除特定位。

```rust
let a = 0b1011; // 11
let b = a & !(1<<1); // 清除第二位
println!("0b{:08b}", b); // 0b00001001, 即9
```

### 位设置

使用或运算设置特定位。

```rust
let a = 0b1011; // 11
let b = a | (1 << 2); // 设置第三位
println!("0b{:08b}", b); // 0b00001111, 即15
```

### 位翻转

使用异或运算来翻转特定位。

```rust
let a = 0b1011; // 11
let b = a ^ 1; // 翻转最低位
println!("0b{:08b}", b); // 0b00001010, 即10
```

### 位测试

使用与运算测试特定位。

```rust
let a = 0b1011; // 11
let is_set = (a & (1<<1)) != 0; // 测试第二位是否为1
println!("{}",is_set); // true
```

## 位掩码

掩码(Mask)在计算机科学和数字电子学中是一种常用工具，用于从一组数据中选择、过滤或修改特定的位。掩码本身是一个二进制数，其中某些位被设置为1，用于指示哪些位是重要的或需要被操作，而其它位被设置为0，表示不重要或不需要被操作的位。

在编程中，位掩码通常用于通过位与(AND)操作来提取、设置或清除特定位。

### 位提取

利用与运算

```rust
let a = 0b1101; // 13
let mask = 0b0101; // 用于提取第一位和第三位
let result = a & mask;
println!("0b{:08b}",result); // 0b00000101; 即5
```

### 位设置

利用或运算

```rust
let a = 0b1101; // 13
let mask = 0b0010; // 设置第二位
let result = a | mask;
println!("0b{:08b}", result); // 0b00001111; 即15
```

### 清除位

利用与运算和非运算

```rust
let a = 0b1101; // 13
let mask = 0b0001; // 清除第一位
let result = a & !mask;
println!("0b{:08b}", result); // 0b00001100; 即12
```

## 位计数

### 计算二进制中1的个数

```rust
/// 使用Brian Kernighan算法计算二进制中1的个数
fn count_ones(n: u32) -> u32 {
  let mut count = 0;
  let mut num = n;
  while num!=0 {
    num &= num -1;
    count+=1;
  }
  count
}
fn main() {
  let number = 0b1101;
  let count = count_ones(number);
  println!("count is {}",count);
}
```

Brian Kernighan算法的工作原理是不断地将输入数与比它小1的数进行按位与操作，这样可以逐个清除最低位的1.每次操作后，都会检查结果是否为0，如果不是，则计数器加1.这个过程一直持续到数变为0.

在这个算法中，利用了一个数-1，会导致这个数的二进制发生以下变化:

- 最低位的1变为0
- 比最低位的1还低的位的0变1
- 比最低位的1高的位不变

然后利用这个变化，再与原数进行与运算，就会清除最低位的1.不断的利用这个性质就可以把这个数的所有的1都清除。而利用这个性质的次数就是这个数中二进制的1的个数。

这个算法的时间复杂度是 **O(k)** ，其中*k*为二进制中1的个数。最坏的时间复杂度为 **O(n)**,这个时候，二进制中都是1.

### 计算二进制中前导1的个数

```rust
fn lead_ones(n:isize) -> usize {
  let mut count = 0;
  let mut number = n;
  let bits = std::mem::size_of::<isize>() * 8;
  let mut total = 0;
  for i in (0..bits).rev() {
    if number & (1<<i) == 0 {
      total += 1;
    }else {
      break
    }
  }
  for i in (0..(bits-total)).rev() {
    if number & (1<<i) != 0 {
      count += 1;
    } else {
      break
    }
  }
  count
}

fn main() {
  let number = 0b1100_1100;
  let count = lead_ones(number);
  println!("count is {}",count);
}
```

在这个算法的实现上，我们两次使用了**位掩码**的性质，第一次是计算1出现的最高位在哪儿，第二次是计算在最高位后连续出现了多少个1，即有多少个前导1.


## 位运算优化

利用位运算的短路特性来优化算法。例如，使用`|=`操作来设置标志位，如果该位已经是1，则不会进行多余的操作。

## 技巧

### 判断一个数是不是偶数

```rust
fn is_even(n:u32) -> bool {
  n & 1
}
fn main() {
  let is = is_even(4);
  let not = is_even(3);
  println!("is is {} and not is {}",is,not);
}
```