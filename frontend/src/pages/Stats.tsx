import { useEffect, useState } from "react";
import axios from "axios";

interface TrafficItem {
    bytes: number;
    dst_ip: string;
    dst_mac: string;
    id: number;
    count: number;
    protocol: string;
    src_ip: string;
    src_mac: string;
}

function formatBytes(bytes: number): string {
    if (bytes === 0) return "0 Bytes";
    const k = 1024;
    const sizes = ["Bytes", "KB", "MB", "GB", "TB", "PB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
}

function isPrivateIP(ip: string): boolean {
    // IPv4 only
    const parts = ip.split('.').map(Number);
    if (parts.length !== 4 || parts.some(n => isNaN(n) || n < 0 || n > 255)) return false;
    if (parts[0] === 10) return true;
    if (parts[0] === 172 && parts[1] >= 16 && parts[1] <= 31) return true;
    if (parts[0] === 192 && parts[1] === 168) return true;
    if (parts[0] === 127) return true; // loopback
    return false;
}

const Stats = () => {
    const [stats, setStats] = useState<TrafficItem[]>([]);

    useEffect(() => {
        axios.get(`http://${window.location.hostname}/api/daily-traffic/2025-05-28`)
            .then(response => {
                // Ensure the data is always an array
                setStats(Array.isArray(response.data) ? response.data : []);
                console.log("Stats fetched:", response.data);
            })
            .catch(error => {
                console.error("Error fetching stats:", error);
            });
    }, []);

    return (
        <div>
            <h2>Device Stats</h2>
            {/* Agrupar por src_ip y sumar bytes y count */}
            {Object.entries(
                stats.reduce<Record<string, { upload: number; download: number; internal: number; count: number }>>((acc, curr) => {
                    const isSrcPrivate = isPrivateIP(curr.src_ip);
                    const isDstPrivate = isPrivateIP(curr.dst_ip);

                    if (isSrcPrivate) {
                        if (!acc[curr.src_ip]) {
                            acc[curr.src_ip] = { upload: 0, download: 0, internal: 0, count: 0 };
                        }
                        if (!isDstPrivate) {
                            acc[curr.src_ip].upload += curr.bytes;
                            acc[curr.src_ip].count += curr.count;
                        } else {
                            acc[curr.src_ip].internal += curr.bytes;
                            acc[curr.src_ip].count += curr.count;
                        }
                    }

                    if (!isSrcPrivate && isDstPrivate) {
                        if (!acc[curr.dst_ip]) {
                            acc[curr.dst_ip] = { upload: 0, download: 0, internal: 0, count: 0 };
                        }
                        acc[curr.dst_ip].download += curr.bytes;
                        acc[curr.dst_ip].count += curr.count;
                    }

                    return acc;
                }, {})
            ).map(([ip, data]) => (
                <div key={ip}>
                    <strong>IP:</strong> {ip} <br />
                    <strong>Upload:</strong> {formatBytes(data.upload) + ' - ' + data.upload} <br />
                    <strong>Download:</strong> {formatBytes(data.download) + ' - ' + data.download} <br />
                    <strong>Internal:</strong> {formatBytes(data.internal) + ' - ' + data.internal} <br />
                    <strong>Paquetes (count):</strong> {data.count} <br />
                    <hr />
                </div>
            ))}

            <ul>
                {stats.map((device, idx) => (
                    <li key={device.id ?? idx}><br />
                        <strong>bytes:</strong> {device.bytes} <br />
                        <strong>Source IP:</strong> {device.src_ip} <br />
                        <strong>Destination IP:</strong> {device.dst_ip} <br />
                        <strong>Paquetes (count):</strong> {device.count} <br />
                    </li>
                ))}
            </ul>
        </div>
    );
};

export default Stats;
