macro_rules! import_formats {
    ($($name:ident->$ext:literal),*) => {
        $(pub mod $name;)*
        #[derive(Copy, Clone, PartialEq, Debug)]
        pub(crate) enum Format {
            $($name),*
        }
        impl<S: std::fmt::Display> std::convert::From<S> for Format {
            fn from(v: S) -> Self {
                match &*format!("{}", v) {
                    $(stringify!($name) => Self::$name,)*
                    _ => panic!("Unkown format {}", v),
                }
            }
        }
        impl From<Format> for String {
            fn from(v: Format) -> String {
                String::from(match v {
                    $(Format::$name => stringify!($name)),*
                })
            }
        }
        impl Format {
            pub fn get_ext(self) -> &'static str {
                match self {
                    $(Format::$name => $ext),*
                }
            }
        }
    };
}

import_formats!(Hex->"hex");
