#!/bin/bash

rfkill unblock wifi
ip addr add 192.168.1.1/24 dev wlan0
ip link set wlan0 promisc on
## Para los drivers de ZTE
rmmod zt9101_ztopmac_usb
modprobe zt9101_ztopmac_usb
# ip link add name br0 type bridge
# ip link set br0 up

iptables-restore < /etc/iptables/rules.v4
ip6tables-restore < /etc/iptables/rules.v6

systemctl restart hostapd
systemctl restart dnsmasq
