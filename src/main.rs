use clap::Parser;
use midir::{MidiIO, MidiInput, MidiOutput};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Can't initialize MIDI")]
    MidiInitError(#[from] midir::InitError),

    #[error("Can't connect to the MIDI port")]
    MidiConnectError(#[from] midir::ConnectError<MidiInput>),
}

type Result<T> = std::result::Result<T, AppError>;

#[derive(Parser)]
#[clap(version, about)]
struct Arguments {
    /// Print available MIDI input ports
    #[clap(short, long = "list-input")]
    list_input: bool,

    /// Print available MIDI output ports
    #[clap(long = "list-output")]
    list_output: bool,

    /// Specify the MIDI input port by name or index (You can check it by executing this with `--list-input`
    /// option)
    ///
    /// If there is a name which is just a number, then the number means a port index (so you
    /// can't specifity the port by name).
    #[clap(short, long)]
    input: Option<String>,
}

fn print_midi_ports<Midi: MidiIO>(midi: &Midi) -> Result<()> {
    println!(
        "{}",
        midi.ports()
            .iter()
            .filter_map(|port| midi.port_name(port).ok())
            .enumerate()
            .map(|(index, port_name)| format!("{index}: {port_name}"))
            .collect::<Vec<_>>()
            .join("\n")
    );

    Ok(())
}

fn main() -> Result<()> {
    let midi_input = MidiInput::new("Simple MIDI Logger")?;
    let midi_output = MidiOutput::new("Simple MIDI Logger")?;

    let args = Arguments::parse();

    // Print infomations and exit
    if args.list_input {
        print_midi_ports(&midi_input)?;
        return Ok(());
    } else if args.list_output {
        print_midi_ports(&midi_output)?;
        return Ok(());
    }

    let midi_input_ports = midi_input.ports();
    let midi_input_port = if let Some(input) = args.input {
        if let Ok(port_index) = input.parse::<usize>() {
            midi_input_ports.get(port_index)
        } else {
            midi_input_ports
                .iter()
                .find(|port| midi_input.port_name(port).as_ref() == Ok(&input))
        }
        .expect(format!("No such MIDI input port ({})", input).as_str())
    } else {
        midi_input_ports
            .get(0)
            .expect("There is no MIDI input port")
    };
    let _midi_input_connection = midi_input.connect(
        midi_input_port,
        "Simple MIDI Logger",
        |_timestamp, midi_message, _data| {
            println!(
                "{}",
                midi_message
                    .iter()
                    .map(|byte| format!("0x{:02X}", byte))
                    .collect::<Vec<_>>()
                    .join(",")
            );
        },
        (),
    )?;

    loop {}
}
