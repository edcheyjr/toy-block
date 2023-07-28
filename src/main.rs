#[macro_use]
extern crate serde_derive;
extern crate colored;

mod blockchain;

use colored::Colorize;
use std::io;
use std::io::Write;
use std::process;

fn main() {
    let smile = '😁';
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();
    let title = "\t TOY BLOCK CHAIN".red();
    let title_upper_bounds = "\n\n******************************************".green();
    let title_lower_bounds = "******************************************\n\n".green();
    let input_choices_bounds = "----------------------------------------".blue();
    let invalid_arg_msg = "Invalid option please retry".bright_red();

    println!("{}", title_upper_bounds);
    println!("{}", title);
    println!("{}", title_lower_bounds);

    print!("input a miner address:");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_addr);
    print!("Difficulty:");
    io::stdout().flush();
    io::stdin().read_line(&mut difficulty);
    let diff = difficulty
        .trim()
        .parse::<u32>()
        .expect("We need an interger");
    println!("generating genesis block ... {}", smile);

    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);
    loop {
        println!("{}", title_upper_bounds);
        println!("{}", "\t\tMENU".red());
        println!("{} {}", "1)".bright_cyan(), "New Transaction".cyan());
        println!("{} {}", "2)".bright_cyan(), "Mine Crypto".cyan());
        println!("{} {}", "3)".bright_cyan(), "Change Difficulty".cyan());
        println!("{} {}", "4)".bright_cyan(), "Change Reward".cyan());
        println!("{} {}", "0)".bright_cyan(), "Exit".cyan());
        println!("{}", title_lower_bounds);

        print!("Enter your choice:");
        io::stdout().flush();
        choice.clear();
        io::stdin().read_line(&mut choice);
        println!(" ");

        match choice.trim().parse().unwrap() {
            0 => {
                println!("exiting...");
                process::exit(0);
            }
            1 => {
                println!("{}", input_choices_bounds);
                println!("Creating A new Transaction \n");
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("Enter sender address: ");
                io::stdout().flush();
                io::stdin()
                    .read_line(&mut sender)
                    .expect("Receiver Address Not Found");
                print!("Enter receiver address:");
                io::stdout().flush();
                io::stdin()
                    .read_line(&mut receiver)
                    .expect("Receiver Address Not Found");
                print!("Enter amount:");
                io::stdout().flush();
                io::stdin()
                    .read_line(&mut amount)
                    .expect("Amount not found");

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap(),
                );
                match res {
                    true => println!("{}", "Transaction completed".bright_green()),
                    false => println!("{}", "Transaction failed".bright_red()),
                }
                println!("{}", input_choices_bounds);
            }
            2 => {
                println!("{}", input_choices_bounds);
                println!("Mining block \n");
                println!(
                    "{} ,Please wait patiently{}",
                    "mining a block ⛏...".on_bright_magenta(),
                    smile
                );
                let res = chain.generate_new_block();
                match res {
                    true => println!("{}", "block generated completed".bright_green()),
                    false => println!("{}", "Block generation failed".bright_red()),
                }
                println!("{}", input_choices_bounds);
            }
            3 => {
                println!("{}", input_choices_bounds);
                println!("Update Difficulty \n");

                let mut new_diff = String::new();
                print!("Enter new difficulty:");
                io::stdout().flush();
                io::stdin().read_line(&mut new_diff);
                let mut new_diff_no = new_diff
                    .trim()
                    .parse::<u32>()
                    .expect("expects a number without decimals");
                let res = chain.update_difficulty(new_diff_no);
                match res {
                    true => println!("{}", "Updated Difficulty successfully!".bright_green()),
                    false => println!("{}", "Failed to Update Difficulty".bright_red()),
                }
                println!("{}", input_choices_bounds);
            }
            4 => {
                let mut new_reward = String::new();
                println!("{}", input_choices_bounds);
                print!("Enter the new reward:");
                io::stdout().flush();
                io::stdin().read_line(&mut new_reward);
                let mut new_reward_no = new_reward.trim().parse::<f32>().expect("expects a number");
                let res = chain.update_reward(new_reward_no);
                match res {
                    true => println!("{}", "Updated Reward successfully!".bright_green()),
                    false => println!("{}", "Failed to Update Reward".bright_red()),
                }
                println!("{}", input_choices_bounds);
            }
            _ => println!("{}", invalid_arg_msg),
        }
    }

    // print!("Exited! {}", smile);
}
