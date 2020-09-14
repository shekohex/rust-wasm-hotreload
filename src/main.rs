use std::sync::{mpsc::channel, Arc};
use std::thread;
use std::time::Duration;

use anyhow::{anyhow, Result};
use atomic_refcell::AtomicRefCell;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use wasmtime::*;

/// Path to the Compiled WASM File from assembly script.
const WASM_FILE_PATH: &str = "./build/optimized.wasm";

/// WASM Module Container is a simple wrapper around the [`Module`] from `wasmtime` crate so it could be shared between threads easily
/// it also contains the Engine that would be used to compile the module.
///
/// It is very cheap to clone the container and be shared between the threads.
#[derive(Clone)]
struct WasmModuleContainer {
    module: Arc<AtomicRefCell<Module>>,
    engine: Engine,
}

impl WasmModuleContainer {
    /// Calling Init will create a new WASM [`Engine`] and using that [`Engine`] to compile the Wasm module
    /// This should only called once at the starting of the program.
    pub fn init() -> Result<Self> {
        let engine = Engine::default();
        let module = Module::from_file(&engine, WASM_FILE_PATH)?;
        Ok(Self {
            engine,
            module: Arc::new(AtomicRefCell::new(module)),
        })
    }

    /// Create an [`Instance`] from the already compiled WASM [`Module`].
    pub fn instance(&self) -> Result<Instance> {
        let module = self.module.borrow();
        let store = Store::new(&self.engine);
        Instance::new(&store, &module, &[])
    }

    /// Reload and Recompile the WASM [`Module`].
    /// Next Calls to [`WasmModuleContainer::instance`] will get a new [`Instance`] with the new compiled code.
    pub fn reload(&self) -> Result<()> {
        println!("Code Changed, Recompile..");
        let module = Module::from_file(&self.engine, WASM_FILE_PATH)?;
        *self.module.borrow_mut() = module;
        println!("Hot Reloaded");
        Ok(())
    }
}

fn main() -> Result<()> {
    let container = WasmModuleContainer::init()?;
    let container2 = container.clone();
    // Start the watching for changes thread.
    thread::spawn(move || watch_for_changes(container2));

    // Main Application Loop.
    loop {
        let instance = container.instance()?;
        let add = instance
            .get_func("add")
            .ok_or_else(|| anyhow!("No function named `add` found in wasm module"))?
            .get2::<i32, i32, i32>()?;

        println!("40 + 2 = {}", add(40, 2)?);

        thread::sleep(Duration::from_millis(200));
    }
}

fn watch_for_changes(container: WasmModuleContainer) -> Result<()> {
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_millis(10))?;
    watcher.watch(WASM_FILE_PATH, RecursiveMode::NonRecursive)?;
    while let Ok(event) = rx.recv() {
        match event {
            DebouncedEvent::Create(_) | DebouncedEvent::Write(_) => {
                container.reload()?;
            }
            _ => {}
        }
    }
    Ok(())
}
