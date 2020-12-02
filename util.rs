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

    pub fn line(file: &str) -> String {
        fs::read_to_string(file).unwrap().trim().to_string()
    }

    pub fn into<T>(file: &str) -> T
    where
        T: str::FromStr,
        <T as str::FromStr>::Err: std::fmt::Debug,
    {
        line(file).parse().unwrap()
    }

    pub fn lines<T>(file: &str) -> Vec<T>
    where
        T: str::FromStr,
        <T as str::FromStr>::Err: std::fmt::Debug,
    {
        vec(file, "\n")
    }

    pub fn bytes(file: &str) -> Vec<u8> {
        line(file).into_bytes()
    }

    pub fn chars(file: &str) -> Vec<char> {
        line(file).chars().collect()
    }

    pub fn vec<T>(file: &str, separator: &str) -> Vec<T>
    where
        T: str::FromStr,
        <T as str::FromStr>::Err: std::fmt::Debug,
    {
        to_vec(line(file), separator)
    }

    fn to_vec<T>(string: String, separator: &str) -> Vec<T>
    where
        T: str::FromStr,
        <T as str::FromStr>::Err: std::fmt::Debug,
    {
        string.split(separator).filter(|s| s != &"").map(|s| s.parse().unwrap()).collect()
    }

    pub fn matrix<T>(file: &str, separator: &str) -> Vec<Vec<T>>
    where
        T: str::FromStr,
        <T as str::FromStr>::Err: std::fmt::Debug,
    {
        line(file).split_terminator('\n').map(|l| to_vec(l.to_string(), separator)).collect()
    }
}
