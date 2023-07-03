pub mod entry;
pub mod module;
pub use module::{
    Module,
    ModuleTrait,
    register_domain_and_subdomains_recursively_wrapped,
    ArgsRegisterDomainAndSubdomainsRecursively
};

// Override print!(...) & println!(...) macros
#[macro_export]
macro_rules! println { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
#[macro_export]
macro_rules! print { ($($args:tt)*) => { polywrap_wasm_rs::wrap_debug_log(format!($($args)*).as_str()); } }
