use floneum_rust::*;
use rustpython_vm::{builtins, stdlib::get_module_inits, Interpreter};

#[export_plugin]
/// Runs a python script. Returns the value in the last line
pub fn run_python(
    /// The python script to run
    source: String,
) -> String {
    Interpreter::with_init(Default::default(), |vm| {
        vm.add_native_modules(get_module_inits());
    })
    .enter(|vm| {
        let scope = vm.new_scope_with_builtins();

        vm.run_block_expr(scope, &source)
            .unwrap()
            .downcast::<builtins::PyStr>()
            .map(|s| s.to_string())
            .unwrap_or_default()
    })
}
