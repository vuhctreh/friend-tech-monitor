//! CLI I/O operations.

use text_io::read;

// Read from CLI.
pub fn get_code_from_cli() -> String {
    log::info!("SMS sent to Phone number.");

    println!("Enter the code sent to your phone number: ");

    let code: String = read!("{}");

    log::info!("Got code: {}", &code);

    code
}