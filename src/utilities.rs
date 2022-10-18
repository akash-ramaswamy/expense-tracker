pub fn get_action() -> char {
    use crate::constants;
    let mut action_type: String = String::new();

    match std::io::stdin().read_line(&mut action_type) {
        Ok(_) => (),
        Err(_) => println!("{}", constants::ERR_MESS_TYPE_ACTION),
    };

    if action_type.trim_end() == constants::EXPENSE.to_string() {
        return '0';
    } else if action_type.trim_end() == constants::INCOME.to_string() {
        return '1';
    } else if action_type.trim_end() == constants::EXIT.to_string() {
        return '3';
    } else {
        println!("{}", constants::ERR_MESS_UNKNOWN_ACTION);
        return 'E';
    }
}

pub fn get_current_directory() -> String {
    use std::env;
    use std::path::Path;

    let current_dir = match env::current_dir() {
        Ok(dir) => dir.display().to_string(),
        Err(_) => Path::new("").display().to_string(),
    };

    current_dir
}

pub fn get_current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    let time_now: u64 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(time) => time.as_secs(),
        Err(_) => 0,
    };

    time_now
}

pub fn write_transaction(data: String) {
    use std::fs::OpenOptions;
    use std::io::BufWriter;
    use std::io::Write;
    use std::path::Path;

    let file_path: &str = &(get_current_directory() + crate::constants::LINUX_OS_DATA_FILE_PATH);

    if Path::new(file_path).exists() == false {
        println!("The file doesn't exist, tried to verify: {}", file_path);
        // process::exit(1);
    }
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .expect(crate::constants::ERR_MESS_FOPEN);

    let mut file = BufWriter::new(file);
    file.write_all(data.as_bytes())
        .expect(crate::constants::ERR_MESS_FWRITE);
}

pub fn get_transaction_data(amount: i32, action: char, timestamp: u64) -> String {
    let mut data_to_be_written = String::new();
    data_to_be_written.push_str(
        &(action.to_string() + " " + &amount.to_string() + " " + &timestamp.to_string() + "\n"),
    );
    data_to_be_written
}
