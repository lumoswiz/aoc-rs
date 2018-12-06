use std::fmt::Debug;
use std::str::FromStr;

pub fn split<'a>(input: &'a str) -> impl 'a + Iterator<Item = &'a str> {
    input.trim().split('\n').map(|s| s.trim())
}

pub fn parse<'a, T>(input: &'a str) -> impl 'a + Iterator<Item = T>
where
    T: 'a + FromStr,
    T::Err: Debug,
{
    parse_with(input, |s| s.parse::<T>().unwrap())
}

pub fn parse_with<'a, T, F>(input: &'a str, f: F) -> impl 'a + Iterator<Item = T>
where
    T: 'a,
    F: 'a + Fn(&'a str) -> T,
{
    split(input).map(f)
}
