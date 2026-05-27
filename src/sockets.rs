use crate::lookup;
use indicatif::ProgressBar;
use netstat2::*;
use std::{net::IpAddr, time::Duration};

#[derive(PartialEq)]
pub struct LocalInfo {
    pub port: u16,
    pub addr: IpAddr,
}

#[derive(Clone, PartialEq)]
pub struct RemoteInfo {
    pub port: u16,
    pub addr: IpAddr,
}

#[derive(PartialEq)]
pub struct PortInfo {
    pub name: String,
    pub local: LocalInfo,
    pub remote: Option<RemoteInfo>,
    pub protocol: &'static str,
    pub pid: u32,
    pub state: Option<TcpState>,
}

pub fn find_processes() -> Vec<PortInfo> {
    let sockets = get_sockets_info(
        AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6,
        ProtocolFlags::TCP | ProtocolFlags::UDP,
    )
    .unwrap();
    let mut network_processes: Vec<PortInfo> = Vec::new();
    let spinner = ProgressBar::new_spinner();

    spinner.set_message("Finding processes...");
    spinner.enable_steady_tick(Duration::from_millis(100));

    for (index, socket) in sockets.iter().enumerate() {
        spinner.set_message(format!(
            "Finding processes... ({}/{})",
            index,
            sockets.len()
        ));
        match &socket.protocol_socket_info {
            ProtocolSocketInfo::Tcp(si) => {
                for pid in &socket.associated_pids {
                    network_processes.push(PortInfo {
                        pid: *pid,
                        protocol: "TCP",
                        name: lookup::process_converter::pid_to_name(*pid),
                        state: Some(si.state),
                        local: LocalInfo {
                            port: si.local_port,
                            addr: si.local_addr,
                        },
                        remote: Some(RemoteInfo {
                            port: si.remote_port,
                            addr: si.remote_addr,
                        }),
                    });
                }
            }
            ProtocolSocketInfo::Udp(si) => {
                for pid in &socket.associated_pids {
                    network_processes.push(PortInfo {
                        pid: *pid,
                        protocol: "UDP",
                        name: lookup::process_converter::pid_to_name(*pid),
                        state: None,
                        local: LocalInfo {
                            port: si.local_port,
                            addr: si.local_addr,
                        },
                        remote: None,
                    });
                }
            }
        }
    }

    println!(
        "\nFinished! Found {} socket entries.",
        network_processes.len()
    );

    return network_processes;
}
