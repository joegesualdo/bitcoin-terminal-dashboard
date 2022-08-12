// Source: https://github.com/TianyiShi2001/hhmmss/blob/main/src/lib.rs
pub fn format_duration(s: i64) -> String {
    let mut neg = false;
    let mut s = s;
    if s < 0 {
        neg = true;
        s = -s;
    }
    let (h, s) = (s / 3600, s % 3600);
    let (m, s) = (s / 60, s % 60);
    // Show hours only if there is any
    if h > 0 {
        format!("{}{:02}:{:02}:{:02}", if neg { "-" } else { "" }, h, m, s)
    } else {
        format!("{}{:02}:{:02}", if neg { "-" } else { "" }, m, s)
    }
}

pub fn format_number_string(ss: String) -> String {
    let split_string = ss.split_once(".");
    if let Some((first_string, second_string)) = split_string {
        let mut o_s: String = first_string.to_string().clone();
        let mut s = String::new();
        let mut negative = false;
        let values: Vec<char> = o_s.chars().collect();
        if values[0] == '-' {
            o_s.remove(0);
            negative = true;
        }
        for (i, char) in o_s.chars().rev().enumerate() {
            if i % 3 == 0 && i != 0 {
                s.insert(0, ',');
            }
            s.insert(0, char);
        }
        if negative {
            s.insert(0, '-');
        }
        return format!("{}.{}", s, second_string);
    } else {
        let mut o_s: String = ss.clone();
        let mut s = String::new();
        let mut negative = false;
        let values: Vec<char> = o_s.chars().collect();
        if values[0] == '-' {
            o_s.remove(0);
            negative = true;
        }
        for (i, char) in o_s.chars().rev().enumerate() {
            if i % 3 == 0 && i != 0 {
                s.insert(0, ',');
            }
            s.insert(0, char);
        }
        if negative {
            s.insert(0, '-');
        }
        return s;
    }
}

pub fn format_number(num: u64) -> String {
    let num_string = format!("{}", num);
    format_number_string(num_string)
}

pub fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i64.pow(decimals) as f64;
    (x * y).round() / y
}
