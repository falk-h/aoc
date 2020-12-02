use std::time::{Duration, Instant};

pub struct Timer {
    start: Instant,
}

#[macro_export]
macro_rules! expand_pattern {
    {($pattern:literal, $separator:literal), $field:ident} => {
        format!("(?P<{}>({}{})*({})*)", stringify!($field), $pattern, $separator, $pattern)
    };
    {$pattern:literal, $field:ident} => {
        format!("(?P<{}>{})", stringify!($field), $pattern)
    };
}

#[macro_export]
macro_rules! parse_field {
    {$str:expr, ($pattern:literal, $separator:literal)} => {
        $str.split_terminator($separator).map(|s| s.parse().unwrap()).collect()
    };
    {$str:expr, $pattern:literal} => {
        $str.parse()?
    };
}

#[macro_export]
macro_rules! parseable_struct {
    {$name:ident = $pattern:literal {$($field:ident: $type:ty = $fieldpat:tt),+,}} => {
        #[allow(dead_code)]
        #[derive(Debug)]
        struct $name {
            $(
                $field: $type,
            )+
        }

        impl std::str::FromStr for $name {
            type Err = Box<dyn std::error::Error>;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                lazy_static::lazy_static! {
                    static ref RE: regex::Regex = regex::Regex::new(&format!(
                        $pattern
                        $(, expand_pattern!($fieldpat, $field))+
                    )).unwrap();
                }
                let cap = RE.captures(s).unwrap();
                Ok($name {
                    $(
                        $field: {
                            parse_field!(&cap[stringify!($field)], $fieldpat)
                        },
                    )+
                })
            }
        }
    };
}

impl Timer {
    pub fn new() -> Self {
        Timer { start: Instant::now() }
    }

    pub fn print(self) {
        let time = self.start.elapsed();
        print!("Time: ");
        match (time.as_secs(), time.as_millis(), time.as_micros(), time.as_nanos()) {
            (0, 0, 0, ns) => println!("{} ns", ns),
            (0, 0, us, ns) => println!("{}.{} µs", us, ns - 1000 * us),
            (0, ms, us, _) => println!("{}.{} ms", ms, us - 1000 * ms),
            (s, ms, _, _) => println!("{}.{} s", s, ms as u64 - 1000 * s),
        }
    }

    pub fn elapsed(self) -> Duration {
        self.start.elapsed()
    }
}

pub mod input {
    use std::fs;
    use std::str;

    pub fn line(day: u8) -> String {
        fs::read_to_string(format!("inputs/day{}.txt", day)).unwrap().trim().to_string()
    }

    pub fn into<T>(day: u8) -> T
    where
        T: str::FromStr,
        <T as str::FromStr>::Err: std::fmt::Debug,
    {
        line(day).parse().unwrap()
    }

    pub fn lines<T>(day: u8) -> Vec<T>
    where
        T: str::FromStr,
        <T as str::FromStr>::Err: std::fmt::Debug,
    {
        vec(day, "\n")
    }

    pub fn bytes(day: u8) -> Vec<u8> {
        line(day).into_bytes()
    }

    pub fn chars(day: u8) -> Vec<char> {
        line(day).chars().collect()
    }

    pub fn vec<T>(day: u8, separator: &str) -> Vec<T>
    where
        T: str::FromStr,
        <T as str::FromStr>::Err: std::fmt::Debug,
    {
        to_vec(line(day), separator)
    }

    fn to_vec<T>(string: String, separator: &str) -> Vec<T>
    where
        T: str::FromStr,
        <T as str::FromStr>::Err: std::fmt::Debug,
    {
        string.split(separator).filter(|s| s != &"").map(|s| s.parse().unwrap()).collect()
    }

    pub fn matrix<T>(day: u8, separator: &str) -> Vec<Vec<T>>
    where
        T: str::FromStr,
        <T as str::FromStr>::Err: std::fmt::Debug,
    {
        line(day).split_terminator('\n').map(|l| to_vec(l.to_string(), separator)).collect()
    }
}
