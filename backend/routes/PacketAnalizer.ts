import express, { Request, Response } from 'express';
import Database from 'better-sqlite3';
import dns from 'dns/promises';
import path from 'path';
const router = express.Router();

// Read all
router.get('/daily-traffic/:date', (req: Request, res: Response) => {
    const { date } = req.params;
    const dbPath = path.join('/var/lib/nethound', `traffic_${date}.db`);
    const db = new Database(dbPath, { readonly: true });
    const stmt = db.prepare(`
        SELECT *
        FROM packet_summary
        ORDER BY last_seen DESC
        LIMIT 100
    `);

    const rows = stmt.all();

    res.send(rows);

    db.close();
});

router.get(
    '/resolve',
    async (
        req: Request<{}, {}, {}, { mac?: string; ip?: string }>,
        res: Response
    ): Promise<void> => {
        const { ip } = req.query;

        if (!ip) {
            res.status(400).json({ success: false, error: 'Missing IP parameter' });
            return;
        }

        try {
            const hostnames = await dns.reverse(ip);
            res.json({ success: true, hostnames });
        } catch (err) {
            res.json({
                success: false,
                error: err instanceof Error ? err.message : String(err),
            });
        }
    }
);


export default router;