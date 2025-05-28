import express, { Request, Response } from 'express';
import sqlite3 from 'sqlite3';
import bodyParser from 'body-parser';
import bcrypt from 'bcrypt';
import axios from 'axios';
const router = express.Router();

// sudo mkdir /var/lib/nethound
const db = new sqlite3.Database('/var/lib/nethound/nethound.db');


async function hashPassword(password: string): Promise<string> {
    const saltRounds = 10; // Número de rondas (10 es seguro y rápido)
    const hash = await bcrypt.hash(password, saltRounds);
    return hash;
}

async function checkPassword(password: string, hash: string): Promise<boolean> {
    return await bcrypt.compare(password, hash);
}

router.get("/reload_firewall", async (req: Request, res: Response) => {
    try {
        await reload_firewall();
        res.json({ message: "Firewall reloaded successfully" });
    } catch (error) {
        console.error("Error reloading firewall:", error);
        res.status(500).json({ error: "Failed to reload firewall" });
    }
});

// Create
router.post('/adduser', async (req: Request, res: Response) => {
    const { username, password } = req.body;
    if (!username || !password) {
        res.status(400).json({ error: "Username and password are required" });
    }
    db.run("INSERT INTO devices (username, password) VALUES (?, ?)", [username, await hashPassword(password)], function (err) {
        if (err) {
            return res.status(500).json({ error: err.message });
        }
        res.json({ id: this.lastID });
    });
});

// Read all
router.get('/getallusers', (req: Request, res: Response) => {
    db.all("SELECT * FROM devices", [], (err, rows) => {
        if (err) {
            console.error(err);
            return res.status(500).json({ error: err.message });
        }
        res.json({ users: rows });
    });
});


// Read one
router.get('/user/:user', (req: Request, res: Response) => {
    const { username } = req.params;
    db.get("SELECT * FROM devices WHERE username = ?", [username], (err, row) => {
        if (err) {
            console.error(err);
            return res.status(500).json({ error: err.message });
        }
        res.json({ users: row });
    });
});

// Read one
router.get('/userip', async (req: Request<{}, {}, {}, { mac?: string; ip?: string }>, res: Response): Promise<void> => {
    const { mac, ip } = req.query;
    db.get("SELECT * FROM devices WHERE ip = ?", [ip], (err, row) => {
        if (err) {
            console.error(err);
            return res.status(500).json({ error: err.message });
        }
        if (!row) {
            return res.json({ authorized: !!row });
        }
        db.run("UPDATE devices SET mac = ? WHERE ip = ?", [mac, ip], function (err) {
            if (err) {
                return res.status(500).json({ error: err.message });
            }
            res.json({ authorized: !!row });
        });
    });
    reload_firewall();
});



// Verify
router.post('/login', async (req: Request, res: Response) => {
    const { username, password, ip, mac } = req.body;
    console.log(username, password, ip, mac);
    if (!username || !password || !ip) { // || !mac
        res.status(400).send("Faltan parámetros (username, password o ip)");
        return;
    }
    // Check if the user exists
    db.get("SELECT password FROM devices WHERE username = ?", [username], async (err, row: { password: string } | undefined) => {
        if (err) {
            return res.status(500).json({ error: err.message });
        }
        try {
            if (row && await checkPassword(password, row.password)) {
                db.run("UPDATE devices SET ip = ?, mac = ? WHERE username = ?", [ip, mac, username], function (err) {
                    if (err) {
                        return res.status(500).json({ error: err.message });
                    }
                    res.json({ verified: true }); // return token
                });
            } else {
                res.json({ verified: false });
            }
        } catch (e) {
            res.json({ verified: false });
        }
    });
    reload_firewall();
});

// Delete
router.delete('/delete/:id', (req: Request, res: Response) => { // añadir seguridad por token
    const { id } = req.params;
    db.run("DELETE FROM devices WHERE id = ?", [id], function (err) {
        if (err) {
            return res.status(500).json({ error: err.message });
        }
        res.json({ changes: this.changes });
    });
});

// Verify IP and MAC
router.get('/verify', async (req: Request<{}, {}, {}, { mac?: string; ip?: string }>, res: Response): Promise<void> => {
    const { mac, ip } = req.query;
    console.log(mac, ip);
    // if (!mac || !ip) {
    //     res.status(400).send("Faltan parámetros (mac o ip)");
    //     return;
    // }
    db.get("SELECT * FROM devices WHERE ip = ?", [ip], (err, row) => {// AND mac = ?
        if (err) {
            console.error(err);
            res.status(500).json({ error: err.message });
            return;
        }
        console.log(!!row);
        res.json({ authorized: !!row });
    });
});

// Verify only IP
router.get('/onlyip', async (req: Request<{}, {}, {}, { ip?: string }>, res: Response): Promise<void> => {
    const { ip } = req.query;
    if (!ip) {
        res.status(400).send("Faltan parámetros (ip)");
        return;
    }
    db.get("SELECT * FROM devices WHERE ip = ?", [ip], (err, row) => {
        if (err) {
            res.status(500).json({ error: err.message });
            return;
        }
        res.json({ authorized: !!row });
    });
});


async function reload_firewall() {
    return new Promise<void>((resolve, reject) => {
        db.all("SELECT ip FROM devices WHERE ip IS NOT NULL", [], async (err, rows: { ip: string }[]) => {
            if (err) {
                return reject(err);
            }
            const ips = rows.map((row) => row.ip);
            try {
                await axios.post('http://127.0.0.1:3030/reload', { ips });
                resolve();
            } catch (error) {
                if (error && typeof error === "object" && "message" in error) {
                    console.error("Failed to reload firewall (axios):", (error as { message: string }).message);
                } else {
                    console.error("Failed to reload firewall (axios):", error);
                }
                // Do not reject, just resolve to prevent crashing the process
                resolve();
            }
        });
    });
}

export default router;