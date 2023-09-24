use clap::{Parser, Subcommand, ValueEnum};

use libskiller::{self, Color};

/// Skiller Pro+ cli
#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Subcommands,

    /// When setting a value, specifies the profile that value applies to
    #[arg(short, long, default_value_t = 1)]
    pub profile: u8,
}

#[derive(Subcommand)]
pub enum Subcommands {
    /// Set the color of the keyboard
    Color {
        /// The color to be set
        color: Color,
    },

    /// Set brightness and effects (and color because the way this gets set is weird)
    Brightness {
        #[command(subcommand)]
        brightness: libskiller::Brightness,
    },

    /// Change the profile to the one provided through the root level profile option
    Profile,

    /// Control if the windows key is enabled or disabled
    WinKey { enable: WinKeyState },

    /// Sets the GLOBAL (not per profile) polling rate of the keyboard
    PollingRate { rate: u32 },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum WinKeyState {
    On,
    Off,
}

impl WinKeyState {
    pub fn to_bool(&self) -> bool {
        match self {
            WinKeyState::On => true,
            WinKeyState::Off => false,
        }
    }
}
