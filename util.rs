pub mod input {
    use std::fs;
    use std::str;

    pub fn line(day: u8) -> String {
        fs::read_to_string(format!("inputs/day{}.txt", day))
            .unwrap()
            .trim()
            .to_string()
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
        string
            .split(separator)
            .filter(|s| s != &"")
            .map(|s| s.parse().unwrap())
            .collect()
    }

    pub fn matrix<T>(day: u8, separator: &str) -> Vec<Vec<T>>
    where
        T: str::FromStr,
        <T as str::FromStr>::Err: std::fmt::Debug,
    {
        line(day)
            .split_terminator('\n')
            .map(|l| {
                to_vec(l.to_string(), separator)
            })
            .collect()
    }
}
