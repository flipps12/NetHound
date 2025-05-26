
pub fn _get_interfaces() -> Vec<String> {
    let mut interfaces = Vec::new();
    for iface in pnet::datalink::interfaces() {
        interfaces.push(iface.name);
    }
    interfaces
}
