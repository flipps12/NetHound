import express, { Request, Response } from 'express';
import apiUsersRouter from './routes/users';
import apiPacketAnalizerRouter from './routes/PacketAnalizer';
import redirectRouter from './redirects';
import bodyParser from 'body-parser';
import sqlite3 from 'sqlite3';
import cors from 'cors';

const app = express();
const port = 80;
const corsOptions = {
    origin: '*', // Replace '*' with specific origins for better security
    methods: ['GET', 'POST', 'PUT', 'DELETE', 'OPTIONS'],
    allowedHeaders: ['Content-Type', 'Authorization']
};

app.use(cors(corsOptions));
app.use(bodyParser.json());
app.use(bodyParser.urlencoded({ extended: true }));

app.use('/api', apiUsersRouter);
app.use('/api', apiPacketAnalizerRouter);
app.use(redirectRouter);

app.listen(port, () => {
    console.log(`Servidor web escuchando en http://localhost:${port}`);
});

export default app;