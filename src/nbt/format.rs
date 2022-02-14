// These macros can be used to easily format NBT tags into strings

#[macro_export]
macro_rules! format_number {
    ( $tag:expr, $f:expr ) => {
        {
            write!($f, "\"{}\": {},", $tag.name, $tag.value)
        }
    };
}

#[macro_export]
macro_rules! format_array {
    ( $tag:expr, $f:expr ) => {
        {
            write!($f, "\"{}\": [", $tag.name)?;
            for value in &$tag.values {
                write!($f, "{},", value)?;
            }
            write!($f, "],")
        }
    };
}

#[macro_export]
macro_rules! format_string {
    ( $tag:expr, $f:expr ) => {
        {
            write!($f, "\"{}\": \"{}\",", $tag.name, $tag.value)
        }
    };
}

#[macro_export]
macro_rules! format_compound {
    ( $tag:expr, $f:expr ) => {
        {
            write!($f, "\"{}\": {{", $tag.name)?;
            for child in &$tag.children {
                write!($f, "{}", child)?
            }
            write!($f, "}},")
        }
    };
}

#[macro_export]
macro_rules! format_list {
    ( $tag:expr, $f:expr ) => {
        {
            write!($f, "\"{}\": [", $tag.name)?;
            for child in &$tag.children {
                write!($f, "{}", child)?
            }
            write!($f, "],")
        }
    };
}
