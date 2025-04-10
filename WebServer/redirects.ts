import express, { Request, Response } from 'express';
import os from 'os';
import { exec } from 'child_process';

const router = express.Router();

const captiveTriggers = [
    "/",               // acceso directo
    "/generate_204",   // Android
    "/hotspot-detect.html", // Apple
    "/ncsi.txt",       // Windows
    "/redirect",       // algunas distros
    "/wpad.dat"
];

// Middleware para redirigir solo si la ruta es de prueba de conectividad
router.use((req, res, next) => {
    if (captiveTriggers.includes(req.path)) {
        let ip = req.headers['x-forwarded-for'] || req.socket.remoteAddress;

        // If the IP is in IPv6 format (::ffff:192.168.x.x), extract the IPv4 part
        if (typeof ip === 'string' && ip.startsWith('::ffff:')) {
            ip = ip.split('::ffff:')[1];
        }

        return res.redirect(`http://192.168.1.1:8080/login?ip=${ip}&mac=`);
    } else {
        next();
    }
});

// Fallback si no hay ninguna ruta
router.use((req, res) => {
    res.status(404).send("Ruta no encontrada");
});


export default router;