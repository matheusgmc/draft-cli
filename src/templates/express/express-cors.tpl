import express from 'express';
import cors from 'cors';

const app = express();

app.use(cors());

app.get('/ping', (_,res) => {
  return res.status(200).json("pong");
});

const port = process.env.PORT || 8080;

app.listen(port,() => console.log(`server is running at ${port}`));
