#[macro_export]
macro_rules! ser_command { // Command serializer
    ($($x:expr),*) => {{
        let mut args = String::new();
        $(
            args.push_str($x);
            args.push(' ');
        )*
        args.trim_end().to_string() // Trim trailing space
    }};
}

#[macro_export]
macro_rules! deser_command {
    ($s:expr) => {{
        let parts: Vec<&str> = $s.split_whitespace().collect();
        parts
    }};
}
