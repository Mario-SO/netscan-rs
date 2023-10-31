extern crate console;
extern crate indicatif;
extern crate ipnetwork;
extern crate pnet;

use ipnetwork::Ipv4Network;

mod network;
mod ui;
mod utils;

fn main() {
    ui::init();

    let interface_name = ui::select_interface();
    let interface = match utils::get_interface_by_name(&interface_name) {
        Some(iface) => iface,
        None => {
            ui::print_error(&format!(
                "Network interface '{}' not found. Please check the interface name and try again.",
                interface_name
            ));
            return;
        }
    };

    let ipv4_addr =
        utils::get_ipv4_addr(&interface).expect("No IPv4 address found for the interface.");
    let ip_network = Ipv4Network::new(ipv4_addr, 24).unwrap();
    ui::print_scanning_message(&interface.name);

    let (mut tx, mut rx) =
        network::create_datalink_channel(&interface).expect("Failed to create datalink channel.");

    let total_ips: u64 = ip_network.size() as u64;
    let pb = ui::create_progress_bar(total_ips);

    let mut scanned_ips = 0;
    for ip in ip_network.iter() {
        scanned_ips += 1;
        pb.set_position(scanned_ips);

        match network::send_arp_request(&mut *tx, &mut *rx, ipv4_addr, ip, &interface) {
            Some(mac) => {
                ui::print_device_found(&pb, ip, &mac.to_string());
            }
            None => {}
        }
    }

    // pb.finish_with_message("🎉 Scan complete! 🎉");
}
