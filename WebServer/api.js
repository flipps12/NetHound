const express = require('express');
const sqlite3 = require('sqlite3').verbose();
var bodyParser = require('body-parser');
const bcrypt = require("bcrypt");

const router = express.Router();

const db = new sqlite3.Database('./users.db');

async function hashPassword(password) {
    const saltRounds = 10; // Número de rondas (10 es seguro y rápido)
    const hash = await bcrypt.hash(password, saltRounds);
    return hash;
}

async function checkPassword(password, hash) {
    return await bcrypt.compare(password, hash);
}

// Create
router.post('/adduser', async (req, res) => {
    const { username, password } = req.body;
    db.run("INSERT INTO devices (username, password) VALUES (?, ?)", [username, await hashPassword(password)], function(err) {
        if (err) {
            return res.status(500).json({ error: err.message });
        }
        res.json({ id: this.lastID });
    });
});

// Read all
router.get('/getallusers', (req, res) => {
    db.all("SELECT * FROM devices", [], (err, rows) => {
        if (err) {
            return res.status(500).json({ error: err.message });
        }
        res.json({ users: rows });
    });
});

// Read one
router.get('/user/:user', (req, res) => {
    const { username } = req.params;
    db.get("SELECT * FROM devices WHERE username = ?", [username], (err, row) => {
        if (err) {
            return res.status(500).json({ error: err.message });
        }
        res.json({ users: row });
    });
});

// Verify
router.post('/login', async (req, res) => {
    const { username, password, ip, mac } = req.body;
    db.get("SELECT password FROM devices WHERE username = ?", [username], async (err, row) => {
        if (err) {
            return res.status(500).json({ error: err.message });
        }
        try {
            if (await checkPassword(password, row.password)) {
                db.run("UPDATE devices SET ip = ?, mac = ? WHERE username = ?", [ip, mac, username], function(err) {
                    if (err) {
                        return res.status(500).json({ error: err.message });
                    }
                    res.json({ verified: true }); // return token
                });
            } else {
                res.json({ verified: false });
            }
        }
        catch (e) {
            res.json({ verified: false });
        }
    });
});

// Delete
router.delete('/delete/:id', (req, res) => { // añadir seguridad por token
    const { id } = req.params;
    db.run("DELETE FROM devices WHERE id = ?", [id], function(err) {
        if (err) {
            return res.status(500).json({ error: err.message });
        }
        res.json({ changes: this.changes });
    });
});

// verify ip and mac
router.get('/verify', (req, res) => {
    const { mac, ip } = req.query;
    if (!mac || !ip) {
        return res.status(400).send("Faltan parámetros (mac o ip)");
    }
    console.log(ip, " - ", mac);
    db.get("SELECT * FROM devices WHERE ip = ? AND mac = ?", [ip, mac], (err, row) => {
        if (err) {
            return res.status(500).json({ error: err.message });
        }
        if (row) {
            res.json({ authorized: true });
        } else {
            res.json({ authorized: false });
        }
    });
});
router.get('/onlyip', (req, res) => {
    const { ip } = req.query;
    if (!ip) {
        return res.status(400).send("Faltan parámetros (mac o ip)");
    }
    console.log(ip, " - ", mac);
    db.get("SELECT * FROM devices WHERE ip = ?", [ip, mac], (err, row) => {
        if (err) {
            return res.status(500).json({ error: err.message });
        }
        if (row) {
            res.json({ authorized: true });
        } else {
            res.json({ authorized: false });
        }
    });
});

module.exports = router;