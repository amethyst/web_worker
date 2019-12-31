//! Utilities to work with web workers and rayon.

use rayon::ThreadPool;
use wasm_bindgen::prelude::*;

/// Macro allowing to write to the js/browser's console.
/// You must import web_worker::log into scope to use this macro.
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

mod pool;

pub use pool::*;

#[wasm_bindgen]
extern "C" {
    /// External binding to the js console.log method. You probably want to use console_log!() instead
    /// of this.
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn logv(x: &JsValue);
}

/// Initializes the logging of panics originating from the rust code.
/// Must be called once at the start of the execution.
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

/// Creates a new `rayon::ThreadPool` from the provided WorkerPool (created in the javascript code)
/// and the concurrency value, which indicates the number of threads to use.
pub fn new_thread_pool(concurrency: usize, pool: &WorkerPool) -> ThreadPool {
    rayon::ThreadPoolBuilder::new()
        .num_threads(concurrency - 1)
        .spawn_handler(|thread| Ok(pool.run(|| thread.run()).unwrap()))
        .build()
        .unwrap()
}

