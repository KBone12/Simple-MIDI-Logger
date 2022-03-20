use clap::Parser;
use midir::MidiInput;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    MidiInitError(#[from] midir::InitError),
}

type Result<T> = std::result::Result<T, AppError>;

#[derive(Parser)]
#[clap(version, about)]
struct Arguments {
    /// Print available MIDI input ports
    #[clap(short, long)]
    list: bool,
}

fn print_midi_ports() -> Result<()> {
    let midi_input = MidiInput::new("Simple MIDI Logger")?;
    println!(
        "{}",
        midi_input
            .ports()
            .iter()
            .filter_map(|port| midi_input.port_name(port).ok())
            .enumerate()
            .map(|(index, port_name)| format!("{index}: {port_name}"))
            .collect::<Vec<_>>()
            .join("\n")
    );

    Ok(())
}

fn main() -> Result<()> {
    let args = Arguments::parse();
    if args.list {
        print_midi_ports()?;
    }

    Ok(())
}
