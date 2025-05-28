import { useEffect, useState } from "react";


async function getHostname(ip: string): Promise<string | null> {
    const res = await fetch(`http://${window.location.hostname}/api/resolve?ip=${ip}`); // el fronted esta en el puerto 8080 y el backend en el 80
    const data = await res.json();
    return data.success ? data.hostnames[0] : null;
}

interface HostnameProps {
    ip: string;
}

const Hostname = ({ ip }: HostnameProps) => {
    const [hostname, setHostname] = useState<string | null>(null);

    useEffect(() => {
        let isMounted = true;
        getHostname(ip).then(resolved => {
            if (isMounted) setHostname(resolved);
        });
        return () => { isMounted = false; };
    }, [ip]);

    if (hostname === null) return null;//<span> (resolving...)</span>;
    if (hostname === undefined) return null;
    return <span> ({hostname})</span>;
};

export default Hostname;