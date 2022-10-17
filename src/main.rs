mod utilities;
mod constants;
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
use std::io::{self, Write};

fn main() {

    println!("{}", constants::MESS_INTRO);
    println!("{}", constants::MESS_INTRO_2);
    print!("{}", constants::MESS_OPT_CHOOSE);
    io::stdout().flush().expect(constants::ERR_MESS_GENERIC);

    let action_chosen = utilities::get_action();
    if action_chosen == 'E' {
        process::exit(1);
    }

    // Expense action
    if action_chosen == constants::EXPENSE {
        // Get expense amount
        let mut amount: String = String::new();

        print!("{}", constants::MESS_ENTER_EXP_AMT);
        io::stdout().flush().expect(constants::ERR_MESS_GENERIC);

        std::io::stdin()
            .read_line(&mut amount)
            .expect(constants::ERR_MESS_AMOUNT_INPUT);

        let amount: i32 = match amount.trim_end().parse::<i32>() {
            Ok(amount) => amount,
            Err(_) => -1
        };

        if amount < 0 {
            println!("{}", constants::ERR_MESS_NOT_INTEGER);
            process::exit(1);
        }

        println!("Entered amount: {}", amount);
    }
}
