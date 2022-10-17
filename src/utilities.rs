pub fn get_action() -> char {
    use crate::constants;
    let mut action_type: String = String::new();

    match std::io::stdin().read_line(&mut action_type) {
        Ok(_) => (),
        Err(_) => println!("{}", constants::ERR_MESS_TYPE_ACTION)
    };

    if action_type.trim_end() == constants::EXPENSE.to_string() {
        return '0';
    } else if action_type.trim_end() == constants::INCOME.to_string() {
        return '1';
    } else {
        println!("{}", constants::ERR_MESS_UNKNOWN_ACTION);
        return 'E';
    }
}

pub fn get_current_directory()-> String {
    use std::env;
    use std::path::Path;

    let current_dir = match env::current_dir() {
        Ok(dir) => dir.display().to_string(),
        Err(_) => Path::new("").display().to_string()
    };

    current_dir
}