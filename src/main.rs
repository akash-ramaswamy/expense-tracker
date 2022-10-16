/**
 * **************** Expense Tracker in Rust ******************
 * This is a simple project to track your expenses and save it
 * to a file. This is just beginning, which could grow to many
 * features as I get familiar with rust. The following are the
 * initial procedures to have a working skeleton of this game:
 *
 * 1. Take user input for either expense or income.
 * 2. Get amount input.
 * 3. Do the necessary manipulation (add / less).
 * 4. Save the transaction.
 * 5. Show the available balance.
 * 6. Exit.
 *
 * The above are the procedures to be followed to get a working
 * skeleton of the game.
 */
use std::io;

fn main() {
    const EXPENSE: &str = "0";
    const INCOME: &str = "1";
    let mut action_type: String = String::new();
    //let mut amount: String = String::new();

    println!("**************** Expense Tracker in Rust ******************");
    println!("Do you want to record expense or income?");
    println!("0 -> Expense / 1 -> Income");

    io::stdin()
        .read_line(&mut action_type)
        .expect("Error while fetching user input...");

    if action_type.trim_end() == EXPENSE {
        println!("chose to expend");
    } else if action_type.trim_end() == INCOME {
        println!("chose to add income");
    } else {
        println!("Unknown action");
    }
}
