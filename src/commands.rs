use crate::mxlp21_keyboard::{set_color, show_info};
use rusb::{Device, GlobalContext};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[repr(u8)]
#[derive(PartialEq)]
pub enum Status {
    Success = 0,
    Failure,
    SuccessNoSave,
}

pub trait Successful {
    fn successful(&self) -> bool;
}

impl Successful for Status {
    fn successful(&self) -> bool {
        Status::Success == *self || Status::SuccessNoSave == *self
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Color(Vec<String>),
    Info,
    Help(Vec<String>),
    Unknown(Vec<String>),
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Color(args) => write!(f, "colour {}", args.join(" ")),
            Command::Info => write!(f, "info"),
            Command::Help(args) => write!(f, "help {}", args.join(" ")),
            Command::Unknown(args) => write!(f, "unknown {}", args.join(" ")),
        }
    }
}

pub fn get_command(args: &[String]) -> Command {
    let cmd = if args.is_empty() { "" } else { &args[0] };

    match cmd.to_lowercase().as_str() {
        "colour" | "c" => Command::Color(args[1..].to_vec()),
        "info" | "i" => Command::Info,
        "help" | "h" | "?" => Command::Help(args[1..].to_vec()),
        _ => Command::Unknown(args.to_vec()),
    }
}
pub trait Run {
    fn run(&self, device: &Device<GlobalContext>) -> Status;
    fn has_args(&self) -> bool;
}

impl Run for Command {
    fn run(&self, device: &Device<GlobalContext>) -> Status {
        match self {
            Command::Color(args) => color_command(device, args),
            Command::Info => info_command(device),
            Command::Help(args) => help_command(args),
            Command::Unknown(args) => {
                eprintln!("Uknown command: {}", args.join(" "));
                Status::SuccessNoSave
            }
        }
    }

    fn has_args(&self) -> bool {
        match self {
            Command::Color(args) => !args.is_empty(),
            Command::Help(args) => !args.is_empty(),
            Command::Unknown(args) => !args.is_empty(),
            _ => false,
        }
    }
}

fn info_command(device: &Device<GlobalContext>) -> Status {
    println!("Device bus:   {}", device.bus_number());
    println!("Device #:     {}", device.address());
    println!("Device speed: {:?}", device.speed());

    show_info(device);
    Status::SuccessNoSave
}

fn help_command(_args: &[String]) -> Status {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    println!("cherrymx - version {}\n", VERSION);
    println!("You do have a MX-LP 2.1 Compact Wireless Mechanical Keyboard ✅\n");
    println!("Please see -- tbd");

    Status::SuccessNoSave
}

fn color_command(device: &Device<GlobalContext>, args: &[String]) -> Status {
    let mut status = Status::Failure;
    if !args.is_empty() {
        let color: u32 = u32::from_str_radix(&args[0], 16).unwrap_or_else(|_| {
            eprintln!("Invalid color format. Please provide a hex color code without the '#'.");
            0 // Defaulting to 0 or any other default value you see fit
        });
        set_color(device, color);
        status = Status::Success;
    } else {
        eprintln!("At least one - 'color' - argument needed for 'color' command");

    }
    status
}
