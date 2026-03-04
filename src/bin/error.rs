use std::process::exit;

pub trait CLIError {
    fn format(&self) -> String;
}

impl<E> CLIError for Box<E>
where
    E: CLIError,
{
    fn format(&self) -> String {
        self.as_ref().format()
    }
}

pub trait CLIErrorResult<T> {
    fn or_quit(self) -> T;
    #[allow(dead_code)]
    fn or_panic(self) -> T;
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

    fn or_panic(self) -> T {
        match self {
            Ok(x) => x,
            Err(e) => {
                eprintln!("{}", e.format());
                panic!("Expected OK.")
            }
        }
    }
}
