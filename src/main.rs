extern crate console;
extern crate indicatif;
extern crate ipnetwork;
extern crate pnet;

use console::{style, Emoji, Style, Term};
use indicatif::{ProgressBar, ProgressStyle};
use ipnetwork::Ipv4Network;
use pnet::datalink::{self, NetworkInterface};
use std::env;
use std::net::Ipv4Addr;

mod network;

fn main() {
    let term = Term::stdout();
    term.clear_screen().unwrap();

    // Header
    println!("{}", style("scan_rs by mariodev").bold().underlined());
    println!();

    let bold = Style::new().bold();
    let green = Style::new().green();
    let red = Style::new().red().on_black();
    let blue = Style::new().blue();
    let yellow = Style::new().yellow();
    let check_mark = Emoji("✅ ", "");
    let cross_mark = Emoji("❌ ", "");
    let info_mark = Emoji("💡 ", "");

    // Get the network interface
    let interfaces = datalink::interfaces();
    let interface_name = env::args().nth(1).unwrap();
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == interface_name);

    let interface = match interface {
        Some(iface) => iface,
        None => {
            eprintln!(
                "{}{}Network interface '{}' not found. Please check the interface name and try again.",
                bold.apply_to("Error: "),
                red.apply_to(cross_mark),
                interface_name
            );
            return;
        }
    };

    // Extract the IPv4 address
    let ipv4_addr = get_ipv4_addr(&interface).expect("No IPv4 address found for the interface.");

    // Define your network (change this to your local network range)
    let ip_network = Ipv4Network::new(ipv4_addr, 24).unwrap();

    println!(
        "{}Scanning on interface: {}",
        bold.apply_to(blue.apply_to(info_mark)),
        interface.name
    );
    println!();

    let (mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!(
            "{}An error occurred when creating the datalink channel: {}",
            bold.apply_to("Error: "),
            e
        ),
    };

    let total_ips: u64 = ip_network.size() as u64;
    let pb = ProgressBar::new(total_ips);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] {wide_bar:.green/red} {pos}/{len} ({eta})",
        )
        .unwrap()
        .progress_chars("#>-"),
    );

    let mut scanned_ips = 0;
    for ip in ip_network.iter() {
        scanned_ips += 1;
        pb.set_position(scanned_ips);

        match network::send_arp_request(&mut *tx, &mut *rx, ipv4_addr, ip, &interface) {
            Some(mac) => {
                // Temporarily disable the progress bar before printing the result
                pb.println(format!(
                    "{}Found device with IP: {}, MAC: {}",
                    green.apply_to(check_mark),
                    yellow.apply_to(ip),
                    blue.apply_to(mac)
                ));
            }
            None => {}
        }
    }
    pb.finish_with_message("🎉 Scan complete! 🎉");
}

fn get_ipv4_addr(interface: &NetworkInterface) -> Option<Ipv4Addr> {
    interface
        .ips
        .iter()
        .filter_map(|ip| match ip.ip() {
            std::net::IpAddr::V4(addr) => Some(addr),
            std::net::IpAddr::V6(_) => None,
        })
        .next()
}
