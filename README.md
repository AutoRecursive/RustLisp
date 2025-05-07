# RustLisp

RustLisp 是一个用 Rust 编写的简单 Lisp 解释器，具有 Rust 和 Lisp 之间的双向互操作性。

## 特性

- 基本的 Lisp 解释器功能
- 从 Rust 调用 Lisp 函数
- 从 Lisp 调用 Rust 函数
- 模块化设计
- 简单的 REPL 界面

## 构建和运行

```bash
# 构建项目
cargo build

# 运行 REPL
cargo run

# 运行完整的互操作性演示
cargo run --example full_interop_demo

# 运行从 Rust 调用 Lisp 的示例
cargo run --example interop_example

# 运行从 Lisp 调用 Rust 的示例
cargo run --example rust_from_lisp
```

## 项目结构

- `src/types.rs` - 核心数据类型定义
- `src/eval.rs` - 表达式求值
- `src/parser.rs` - Lisp 代码解析
- `src/printer.rs` - 值打印
- `src/environment.rs` - 环境和原始函数
- `src/interop.rs` - Rust 调用 Lisp 的互操作性
- `src/rust_functions.rs` - Lisp 调用 Rust 的互操作性
- `src/main.rs` - REPL 实现
- `examples/` - 各种示例

## 使用 REPL

启动 REPL 后，您可以输入 Lisp 表达式并查看结果：

```lisp
🦀λ> (+ 1 2 3)
6

🦀λ> (define square (lambda (x) (* x x)))
<lambda>

🦀λ> (square 5)
25
```

## Rust-Lisp 互操作性

### 从 Lisp 调用 Rust 函数

使用 `rust-call` 特殊形式：

```lisp
🦀λ> (rust-call rust-add 1 2 3)
6

🦀λ> (rust-call rust-uppercase hello)
HELLO
```

### 从 Rust 调用 Lisp 函数

```rust
// 定义 Lisp 函数
register_lisp_function("square", "(x)", "(* x x)", env.clone());

// 调用 Lisp 函数
let args = vec![rust_to_lisp_number(5.0)];
let result = call_lisp_function("square", args, env.clone());
```

## 扩展

### 添加新的 Rust 函数

```rust
register_rust_function("my-function", |args| {
    // 实现函数逻辑
    Rc::new(Value::Number(42.0))
});
```

### 添加新的 Lisp 函数

```rust
register_lisp_function(
    "my-function",
    "(x y)",
    "(+ x y)",
    env.clone()
);
```

## 许可证

MIT
