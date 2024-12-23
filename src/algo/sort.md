# 排序

排序算法是算法的基础之一。常见的排序算法有:

- 冒泡排序
- 插入排序
- 选择排序
- 希尔排序
- 归并排序
- 快速排序
- 堆排序
- 计数排序
- 桶排序
- 基数排序

## 冒泡排序(Bubble Sort)

- **特点**:通过重复遍历待排序的迷茫，比较每对相邻元素的大小，并在必要时元素它们的位置。
- **时间复杂度**:平均和最坏情况都是*O(n<sup>2</sup>)*,最好情况是*O(n)*(当数列已经是有序时).
- **空间复杂度**:*O(n)*
- **稳定性**:稳定排序。

示例代码:

```rust
fn main() {
  let mut nums = [1,2,3,4,5];
  bubble_sort1(&mut nums);
  println!("sorted nums is {:?}",nums);
  bubble_sort2(&mut nums);
  println!("sorted nums is {:?}",nums);
}

fn bubble_sort1(nums: &mut [i32]) {
  for i in 1..nums.len() {
    for j in 0..nums.len()-i {
      if nums[j] < nums[j+1] {
        nums.swap(j,j+1)
      }
    }
  }
}

fn bubble_sort2(nums: &mut [i32]) {
  let mut len = nums.len() - 1;
  while len > 0 {
    for i in 0..len {
      if nums[i] > nums[i+1] {
        nums.swap(i,i+1)
      }
    }
    len -= 1;
  }
}
```

## 选择排序(Selection Sort)

- **特点**:从未排序的元素中找到最小(或最大)的元素，放到已排序序列的末尾。
- **时间复杂度**:*O(n<sup>2</sup>)*
- **空间复杂度**:*O(1)*
- **稳定性**:不稳定排序


代码示例:

```rust
fn main() {
  let mut nums = [2,3,4,1,5];
  selection_sort(&mut nums);
  println!("nums is {:?}",nums);
}

fn selection_sort(nums: &mut [i32]) {
  let len = nums.len();
  for i in 0..len {
    let mut index = i;
    let mut max = nums[index];
    for j in i+1..len {
      if nums[j] > max {
        max = nums[j];
        index = j;
      }
    }
    nums.swap(i,index);
  }
}
```

## 插入排序(Insertion Sort)

- **特点**:构建有序序列，对未排序数据，在已排序序列中从后向前扫描，找到相应位置并插入
- **时间复杂度**:平均和最坏情况都是*O(n<sup>2</sup>)*，最好情况是*O(n)*(当数列已经有序时)
- **空间复杂度**:*O(1)*
- **稳定性**:稳定排序

示例代码:

```rust
fn main() {
  let mut nums = [2,5,3,4,1];
  insertion_sort(&mut nums);
  println!("sorted nums is {:?}",nums);
}

fn insertion_sort(nums: &mut [i32]) {
  let len = nums.len();
  for i in 1..len {
    for j in 0..i {
      let current = nums[i];
      if current < nums[j] {
        move_right(nums,j,i);
        nums[j] = current;
      }
    }
  }
}
fn move_right(nums: &mut [i32], start: usize,end:usize) {
  let mut current = end;
  while current > start {
    nums[current] = nums[current - 1];
    current -=1;
  }
}
```

## 希尔排序(Shell Sort)

- **特点**:是插入排序的一种更高效的改进版本，也称为缩小增量排序。
- **时间复杂度**:取决于增量序列，平均情况是*O(nlogn)*,最坏情况是*O(n^2)*
- **空间复杂度**:*O(1)*
- **稳定性**:不稳定排序

## 归并排序(Merge Sort)

- **特点**: 采用分治法，将已有序的子序列合并得到完全有所的序列。
- **时间复杂度**:*O(nlogn)*
- **空间复杂度**:*O(n)*,需要额外的空间来存储临时数组。
- **稳定性**:稳定排序

## 快速排序(Quick Sort)

- **特点**:通过一个基准值将数据分为两部分，一部分数据比基准值小，另一部分数据比基值大，火递归地排序这两部分。
- **时间复杂度**:平均情况是*O(nlogn)*,最坏情况是*O(n<sup>2</sup>)*(当输入数组已经有序或几乎有序时)
- **空间复杂度**: *O(logn)* 
- **稳定性**: 不稳定排序

## 堆排序(Heap Sort)

- **特点**: 利用堆这种数据结构所设计的一种排序算法，堆是一个近似完全二叉树的结构，并同时满足堆积的性质:即子节点的键值或索引总是小于(或者大于)它的父节点。
- **时间复杂度**: *O(nlogn)* 
- **空间复杂度**: *O(1)* 
- **稳定性**: 不稳定排序

## 计数排序(Counting Sort)

- **特点**: 非比较型排序算法，适用于一定范围内的整数排序
- **时间复杂度**: *O(n+k)* ,k是整数的范围
- **空间复杂度**: *O(k)* 
- **稳定性**: 稳定排序

## 桶排序(Bucket Sort)

- **特点**: 将数组分为有限数量的桶里，每个桶再分别排序
- **时间复杂度**: *O(n+k)*,其中*n*是数组的元素数量，*k*是桶的数量
- **空间复杂度**: *O(n+k)* 
- **稳定性**: 稳定排序

## 基数排序(Radix Sort)

- **特点**: 按照低位先排序，然后收集；再按照高位排序，然后再收集，以此类推，直到最高位。
- **时间复杂度**: *O(nk)* ，其中*n*是数组的元素数量，*k*是数组中最长数字的位数。
- **空间复杂度**: *O(n+k)* 
- **稳定性**: 稳定排序

