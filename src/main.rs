#[macro_use]
extern crate serde_derive;

mod blockchain;

use blockchain::BlockChain;
use std::io;
use std::io::Write;
use std::process;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("Input your miner address: ");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_addr);
    print!("Difficulty: ");
    io::stdout().flush();
    io::stdin().read_line(&mut difficulty);
    let diff = difficulty
        .trim()
        .parse::<u32>()
        .expect("Difficulty must be an integer!");
    println!("Generating first block...");
    let mut chain = BlockChain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("Menu");
        println!("0) Exit");
        println!("1) Make a New Transaction");
        println!("2) Mine Block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        print!("Enter your choice: ");
        io::stdout().flush();
        choice.clear();
        io::stdin().read_line(&mut choice);
        println!("");

        match choice.trim().parse().unwrap() {
            0 => {
                println!("Thank you! You safely close your console now!");
                process::exit(0);
            }
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("Please enter a sender address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut sender);
                print!("Please enter a receiver address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut receiver);
                print!("Please enter amount: ");
                io::stdout().flush();
                io::stdin().read_line(&mut amount);

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap(),
                );

                match res {
                    true => println!("Transaction processed!"),
                    false => println!("Transaction could not be processed!"),
                }
            }
            2 => {
                println!("Generating block...");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Block generated successfully!"),
                    false => println!("Block could not be generation!"),
                }
            }
            3 => {
                let mut new_diff = String::new();
                print!("Please enter a new difficulty: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_diff);
                let res = chain.set_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("Difficulty updated!"),
                    false => println!("Difficulty could not be updated!"),
                }
            }
            4 => {
                let mut new_reward = String::new();
                print!("Please enter new reward: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_reward);
                let res = chain.set_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("Reward updated!"),
                    false => println!("Reward could not be updated!"),
                }
            }
            _ => println!("Invalid option, please retry..."),
        }
    }
}
