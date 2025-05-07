pub mod types;
pub mod eval;
pub mod parser;
pub mod printer;
pub mod environment;
pub mod interop;
pub mod rust_functions;

// Re-export commonly used items
pub use types::{Value, Environment, car, cdr, cons};
pub use eval::eval;
pub use parser::read;
pub use printer::print_value;
pub use environment::setup_environment;
pub use interop::{
    call_lisp_function,
    rust_to_lisp_number,
    rust_to_lisp_symbol,
    rust_to_lisp_bool,
    rust_to_lisp_list,
    lisp_to_rust_number,
    lisp_to_rust_string,
    lisp_to_rust_bool,
    lisp_to_rust_vector,
    register_lisp_function,
};
pub use rust_functions::{
    register_rust_function,
    setup_rust_functions,
    rust_add,
    rust_multiply,
    rust_length,
    rust_uppercase,
    RustFunction,
};
