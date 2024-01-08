cfg_if::cfg_if! {
    if #[cfg(feature = "wasmer")] {
        mod lib_wasmer;
        pub use lib_wasmer::*;
    } else if #[cfg(feature = "component")] {
        mod lib_component;
        pub use lib_component::*;
    } else {
        compile_error!("'wasmer' or 'component' feature must be enabled");
    }
}
