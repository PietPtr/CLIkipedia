#[macro_export]
macro_rules! flog {
    ($val:expr) => {{
        use chrono::Local;
        use std::fs::OpenOptions;
        use std::io::Write;

        let now = Local::now();
        let formatted_time = now.format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        let formatted_value = format!("{:#?}", $val);
        let log_entry = format!("{} - {}\n", formatted_time, formatted_value);

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("log.txt")
            .expect("Failed to open or create the file");

        file.write_all(log_entry.as_bytes())
            .expect("Failed to write to the file");
    }};
}

pub fn usize_to_base26(mut num: usize) -> String {
    let mut result = Vec::new();
    while num >= 26 {
        let remainder = num % 26;
        result.push((remainder as u8 + b'a') as char);
        num = num / 26 - 1;
    }
    result.push((num as u8 + b'a') as char);
    result.into_iter().rev().collect()
}

pub fn base26_to_usize(s: &str) -> usize {
    if s.is_empty() || !s.chars().all(|c| c.is_ascii_lowercase()) {
        panic!("Input string '{s}' is empty or contains non-lowercase letters");
    }

    let mut num = 0;
    for c in s.chars() {
        num = num * 26 + (c as usize - 'a' as usize) + 1;
    }
    num - 1
}

#[test]
fn test_base26_conversion() {
    for num in 0..10 {
        let base26 = usize_to_base26(num);
        let result = base26_to_usize(&base26);
        println!("{} -> {} -> {}", num, base26, result);
        assert_eq!(num, result, "Failed for num: {}", num);
    }
}
