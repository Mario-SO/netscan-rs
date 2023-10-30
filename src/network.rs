use pnet::datalink::MacAddr;
use pnet::datalink::{DataLinkReceiver, DataLinkSender, NetworkInterface};
use pnet::packet::arp::{ArpHardwareTypes, ArpOperations, ArpPacket, MutableArpPacket};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket};
use pnet::packet::{MutablePacket, Packet};
use pnet::util::MacAddr as PnetMacAddr;
use std::net::Ipv4Addr;
use std::time::Duration;

pub fn send_arp_request(
    tx: &mut dyn DataLinkSender,
    rx: &mut dyn DataLinkReceiver,
    source_ip: Ipv4Addr,
    target_ip: Ipv4Addr,
    interface: &NetworkInterface,
) -> Option<MacAddr> {
    let source_mac = match interface.mac {
        Some(mac) => PnetMacAddr::new(mac.0, mac.1, mac.2, mac.3, mac.4, mac.5),
        None => return None,
    };

    let mut arp_buffer = [0u8; 42];
    let mut ethernet_packet = MutableEthernetPacket::new(&mut arp_buffer).unwrap();

    ethernet_packet.set_destination(MacAddr::broadcast());
    ethernet_packet.set_source(source_mac);
    ethernet_packet.set_ethertype(EtherTypes::Arp);

    let mut arp_packet = MutableArpPacket::new(ethernet_packet.payload_mut()).unwrap();
    arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
    arp_packet.set_protocol_type(EtherTypes::Ipv4);
    arp_packet.set_hw_addr_len(6);
    arp_packet.set_proto_addr_len(4);
    arp_packet.set_operation(ArpOperations::Request);
    arp_packet.set_sender_hw_addr(source_mac);
    arp_packet.set_sender_proto_addr(source_ip);
    arp_packet.set_target_hw_addr(MacAddr::zero());
    arp_packet.set_target_proto_addr(target_ip);

    tx.send_to(ethernet_packet.packet(), None);

    let timeout = Duration::from_secs(1);
    let start = std::time::Instant::now();

    while start.elapsed() < timeout {
        if let Ok(packet) = rx.next() {
            if let Some(ethernet_packet) = EthernetPacket::new(packet) {
                if ethernet_packet.get_ethertype() == EtherTypes::Arp {
                    let arp_packet = ArpPacket::new(ethernet_packet.payload()).unwrap();

                    if arp_packet.get_operation() == ArpOperations::Reply
                        && arp_packet.get_sender_proto_addr() == target_ip
                    {
                        return Some(MacAddr::new(
                            arp_packet.get_sender_hw_addr().0,
                            arp_packet.get_sender_hw_addr().1,
                            arp_packet.get_sender_hw_addr().2,
                            arp_packet.get_sender_hw_addr().3,
                            arp_packet.get_sender_hw_addr().4,
                            arp_packet.get_sender_hw_addr().5,
                        ));
                    }
                }
            }
        }
    }

    None
}
