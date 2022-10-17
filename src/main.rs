mod utilities;
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
use std::process;

fn main() {
    //let mut amount: String = String::new();

    println!("**************** Expense Tracker in Rust ******************");
    println!("Do you want to record expense or income?");
    println!("0 -> Expense / 1 -> Income");

    let action_chosen = utilities::get_action();
    if action_chosen == 'E' {
        process::exit(1);
    }
}
