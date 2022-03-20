use clap::Parser;

#[derive(Parser)]
#[clap(version, about)]
struct Arguments {
    /// Print available MIDI input ports
    #[clap(short, long)]
    list: bool,
}

fn main() {
    let _args = Arguments::parse();
}
