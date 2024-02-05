use cherrymx::commands::{get_command, Run, Successful};
use cherrymx::mxlp21_keyboard::find_mxlp21_keyboard;
use std::{env::args, process::ExitCode};

fn main() -> ExitCode {
    let device = find_mxlp21_keyboard()
        .expect("No MX-LP 2.1 Compact Wireless Mechanical Keyboard keyboard found, sorry!");
    let args = args().skip(1).collect::<Vec<_>>();

    let command = get_command(&args);
    let cmd_status = command.run(&device);

    if cmd_status.successful() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(cmd_status as u8)
    }
}
