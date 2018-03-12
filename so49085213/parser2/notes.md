Are you sure that works? I tried ```let f = "Hello"; println!(f); ``` and
got 'error: expected a literal'. As far as I'm aware a literal is
the actual value appearing in the source code. In your match statement
you create a new variable called literal but its not actually a
lietral value, hence the error.

#[macro_export]
#[stable(feature = "rust1", since = "1.0.0")]
#[allow_internal_unstable]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::_print(format_args!($($arg)*)));
}

[macro_export]
#[stable(feature = "rust1", since = "1.0.0")]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! concat {
    ($($e:expr),*) => ({ /* compiler built-in */ });
    ($($e:expr,)*) => ({ /* compiler built-in */ });
}
