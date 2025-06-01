import express, { Request, Response, Router } from 'express';
import os from 'os';
import { exec } from 'child_process';
import { checkIp } from './routes/users';
const router: Router = express.Router();

const captiveTriggers = [
    "/",               // acceso directo
    "/generate_204",   // Android
    "/hotspot-detect.html", // Apple
    "/ncsi.txt",       // Windows
    "/redirect",       // algunas distros
    "/wpad.dat"
];

// // Middleware para redirigir solo si la ruta es de prueba de conectividad
// router.use((req, res, next) => {
//     if (captiveTriggers.includes(req.path)) {
//         let ip = req.headers['x-forwarded-for'] || req.socket.remoteAddress;

//         // If the IP is in IPv6 format (::ffff:192.168.x.x), extract the IPv4 part
//         if (typeof ip === 'string' && ip.startsWith('::ffff:')) {
//             ip = ip.split('::ffff:')[1];
//         }

//         return res.redirect(`http://192.168.1.1:8080/login?ip=${ip}&mac=`);
//     } else {
//         next();
//     }
// });
function extractClientIp(req: Request): string | null {
    let ip = req.headers['x-forwarded-for'] || req.socket.remoteAddress;
    
    if (Array.isArray(ip)) ip = ip[0];

    if (typeof ip === 'string') {
        if (ip.startsWith('::ffff:')) ip = ip.split('::ffff:')[1];
        if (/^\d{1,3}(\.\d{1,3}){3}$/.test(ip)) return ip;
    }

    return null;
}

const p = async () => {
    console.log("Testing checkIp function...");
    console.log(await checkIp('192.168.1.15'))
}
p();
router.get('/generate_204', async (req: Request<{}, {}, {}, { mac?: string; ip?: string }>, res: Response): Promise<void> => {
    const ip = extractClientIp(req);
    console.log("IP from generate_204:", ip);
    if (!ip) {
        res.status(400).send("IP address not found");
        return;
    }

    try {
        const isAuthorized = await checkIp(ip);
        console.log(`Checking IP: ${ip}, Authorized: ${isAuthorized}`);
        if (isAuthorized) {
            res.status(204).send(); // Android espera esto
        } else {
            res.redirect(`http://192.168.1.1:8080/login?ip=${ip}&mac=`);
        }
    } catch (err) {
        console.error("Error in checkIp:", err);
        res.status(500).send("Internal Server Error");
    }
});

router.get('/', async (req: Request<{}, {}, {}, { mac?: string; ip?: string }>, res: Response): Promise<void> => {
    const ip = extractClientIp(req);

    if (!ip) {
        res.status(400).send("IP address not found");
        return;
    }

    try {
        const isAuthorized = await checkIp(ip);
        console.log(`Checking IP: ${ip}, Authorized: ${isAuthorized}`);
        if (isAuthorized) {
            res.status(204).send(); // Android espera esto
            return;
        } else {
            res.redirect(`http://192.168.1.1:8080/login?ip=${ip}&mac=`);
            return;
        }
    } catch (err) {
        console.error("Error in checkIp:", err);
        res.status(500).send("Internal Server Error");
    }
});


// Fallback si no hay ninguna ruta
router.use((req, res) => {
    res.status(404).send("Ruta no encontrada");
});


export default router;