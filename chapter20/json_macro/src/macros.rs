pub use std::collections::HashMap;
pub use std::boxed::Box;

#[allow(dead_code)]
#[derive(Clone, PartialEq, Debug)]
pub enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

#[macro_export]
macro_rules! json {
    (null) => {
        $crate::Json::Null
    };
    ([ $( $element:tt ),* ]) => {
        $crate::Json::Array(vec![ $( json!($element) ),* ])
    };
     ({ $( $key:tt : $value:tt ),* }) => {
        {
            let mut fields = $crate::macros::Box::new(
                $crate::macros::HashMap::new());
            $ ( fields.insert(String::from($key), json!($value)); )*
            $crate::Json::Object(fields)
        }
    };
    (true) => {
        $crate::Json::Boolean(true)
    };
    (false) => {
        $crate::Json::Boolean(false)
    };
    ($other:tt) => {
        $crate::Json::from($other)
    };
}

impl From<bool> for Json {
    fn from(b: bool) -> Json {
        Json::Boolean(b)
    }
}

impl From<String> for Json {
    fn from(s: String) -> Json {
        Json::String(s)
    }
}

impl<'a> From<&'a str> for Json {
    fn from(s: &'a str) -> Json {
        Json::String(String::from(s))
    }
}

macro_rules! impl_from_num_to_json {
    ( $( $t:ident )* ) => {
        $(
            impl From<$t> for Json {
                fn from(n: $t) -> Json {
                    Json::Number(n as f64)
                }
            }
        )*
    };
}

impl_from_num_to_json!(u8 i8 u16 i16 u32 i32 u64 i64 usize isize f32 f64);
