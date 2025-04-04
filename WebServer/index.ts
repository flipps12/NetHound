import express, { Request, Response } from 'express';
import apiRouter from './api';
import bodyParser from 'body-parser';

const app = express();
const port = 8080;

app.use(bodyParser.json());
app.use(bodyParser.urlencoded({ extended: true }));

app.get('/', (req: Request, res: Response) => {
    res.send('¡Hola, mundo!');
});

// Captura de detección de Captive Portal
app.get("/generate_204", (req: Request, res: Response) => {
    res.redirect("http://captive.portal.local/");
});

app.get("/hotspot-detect.html", (req: Request, res: Response) => {
    res.redirect("http://captive.portal.local/");
});

app.get("/connecttest.txt", (req: Request, res: Response) => {
    res.redirect("http://captive.portal.local/");
});

app.use('/api', apiRouter);

app.listen(port, () => {
    console.log(`Servidor web escuchando en http://localhost:${port}`);
});

export default app;