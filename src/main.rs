use std::{process::exit, time::Duration};

use clap::Parser;
use cli::Cli;
use libskiller::{Brightness, SkillerProPlus};

mod cli;

fn main() {
    let args = Cli::parse();

    if (args.profile == 0) || (args.profile > 3) {
        eprintln!("Profile can only be 1, 2 or 3");
        exit(1);
    }
    let p = num_to_profile_unchecked(args.profile);

    let skiller = match SkillerProPlus::new(Duration::from_secs(1)) {
        Err(e) => {
            print_rusb_error(&e);
            exit(1);
        }
        Ok(skiller) => {
            match skiller {
                Some(s) => s,
                None => {
                    eprintln!("No Skiller Pro+ device found. Make sure the keyboard is connected to your pc.");
                    exit(1);
                }
            }
        }
    };

    let command_result = match args.command {
        cli::Subcommands::Color { color } => {
            println!("Setting Profile {:?} to {:?}.", p, color);
            skiller.set_color(color, p)
        }
        cli::Subcommands::Brightness { brightness } => {
            if let libskiller::Brightness::Static { level, color: _ } = &brightness {
                if level > &10 {
                    eprintln!("Level must be between 0 and 10.");
                    exit(1);
                }
            }

            pretty_print_brightness(&brightness);
            skiller.set_brightness(brightness, p)
        }
        cli::Subcommands::WinKey { enable } => {
            println!(
                "Setting windows key to {}.",
                format!("{:?}", enable).to_lowercase()
            );

            skiller.set_win_key(enable.to_bool(), p)
        }
        cli::Subcommands::Profile => {
            println!("Setting profile to {:?}", p);
            skiller.set_profile(p)
        }
        cli::Subcommands::PollingRate { rate } => {
            if let Some(rate) = try_to_polling(rate) {
                println!("Setting polling rate to {:?}", rate);
                skiller.set_polling_rate(rate)
            } else {
                eprintln!("Polling rate must be 125, 250, 500 or 1000.");
                exit(1);
            }
        }
    };

    match command_result {
        Err(err) => {
            print_rusb_error(&err);
            exit(1);
        }
        Ok(bytes_written) => {
            if bytes_written == 0 {
                eprintln!("Error: a command was sent but no bytes were written!");
                exit(1);
            }
        }
    }
}

fn print_rusb_error(err: &rusb::Error) {
    match err {
        rusb::Error::Access => {
            eprintln!("Got libusb Access error. Consider running as root or setting a udev rule.");
        }
        _ => {
            eprintln!("Libusb returned an error: {:?}.", err);
        }
    }
}

fn num_to_profile_unchecked(num: u8) -> libskiller::Profile {
    match num {
        1 => libskiller::Profile::P1,
        2 => libskiller::Profile::P2,
        3 => libskiller::Profile::P3,
        _ => unreachable!(),
    }
}

fn pretty_print_brightness(brightness: &Brightness) {
    match brightness {
        Brightness::Static { level, color } => {
            println!("Setting static brightness to level {}, {:?}.", level, color)
        }
        Brightness::Pulsating { color } => {
            println!("Setting pulsating brightness with color {:?}.", color)
        }
        Brightness::Cycle => println!("Setting brightness cycle."),
    }
}

fn try_to_polling(num: u32) -> Option<libskiller::PollingRate> {
    match num {
        125 => Some(libskiller::PollingRate::HZ125),
        250 => Some(libskiller::PollingRate::HZ250),
        500 => Some(libskiller::PollingRate::HZ500),
        1000 => Some(libskiller::PollingRate::HZ1000),
        _ => None,
    }
}
