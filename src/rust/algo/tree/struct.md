{{#title Rust算法-树的表示}}
# 树的表示

在Rust中,我们通常使用`struct`来表示树的结构。使用`struct`表示树非常直观，且不用嵌套。如下表示在Rust中树的struct结构：

```rust
{{#include ../../src/binary_tree.rs::8}}
```

在这之中，我们使用字段`key`来保存字段的值，使用`left`和`right`来保存指向节点的指针。在这里，`Link<T>`类型是`Option<Box<T>>`类型的别名。其中的`T`表示泛型，而使用`Option`表示左右节点可以不存在,使用`Box`是为了在编译时确定`BinaryTree`在内存中占用的大小，否则无法编译通过。

在确定了树在Rust中的基本表示之后，我们就可以为树添加一些插入操作了:

```rust
{{#include ../../src/binary_tree.rs:10:52}}
```

当我们为树添加了插入操作之后，我们就可以构建一个树了，但在这之前，我们还是需要为树添加一些获取和更新值的操作，以便我们可以查看树中某个节点的值。

```rust
{{#include ../../src/binary_tree.rs:53:}}
```

这就是树在Rust中的基本表示。

接下来让我们来看看我们构建的树的效果:

```rust
{{#include ../../src/binary_tree.rs}}
{{#include ../../src/bin/tree_base.rs:3:}}

```
