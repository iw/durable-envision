use std::process::ExitCode;

fn main() -> ExitCode {
    match durable_envision::cli::main() {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: {e:#}");
            ExitCode::from(2)
        }
    }
}
