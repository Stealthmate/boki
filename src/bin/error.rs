use std::process::exit;

pub trait CLIError {
    fn format(&self) -> String;
}

pub trait CLIErrorResult<T> {
    fn or_quit(self) -> T;
}

impl<T, E> CLIErrorResult<T> for Result<T, E>
where
    E: CLIError,
{
    fn or_quit(self) -> T {
        match self {
            Ok(x) => x,
            Err(e) => {
                eprintln!("{}", e.format());
                exit(-1)
            }
        }
    }
}
