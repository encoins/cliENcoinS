//! Deals with the terminal UI
use std::io;
use std::io::Write;

/// Prints the terminal UI
pub fn show_terminal(strings_to_show : &Vec<String>)
{
    print!("{esc}c", esc = 27 as char);
    print_logo();
    println!();
    for string_ts in strings_to_show
    {
        println!("{}",string_ts);
    }

    println!("\n\nEnter a command : ");
    print!("> ");
    io::stdout().flush().unwrap();
}


/// Prints the encoins (c) logo
pub fn print_logo()
{
    println!("
          _____                    _____                    _____                   _______                   _____                    _____                    _____
         /\\    \\                  /\\    \\                  /\\    \\                 /::\\    \\                 /\\    \\                  /\\    \\                  /\\    \\
        /::\\    \\                /::\\____\\                /::\\    \\               /::::\\    \\               /::\\    \\                /::\\____\\                /::\\    \\
       /::::\\    \\              /::::|   |               /::::\\    \\             /::::::\\    \\              \\:::\\    \\              /::::|   |               /::::\\    \\
      /::::::\\    \\            /:::::|   |              /::::::\\    \\           /::::::::\\    \\              \\:::\\    \\            /:::::|   |              /::::::\\    \\
     /:::/\\:::\\    \\          /::::::|   |             /:::/\\:::\\    \\         /:::/~~\\:::\\    \\              \\:::\\    \\          /::::::|   |             /:::/\\:::\\    \\
    /:::/__\\:::\\    \\        /:::/|::|   |            /:::/  \\:::\\    \\       /:::/    \\:::\\    \\              \\:::\\    \\        /:::/|::|   |            /:::/__\\:::\\    \\
   /::::\\   \\:::\\    \\      /:::/ |::|   |           /:::/    \\:::\\    \\     /:::/    / \\:::\\    \\             /::::\\    \\      /:::/ |::|   |            \\:::\\   \\:::\\    \\
  /::::::\\   \\:::\\    \\    /:::/  |::|   | _____    /:::/    / \\:::\\    \\   /:::/____/   \\:::\\____\\   ____    /::::::\\    \\    /:::/  |::|   | _____    ___\\:::\\   \\:::\\    \\
 /:::/\\:::\\   \\:::\\    \\  /:::/   |::|   |/\\    \\  /:::/    /   \\:::\\    \\ |:::|    |     |:::|    | /\\   \\  /:::/\\:::\\    \\  /:::/   |::|   |/\\    \\  /\\   \\:::\\   \\:::\\    \\
/:::/__\\:::\\   \\:::\\____\\/:: /    |::|   /::\\____\\/:::/____/     \\:::\\____\\|:::|____|     |:::|    |/::\\   \\/:::/  \\:::\\____\\/:: /    |::|   /::\\____\\/::\\   \\:::\\   \\:::\\____\\
\\:::\\   \\:::\\   \\::/    /\\::/    /|::|  /:::/    /\\:::\\    \\      \\::/    / \\:::\\    \\   /:::/    / \\:::\\  /:::/    \\::/    /\\::/    /|::|  /:::/    /\\:::\\   \\:::\\   \\::/    /
 \\:::\\   \\:::\\   \\/____/  \\/____/ |::| /:::/    /  \\:::\\    \\      \\/____/   \\:::\\    \\ /:::/    /   \\:::\\/:::/    / \\/____/  \\/____/ |::| /:::/    /  \\:::\\   \\:::\\   \\/____/
  \\:::\\   \\:::\\    \\              |::|/:::/    /    \\:::\\    \\                \\:::\\    /:::/    /     \\::::::/    /                   |::|/:::/    /    \\:::\\   \\:::\\    \\
   \\:::\\   \\:::\\____\\             |::::::/    /      \\:::\\    \\                \\:::\\__/:::/    /       \\::::/____/                    |::::::/    /      \\:::\\   \\:::\\____\\
    \\:::\\   \\::/    /             |:::::/    /        \\:::\\    \\                \\::::::::/    /         \\:::\\    \\                    |:::::/    /        \\:::\\  /:::/    /
     \\:::\\   \\/____/              |::::/    /          \\:::\\    \\                \\::::::/    /           \\:::\\    \\                   |::::/    /          \\:::\\/:::/    /
      \\:::\\    \\                  /:::/    /            \\:::\\    \\                \\::::/    /             \\:::\\    \\                  /:::/    /            \\::::::/    /
       \\:::\\____\\                /:::/    /              \\:::\\____\\                \\::/____/               \\:::\\____\\                /:::/    /              \\::::/    /
        \\::/    /                \\::/    /                \\::/    /                 ~~                      \\::/    /                \\::/    /                \\::/    /
         \\/____/                  \\/____/                  \\/____/                                           \\/____/                  \\/____/                  \\/____/\
         \n\
         \n=================================================================================================================================================================================");
}

/// Shows the help menu
pub(crate) fn show_help()
{
    print!("{esc}c", esc = 27 as char);
    print_logo();
    println!(
        "\n\n\
        =================================================================================================================================================================================\n\n\
        Available commands : \n\
        \t• transfer <user> <amount>                            : Makes transaction request of <amount> coins to user <user> \n\
        \t• balance <according-to>                              : Displays all current balances according to account <according-to>\n\
        \t• ldwallet <path-to-wallet>                           : Loads a public key and secret key from a .wallet file\n\
        \t• genwallet <name>                                    : Generates and loads a new wallet and saves it to the given path\n\
        \t• clear                                               : Clears terminal from previous entered instructions \n\
        \t• help                                                : Displays the list of possible instructions \n\
        \t• quit                                                : Quits program\n\
        \n=================================================================================================================================================================================\n");
    println!("\nPress any key to exit:");
    print!("> ");
    io::stdout().flush().unwrap();
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");

}