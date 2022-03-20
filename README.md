# Simple MIDI Logger
This app prints MIDI messages from the selected MIDI input port to stdout. Each message is divided by '\n' (line break) and the bytes in a message is divided by ',' (comma).

## Usage
```
simple-midi-logger 0.2.0
Printing MIDI messages from the selected MIDI input port to stdout.

USAGE:
    simple-midi-logger [OPTIONS]

OPTIONS:
    -h, --help
            Print help information

    -i, --input <INPUT>
            Specify the MIDI input port by name or index (You can check it by executing this with
            `--list-input` option)

    -l, --list-input
            Print available MIDI input ports

        --list-output
            Print available MIDI output ports

    -o, --output <OUTPUT>
            Specify the MIDI output port (to through the received messages) by name or index (You
            can check it by executing this with `--list-output` option)

    -V, --version
            Print version information

        --virtual-output <VIRTUAL_OUTPUT>
            Create a virtual MIDI output port with this name (for *nix only)
```

### For Windows
This app will be run on Windows 8+ because `winrt` feature in midir, which is the one of the dependencies, is enabled.

### For JACK
JACK can be used on \*nix OS instead of the default backend by enabling `jack` feature in this app and rebuilding.
