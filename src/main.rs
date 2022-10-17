mod constants;
mod utilities;

use std::io::{self, Write};
use std::process;
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

fn main() {
    if constants::CURRENT_OS != "linux" && constants::CURRENT_OS != "macos" {
        println!("Unsupported OS..");
        process::exit(0);
    }

    println!("{}", constants::MESS_INTRO);
    loop {
        println!("{}", constants::MESS_INTRO_2);
        print!("{}", constants::MESS_OPT_CHOOSE);
        io::stdout().flush().expect(constants::ERR_MESS_GENERIC);

        let action_chosen = utilities::get_action();
        if action_chosen == 'E' {
            process::exit(1);
        } else if action_chosen == constants::EXIT {
            process::exit(0);
        }

        let mut amount: String = String::new();

        print!("{}", constants::MESS_ENTER_AMT);
        io::stdout().flush().expect(constants::ERR_MESS_GENERIC);

        std::io::stdin()
            .read_line(&mut amount)
            .expect(constants::ERR_MESS_AMOUNT_INPUT);

        let amount: i32 = match amount.trim_end().parse::<i32>() {
            Ok(amount) => amount,
            Err(_) => -1,
        };

        if amount < 0 {
            println!("{}", constants::ERR_MESS_NOT_INTEGER);
            process::exit(1);
        }

        let action = match action_chosen {
            constants::EXPENSE => "Expense",
            constants::INCOME => "Income",
            _ => "",
        };

        println!("Rs.{} {} recorded successfully!\n\n", amount, action);

        let data = utilities::get_transaction_data(
            amount,
            action_chosen,
            utilities::get_current_timestamp(),
        );

        utilities::write_transaction(data);
    }
}
