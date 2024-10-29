{{#title Rust算法-树的遍历}}

# 树的遍历

树的遍历通常可以分为*前序遍历*、*中序遍历*和*后序遍历*。而这三种都是以访问根元素的先后顺序来命名的。

- 前序遍历：根节点 --> 左子树 --> 右节点(**根左右**)
- 中序遍历: 左节点 --> 根节点 --> 右节点(**左根右**)
- 后序遍历: 左节点 --> 右节点 --> 根节点(**左右根**)

## 前序遍历

下面是使用Rust实现的树的前序遍历：

```rust,no_run,noplayground
{{#include ../../src/traverse.rs:3:17}}
```

## 后序遍历

下面是使用Rust实现的树的后序遍历

```rust,no_run,noplayground
{{#include ../../src/traverse.rs:19:33}}
```

## 中序遍历

下面是使用Rust实现的树的中序遍历

```rust,no_run,noplayground
{{#include ../../src/traverse.rs:34:}}
```

当我们了解了树的前序，后序和中序遍历之后，让我们来看看完整的代码和运行结果:

```rust
{{#include ../../src/traverse.rs:3:}}
{{#include ../../src/binary_tree.rs:1:}}
{{#include ../../src/bin/tree_traverse.rs:3:}}
```
