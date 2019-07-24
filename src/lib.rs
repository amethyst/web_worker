use rayon::ThreadPool;
use wasm_bindgen::prelude::*;

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

mod pool;

pub use pool::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn logv(x: &JsValue);
}

pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

pub fn new_thread_pool(concurrency: usize, pool: &WorkerPool) -> ThreadPool {
    rayon::ThreadPoolBuilder::new()
        .num_threads(concurrency - 1)
        .spawn_handler(|thread| Ok(pool.run(|| thread.run()).unwrap()))
        .build()
        .unwrap()
}

