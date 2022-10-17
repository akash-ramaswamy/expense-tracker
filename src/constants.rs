use std::env;

pub const EXPENSE: char = '0';
pub const INCOME: char = '1';
pub const CURRENT_OS: &str = env::consts::OS;
pub const LINUX_OS_DATA_FILE_PATH: &str = "/src/data.txt";
// Error messages
pub const ERR_MESS_TYPE_ACTION: &str = "Error while fetching user input for Expense / Income...";
pub const ERR_MESS_UNKNOWN_ACTION: &str = "Unknown action, Exiting...";
pub const ERR_MESS_AMOUNT_INPUT: &str = "Error while fetching user input for Amount...";
pub const ERR_MESS_NOT_INTEGER: &str = "Entered invalid value, please enter integer!!!";
pub const ERR_MESS_GENERIC: &str = "ERROR OCCURRED...";
// Console messages
pub const MESS_INTRO: &str = "**************** Expense Tracker in Rust ******************";
pub const MESS_INTRO_2: &str = "Do you want to record expense or income?";
pub const MESS_OPT_CHOOSE: &str = "0 -> Expense / 1 -> Income: ";
pub const MESS_ENTER_EXP_AMT: &str = "Enter Expense amount: ";
