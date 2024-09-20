/* Create a program that simulates the operation of an ATM. At the beginning, ask the user what the amount 
to be withdrawn will be (integer number), and the program will inform how many bills of each 
denomination will be dispensed */

use dialoguer;

fn main() {
    let mut withdraw: usize = dialoguer::Input::new()
    .with_prompt("Quantity to withdraw")
    .interact()
    .unwrap();

    let mut bills: usize = 50;
    let mut counter: usize = 0;

    loop {
        if withdraw >= bills {
            counter += 1;
            withdraw -= bills;
        } else {
            if counter > 0 {
                println!("You will need {} bills of ${}", counter, bills);
            }
            
            /* This part ensures that all bills are checked until the first 
            condition (the first if statement of the loop) is met; if there are no more bills, the 
            program ends. */
            match bills {
                50 => bills = 20,
                20 => bills = 10,
                10 => bills = 1,
                _ => break
            }

            // Here, the counter is reset to 0 to check if it's necessary to 
            // display another bill of a different value or if the ones already shown are sufficient.
            // ex: | input: $260 |
            /* output:
            You will need 5 bills of $50
            You will need 1 bills of $10 */
            counter = 0;
            
            if withdraw == 0 {
                break;
            }
        }
    }
}