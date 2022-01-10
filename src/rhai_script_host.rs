use std::path::PathBuf;
use rhai::{AST, Dynamic, Engine, EvalAltResult, Scope};
use crate::script_stuff::{Event, print_fancy, ScriptHost};

pub struct RhaiScriptHost;

impl ScriptHost for RhaiScriptHost {
    type ScriptContext = (Engine, Scope<'static>, AST);
    type Error = Box<EvalAltResult>;

    fn new_context() -> Result<Self::ScriptContext, Self::Error> {
        let mut engine = Engine::new();
        engine.register_fn("print_fancy", print_fancy);
        let mut scope = Scope::new();
        let name = "james";
        scope.push("name", name);

        let ast = engine.compile_file_with_scope(&mut scope, PathBuf::from("scripts/rhai-sample.rhai"))?;
        // if there's some global state or on-boot handling, make sure it runs
        engine.run_ast_with_scope(&mut scope, &ast)?;
        Ok((engine, scope, ast))
    }

    fn handle_event((engine, scope, ast): &mut Self::ScriptContext, event: Event) -> Result<(), Self::Error> {
        let argument = match event {
            Event::Number(number) => Dynamic::from(number),
            Event::Text(text) => Dynamic::from(text),
        };
        engine.call_fn(scope, ast, "handle_event", (argument,))?;
        Ok(())
    }
}