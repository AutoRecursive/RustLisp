# RustLisp 互操作性示例

本目录包含展示 Rust 和 Lisp 之间互操作性的示例。

## 从 Rust 调用 Lisp 函数

`interop_example.rs` 展示了如何在 Rust 中定义 Lisp 函数，并从 Rust 代码中调用它们。

运行示例：

```bash
cargo run --example interop_example
```

## 从 Lisp 调用 Rust 函数

`rust_from_lisp.rs` 展示了如何定义 Rust 函数，并从 Lisp 代码中调用它们。

运行示例：

```bash
cargo run --example rust_from_lisp
```

## 主程序

主程序 (`cargo run`) 结合了这两种互操作性，允许您在 REPL 中尝试这两种方式。

在 REPL 中，您可以：

1. 使用 `rust-call` 特殊形式调用 Rust 函数：

```lisp
(rust-call rust-add 1 2 3)
(rust-call rust-multiply 2 3 4)
(rust-call rust-length '(1 2 3))
(rust-call rust-uppercase hello)
(rust-call rust-square 5)
```

2. 定义 Lisp 函数并在 Lisp 中使用它们：

```lisp
(define square (lambda (x) (* x x)))
(square 5)
```

3. 在 Lisp 函数中使用 Rust 函数：

```lisp
(define sum-of-squares 
  (lambda (x y) 
    (+ (rust-call rust-square x) (rust-call rust-square y))))
(sum-of-squares 3 4)
```

4. 在 Rust 代码中调用 Lisp 函数（通过主程序中的 `call_lisp_function`）。

这种双向互操作性使 Rust 和 Lisp 能够无缝协作，结合了两种语言的优势。
