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
