import http from 'http';
import express from 'express';
import path from 'path';
import serveStatic from 'serve-static';

const app = express();
app.use(serveStatic(path.resolve(process.cwd(), 'app'), { index: ['index.html'] }));
const server = http.createServer(app);
server.listen(8081, () => console.log('express app listening on 8081...'));