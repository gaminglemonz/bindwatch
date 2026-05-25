# bindwatch
A command line tool written in Rust to list and search for network processes on your PC


## Usage
`bindwatch list` - List all network processes found on machine

&nbsp;&nbsp;&nbsp;&nbsp;`--tcp` - Only list sockets with TCP protocol

&nbsp;&nbsp;&nbsp;&nbsp;`--udp` - Only list sockets with UDP protocol

`bindwatch search` - Search for a network process

&nbsp;&nbsp;&nbsp;&nbsp;`-p, --port` - Search for a network process by the port

&nbsp;&nbsp;&nbsp;&nbsp;`--pid` - Search for a network process by an associated PID

&nbsp;&nbsp;&nbsp;&nbsp;`--name` - Search for a network process by the process name
