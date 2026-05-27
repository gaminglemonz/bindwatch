use indicatif::ProgressBar;
use netstat2::*;
mod lookup;
mod parser;
mod sockets;

fn main() {
    let args = parser::parse();
    let mut seen_pids: Vec<u32> = Vec::new();

    match args.command {
        parser::Commands::List { tcp, udp, all } => {
            let spinner = ProgressBar::new_spinner();
            let network_processes = sockets::find_processes();

            for (index, process) in network_processes.iter().enumerate() {
                spinner.set_message(format!(
                    "Scanning socket entries {} ({}/{})",
                    index,
                    index,
                    network_processes.len()
                ));
                if let Some(p) = Some(process) {
                    if seen_pids.contains(&p.pid) {
                        continue;
                    }

                    match p.protocol {
                        "UDP" => {
                            if !tcp {
                                spinner.println(format!(
                                    "UDP socket on {}:{} with PID {:?} ({})",
                                    p.local.addr, p.local.port, p.pid, p.name
                                ));
                            } else {
                                if !all {
                                    continue;
                                }
                            }
                        }
                        "TCP" => {
                            if !udp {
                                spinner.println(format!(
                                    "TCP socket {} on {}:{} -> {}:{} with PID {:?} ({})",
                                    string_state(p.state.as_ref()),
                                    p.local.addr,
                                    p.local.port,
                                    p.remote.as_ref().unwrap().addr,
                                    p.remote.as_ref().unwrap().port,
                                    p.pid,
                                    p.name
                                ));
                            } else {
                                if !all {
                                    continue;
                                }
                            }
                        }
                        _ => println!("Process with unknown protocol found"),
                    }
                    seen_pids.push(p.pid)
                }
            }
            spinner.finish_with_message(format!(
                "\nDone! Found {} processes ({} unlisted duplicates) \n",
                network_processes.len(),
                network_processes.len() - seen_pids.len()
            ))
        }
        parser::Commands::Search {
            ref name,
            ref path,
            pid,
            port,
            local,
            remote,
        } => {
            let network_processes = sockets::find_processes();
            if port != None {
                if local {
                    let process =
                        lookup::find_process::by_local_port(&network_processes, port.unwrap());
                    if let Some(_process) = process {
                        println!(
                            "Process {} found {} on port {:?}",
                            _process.name,
                            string_state(_process.state.as_ref()),
                            _process.local.port
                        );
                    } else if process == None {
                        println!("No process was found with local port {}", port.unwrap())
                    }
                } else if remote {
                    let process =
                        lookup::find_process::by_remote_port(&network_processes, port.unwrap());
                    if let Some(_process) = process {
                        println!(
                            "Process {} found {} on port {:?}",
                            _process.name,
                            string_state(_process.state.as_ref()),
                            _process.local.port
                        );
                    } else if process == None {
                        println!(
                            "No process was found with on a remote socket endpoint with port {}",
                            port.unwrap()
                        )
                    }
                } else {
                    println!(
                        "Please provide whether the process is local or remote (use --local or --remote flags)"
                    );
                    println!(
                        "help: bindwatch search --port {} --local or bindwatch search --port {} --remote",
                        port.unwrap(),
                        port.unwrap()
                    )
                }
            } else if let Some(_name) = &name {
                let process = lookup::find_process::by_name(&network_processes, _name.to_string());
                if let Some(_process) = process {
                    println!(
                        "Process {} found {} on port {:?}",
                        _process.name,
                        string_state(_process.state.as_ref()),
                        _process.local.port
                    );
                } else if process == None {
                    println!("No process was found with name {}", name.as_ref().unwrap())
                }
            } else if let Some(_pid) = pid {
                let process = lookup::find_process::by_pid(&network_processes, pid.unwrap());
                if let Some(_process) = process {
                    println!(
                        "Process {} found {} on port {:?}",
                        _process.name,
                        string_state(_process.state.as_ref()),
                        _process.local.port
                    );
                } else if process == None {
                    println!("No process was found with PID {}", pid.unwrap())
                }
            } else if let Some(_path) = path {
                let process = lookup::find_process::by_path(
                    &network_processes,
                    path.as_ref().unwrap().to_string(),
                );
                if let Some(_process) = process {
                    println!(
                        "Process {} found {} on port {:?}",
                        _process.name,
                        string_state(_process.state.as_ref()),
                        _process.local.port
                    );
                } else if process == None {
                    println!("No process was found with path {}", path.as_ref().unwrap())
                }
            }
        }
    }
}

fn string_state(state: Option<&TcpState>) -> &'static str {
    match state {
        Some(&TcpState::Listen) => "listening",
        Some(&TcpState::Closed) => "closed",
        Some(&TcpState::Unknown) => "in an unknown state or connected to unknown host",
        Some(&TcpState::Closing) => "closing",
        Some(&TcpState::Established) => "connected",
        Some(&TcpState::CloseWait) => "awaiting disconnection",
        Some(&TcpState::FinWait1) => "closing connection (phase 1)",
        Some(&TcpState::FinWait2) => "closing connection (phase 2)",
        Some(&TcpState::SynSent) => "initiating connection",
        Some(&TcpState::SynReceived) => "receiving connection request",
        Some(&TcpState::DeleteTcb) => "deleting connection contorl block",
        Some(&TcpState::TimeWait) => "awaiting connection timeout",
        Some(&TcpState::LastAck) => "sending final acknowledgment",
        _ => "in an unknown state",
    }
}
