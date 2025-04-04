import express, { Request, Response } from 'express';
import sqlite3 from 'sqlite3';
import bodyParser from 'body-parser';
import bcrypt from 'bcrypt';

const router = express.Router();
const db = new sqlite3.Database('./users.db');

async function hashPassword(password: string): Promise<string> {
    const saltRounds = 10; // Número de rondas (10 es seguro y rápido)
    const hash = await bcrypt.hash(password, saltRounds);
    return hash;
}

async function checkPassword(password: string, hash: string): Promise<boolean> {
    return await bcrypt.compare(password, hash);
}

// Create
router.post('/adduser', async (req: Request, res: Response) => {
    const { username, password } = req.body;
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
            return res.status(500).json({ error: err.message });
        }
        res.json({ users: row });
    });
});

// Verify
router.post('/login', async (req: Request, res: Response) => {
    const { username, password, ip, mac } = req.body;
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
    if (!mac || !ip) {
        res.status(400).send("Faltan parámetros (mac o ip)");
        return;
    }
    db.get("SELECT * FROM devices WHERE ip = ? AND mac = ?", [ip, mac], (err, row) => {
        if (err) {
            res.status(500).json({ error: err.message });
            return;
        }
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

export default router;