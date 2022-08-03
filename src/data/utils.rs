use std::fmt::Display;
use std::string::ToString;
use strum::IntoEnumIterator;

pub trait StringJoin<T> {
    fn join_string() -> String;
}

impl<T> StringJoin<T> for T
where
    T: Display + IntoEnumIterator,
{
    fn join_string() -> String {
        T::iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }
}
