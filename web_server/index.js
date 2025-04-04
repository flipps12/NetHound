const express = require('express');
const apiRouter = require('./api');
const bodyParser = require('body-parser');

const app = express();
const port = 8080;

app.use(bodyParser.json());
app.use(bodyParser.urlencoded({ extended: true }));

app.get('/', (req, res) => {
    res.send('¡Hola, mundo!');
});

// Captura de detección de Captive Portal
app.get("/generate_204", (req, res) => {
    res.redirect("http://captive.portal.local/");
});

app.get("/hotspot-detect.html", (req, res) => {
    res.redirect("http://captive.portal.local/");
});

app.get("/connecttest.txt", (req, res) => {
    res.redirect("http://captive.portal.local/");
});

app.use('/api', apiRouter);

app.listen(port, () => {
    console.log(`Servidor web escuchando en http://localhost:${port}`);
});