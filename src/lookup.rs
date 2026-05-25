pub mod process_converter {
    use std::path::Path;
    use sysinfo::System;

    pub fn pid_to_name(target_pid: u32) -> String {
        let mut sys = System::new_all();
        sys.refresh_all();
        for (pid, process) in sys.processes() {
            if pid.as_u32() == target_pid {
                return process.name().to_string_lossy().to_string();
            }
        }
        "Unknown".to_string()
    }
    pub fn path_to_pid(target_path: String) -> Option<u32> {
        let mut sys = System::new_all();
        sys.refresh_all();
        for (pid, process) in sys.processes() {
            if let Some(path) = process.exe() {
                if path == Path::new(target_path.as_str()) {
                    Some(pid.as_u32());
                }
            }
        }
        None
    }
}

pub mod find_process {
    use crate::sockets;
    pub fn by_local_port(
        processes: &Vec<sockets::PortInfo>,
        port: u16,
    ) -> Option<&sockets::PortInfo> {
        processes.iter().find(|p| p.local.port == port)
    }
    pub fn by_remote_port(
        processes: &Vec<sockets::PortInfo>,
        port: u16,
    ) -> Option<&sockets::PortInfo> {
        processes
            .iter()
            .find(|p| p.remote.as_ref().unwrap().port == port)
    }
    pub fn by_name(
        processes: &Vec<sockets::PortInfo>,
        name: String,
    ) -> Option<&sockets::PortInfo> {
        let target: String = if name.as_str().ends_with(".exe") {
            name.to_string()
        } else {
            name.to_string() + ".exe"
        };
        processes.iter().find(|p| p.name == target)
    }
    pub fn by_pid(processes: &Vec<sockets::PortInfo>, pid: u32) -> Option<&sockets::PortInfo> {
        processes.iter().find(|p| p.pid == pid)
    }
    pub fn by_path(
        processes: &Vec<sockets::PortInfo>,
        path: String,
    ) -> Option<&sockets::PortInfo> {
        let pid = super::process_converter::path_to_pid(path);
        match pid {
            Some(_p) => by_pid(processes, pid.unwrap()),
            None => panic!("No process with that path!"),
        }
    }
}
