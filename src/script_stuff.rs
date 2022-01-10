use std::io::stdin;

pub enum Event {
    Number(i64),
    Text(String),
}

pub fn print_fancy(text: &str) {
    println!("✨{}✨", text);
}

pub trait ScriptHost {
    type ScriptContext;
    type Error;

    fn new_context() -> Result<Self::ScriptContext, Self::Error>;

    fn handle_event(context: &mut Self::ScriptContext, event: Event) -> Result<(), Self::Error>;
}

pub fn run_main<SH: ScriptHost>() -> Result<(), SH::Error> {
    let stdin = stdin();

    let mut context = SH::new_context()?;

    loop {
        println!("Type number or text to fire an event, or nothing to exit.");
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            break;
        }
        let event = match input.parse() {
            Ok(i) => Event::Number(i),
            Err(_) => Event::Text(input.to_string())
        };

        SH::handle_event(&mut context, event)?;
    }

    Ok(())
}