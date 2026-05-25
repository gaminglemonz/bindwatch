# bindwatch
A command-line tool written in Rust for listing and searching for network processes and sockets on your machine


## Usage

`*` = Required flag

`bindwatch list` - List all network processes found on machine

&nbsp;&nbsp;&nbsp;&nbsp;`--tcp` - Only list sockets with the TCP protocol

&nbsp;&nbsp;&nbsp;&nbsp;`--udp` - Only list sockets with the UDP protocol

`bindwatch search` - Search for a network process

&nbsp;&nbsp;&nbsp;&nbsp;`-p, --port <PORT>` - Search for a network process by the port

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`-l, --local` - Search for a local port

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`-r, --remote` - Search for a remote socket endpoint

&nbsp;&nbsp;&nbsp;&nbsp;`--pid <PID>` - Search by an associated PID

&nbsp;&nbsp;&nbsp;&nbsp;`--name <NAME>` - Search by the process name

&nbsp;&nbsp;&nbsp;&nbsp;`--path <PATH>` - Search by path
