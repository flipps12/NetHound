use pnet::packet::{
    Packet,
    ethernet::{EthernetPacket, EtherTypes},
    ipv4::Ipv4Packet,
    ipv6::Ipv6Packet,
    ip::IpNextHeaderProtocols,
    tcp::TcpPacket,
    udp::UdpPacket,
};

#[derive(Debug, Clone)]
pub struct PacketData {
    pub _raw_data: Vec<u8>,
    // Datos de Ethernet
    pub src_mac: Option<String>,
    pub dst_mac: Option<String>,
    pub ethertype: Option<u16>,
    // Datos de IP (IPv4 o IPv6)
    pub src_ip: Option<String>,
    pub dst_ip: Option<String>,
    pub ip_protocol: Option<u8>,
    // Datos de TCP
    pub tcp_src_port: Option<u16>,
    pub tcp_dst_port: Option<u16>,
    pub tcp_sequence: Option<u32>,
    pub tcp_ack: Option<u32>,
    pub tcp_flags: Option<u16>,
    // Datos de UDP
    pub udp_src_port: Option<u16>,
    pub udp_dst_port: Option<u16>,
    pub udp_length: Option<u16>,
}

pub fn parse_packet(packet: &[u8]) -> PacketData {
    let mut data = PacketData {
        _raw_data: packet.to_vec(),
        src_mac: None,
        dst_mac: None,
        ethertype: None,
        src_ip: None,
        dst_ip: None,
        ip_protocol: None,
        tcp_src_port: None,
        tcp_dst_port: None,
        tcp_sequence: None,
        tcp_ack: None,
        tcp_flags: None,
        udp_src_port: None,
        udp_dst_port: None,
        udp_length: None,
    };

    if let Some(eth_packet) = EthernetPacket::new(packet) {
        data.src_mac = Some(format!("{}", eth_packet.get_source()));
        data.dst_mac = Some(format!("{}", eth_packet.get_destination()));
        data.ethertype = Some(eth_packet.get_ethertype().0);

        match eth_packet.get_ethertype() {
            EtherTypes::Ipv4 => {
                if let Some(ip_packet) = Ipv4Packet::new(eth_packet.payload()) {
                    data.src_ip = Some(format!("{}", ip_packet.get_source()));
                    data.dst_ip = Some(format!("{}", ip_packet.get_destination()));
                    data.ip_protocol = Some(ip_packet.get_next_level_protocol().0);

                    if ip_packet.get_next_level_protocol() == IpNextHeaderProtocols::Tcp {
                        if let Some(tcp_packet) = TcpPacket::new(ip_packet.payload()) {
                            data.tcp_src_port = Some(tcp_packet.get_source());
                            data.tcp_dst_port = Some(tcp_packet.get_destination());
                            data.tcp_sequence = Some(tcp_packet.get_sequence());
                            data.tcp_ack = Some(tcp_packet.get_acknowledgement());
                            data.tcp_flags = Some(tcp_packet.get_flags().into());
                        }
                    }
                    if ip_packet.get_next_level_protocol() == IpNextHeaderProtocols::Udp {
                        if let Some(udp_packet) = UdpPacket::new(ip_packet.payload()) {
                            data.udp_src_port = Some(udp_packet.get_source());
                            data.udp_dst_port = Some(udp_packet.get_destination());
                            data.udp_length = Some(udp_packet.get_length());
                        }
                    }
                }
            },
            EtherTypes::Ipv6 => {
                if let Some(ipv6_packet) = Ipv6Packet::new(eth_packet.payload()) {
                    data.src_ip = Some(format!("{}", ipv6_packet.get_source()));
                    data.dst_ip = Some(format!("{}", ipv6_packet.get_destination()));
                    data.ip_protocol = Some(ipv6_packet.get_next_header().0);

                    if ipv6_packet.get_next_header() == IpNextHeaderProtocols::Tcp {
                        if let Some(tcp_packet) = TcpPacket::new(ipv6_packet.payload()) {
                            data.tcp_src_port = Some(tcp_packet.get_source());
                            data.tcp_dst_port = Some(tcp_packet.get_destination());
                            data.tcp_sequence = Some(tcp_packet.get_sequence());
                            data.tcp_ack = Some(tcp_packet.get_acknowledgement());
                            data.tcp_flags = Some(tcp_packet.get_flags().into());
                        }
                    }
                    if ipv6_packet.get_next_header() == IpNextHeaderProtocols::Udp {
                        if let Some(udp_packet) = UdpPacket::new(ipv6_packet.payload()) {
                            data.udp_src_port = Some(udp_packet.get_source());
                            data.udp_dst_port = Some(udp_packet.get_destination());
                            data.udp_length = Some(udp_packet.get_length());
                        }
                    }
                }
            },
            _ => {}
        }
    }
    data
}
