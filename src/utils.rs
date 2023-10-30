use pnet::datalink::{self, NetworkInterface};
use std::net::Ipv4Addr;

pub fn get_interface_by_name(interface_name: &str) -> Option<NetworkInterface> {
    datalink::interfaces()
        .into_iter()
        .find(|iface| iface.name == interface_name)
}

pub fn get_ipv4_addr(interface: &NetworkInterface) -> Option<Ipv4Addr> {
    interface
        .ips
        .iter()
        .filter_map(|ip| match ip.ip() {
            std::net::IpAddr::V4(addr) => Some(addr),
            std::net::IpAddr::V6(_) => None,
        })
        .next()
}
