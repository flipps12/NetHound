import express, { Request, Response } from 'express';
import apiRouter from './api';
import redirectRouter from './redirects';
import bodyParser from 'body-parser';

const app = express();
const port = 80;

app.use(bodyParser.json());
app.use(bodyParser.urlencoded({ extended: true }));

app.use('/api', apiRouter);
app.use(redirectRouter);

app.listen(port, () => {
    console.log(`Servidor web escuchando en http://localhost:${port}`);
});

export default app;
