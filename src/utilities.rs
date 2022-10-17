pub fn get_action() -> char {
    const EXPENSE: &str = "0";
    const INCOME: &str = "1";
    let mut action_type: String = String::new();

    std::io::stdin()
        .read_line(&mut action_type)
        .expect("Error while fetching user input...");

    if action_type.trim_end() == EXPENSE {
        println!("chose to expend");
        return '0';
    } else if action_type.trim_end() == INCOME {
        println!("chose to add income");
        return '1';
    } else {
        println!("Unknown action, Exiting...");
        return 'E';
    }
}