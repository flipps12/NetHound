"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const express_1 = __importDefault(require("express"));
const sqlite3_1 = __importDefault(require("sqlite3"));
const bcrypt_1 = __importDefault(require("bcrypt"));
const router = express_1.default.Router();
const db = new sqlite3_1.default.Database(__dirname + '/users.db');
function hashPassword(password) {
    return __awaiter(this, void 0, void 0, function* () {
        const saltRounds = 10; // Número de rondas (10 es seguro y rápido)
        const hash = yield bcrypt_1.default.hash(password, saltRounds);
        return hash;
    });
}
function checkPassword(password, hash) {
    return __awaiter(this, void 0, void 0, function* () {
        return yield bcrypt_1.default.compare(password, hash);
    });
}
// Create
router.post('/adduser', (req, res) => __awaiter(void 0, void 0, void 0, function* () {
    const { username, password } = req.body;
    db.run("INSERT INTO devices (username, password) VALUES (?, ?)", [username, yield hashPassword(password)], function (err) {
        if (err) {
            return res.status(500).json({ error: err.message });
        }
        res.json({ id: this.lastID });
    });
}));
// Read all
router.get('/getallusers', (req, res) => {
    db.all("SELECT * FROM devices", [], (err, rows) => {
        if (err) {
            console.error(err);
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
            console.error(err);
            return res.status(500).json({ error: err.message });
        }
        res.json({ users: row });
    });
});
// Verify
router.post('/login', (req, res) => __awaiter(void 0, void 0, void 0, function* () {
    const { username, password, ip, mac } = req.body;
    console.log(username, password, ip, mac);
    if (!username || !password || !ip) { // || !mac
        res.status(400).send("Faltan parámetros (username, password o ip)");
        return;
    }
    // Check if the user exists
    db.get("SELECT password FROM devices WHERE username = ?", [username], (err, row) => __awaiter(void 0, void 0, void 0, function* () {
        if (err) {
            return res.status(500).json({ error: err.message });
        }
        try {
            if (row && (yield checkPassword(password, row.password))) {
                db.run("UPDATE devices SET ip = ?, mac = ? WHERE username = ?", [ip, mac, username], function (err) {
                    if (err) {
                        return res.status(500).json({ error: err.message });
                    }
                    res.json({ verified: true }); // return token
                });
            }
            else {
                res.json({ verified: false });
            }
        }
        catch (e) {
            res.json({ verified: false });
        }
    }));
}));
// Delete
router.delete('/delete/:id', (req, res) => {
    const { id } = req.params;
    db.run("DELETE FROM devices WHERE id = ?", [id], function (err) {
        if (err) {
            return res.status(500).json({ error: err.message });
        }
        res.json({ changes: this.changes });
    });
});
// Verify IP and MAC
router.get('/verify', (req, res) => __awaiter(void 0, void 0, void 0, function* () {
    const { mac, ip } = req.query;
    console.log(mac, ip);
    // if (!mac || !ip) {
    //     res.status(400).send("Faltan parámetros (mac o ip)");
    //     return;
    // }
    db.get("SELECT * FROM devices WHERE ip = ?", [ip], (err, row) => {
        if (err) {
            console.error(err);
            res.status(500).json({ error: err.message });
            return;
        }
        console.log(!!row);
        res.json({ authorized: !!row });
    });
}));
// Verify only IP
router.get('/onlyip', (req, res) => __awaiter(void 0, void 0, void 0, function* () {
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
}));
exports.default = router;
