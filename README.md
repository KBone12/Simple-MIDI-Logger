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

### Example
```
/path/to/simple-midi-logger>$ ./simple-midi-logger -l
0: Midi Through:Midi Through Port-0 14:0
1: Oxygen Pro 61:Oxygen Pro 61 USB MIDI 24:0
2: Oxygen Pro 61:Oxygen Pro 61 MIDI DIN 24:1
3: Oxygen Pro 61:Oxygen Pro 61 Mackie/HUI 24:2
4: Oxygen Pro 61:Oxygen Pro 61 Editor 24:3
/path/to/simple-midi-logger>$ ./simple-midi-logger -i "Oxygen Pro 61:Oxygen Pro 61 USB MIDI 24:0"
0x90,0x43,0x67
0x90,0x3C,0x60
0x90,0x40,0x64
0x80,0x40,0x00
0x80,0x43,0x00
0x80,0x3C,0x00
^C
/path/to/simple-midi-logger>$
```

### For Windows
This app will be run on Windows 8+ because `winrt` feature in midir, which is the one of the dependencies, is enabled.

### For JACK
JACK can be used on \*nix OS instead of the default backend by enabling `jack` feature in this app and rebuilding.
