use sysinfo::{ProcessorExt, RefreshKind, System, SystemExt};

mod shell;
mod terminal;
mod uptime;
mod wm;

fn main() {
    let sys = System::new_with_specifics(RefreshKind::new().with_cpu().with_memory());
    let distro = sys.name().expect("Couldn't find distro");
    let hostname = sys.host_name().expect("Couldn't get hostname");
    let kernel = sys.kernel_version().expect("Couldn't get kernel version");
    let uptime = uptime::duration(sys.uptime());
    let shell = shell::get_shell().expect("Couldn't get shell");
    let wm = wm::get_wm().expect("Couldn't get WM");
    let terminal = terminal::get_terminal().expect("Couldn't get terminal");
    println!("OS: {}", distro);
    println!("Hostname: {}", hostname);
    println!("Kernel: {}", kernel);
    println!("Uptime: {:?}", uptime);
    println!("Shell: {}", shell);
    println!("WM: {}", wm);
    println!("Terminal: {}", terminal);
}
