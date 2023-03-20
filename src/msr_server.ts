import express from "express";
import { ExecException, exec } from 'child_process';
//const express = require("express")

const app = express();
const port = 3230;
app.use(express.json())

app.get('/', (req:any, res: { send: (arg0: any) => void; }) => {
    exec('sudo msr_gen -o', (error, stdout, stderr) => {
        if (stdout !== null)
            return res.send(stdout);
        else if (stderr !== null)
            res.send(stderr);
        else
            return res.send(error);
    });
});

app.get('/json', (req:any, res: { send: (arg0: any) => void; }) => {
    exec('sudo msr_gen -j', (error, stdout, stderr) => {
        if (stdout !== null)
            return res.send(stdout);
        else if (stderr !== null)
            res.send(stderr);
        else
            return res.send(error);
    });
});

const execute_middleware =(req: { body: string; }): string => {
    let json = JSON.parse(JSON.stringify(req.body))
    return "currently not implemented"
    if (json.key !== null && json.key === "lorem") {
        if (json.command !== null)
            return json.command
        else {
            return "echo \"no key supplied in attached json\""
        }
    } else {
        return "echo \"no key supplied in attached json\""
    }

}

app.post('/execute', (req , res: { send: (arg0: any) => void; } ) => {
    exec(execute_middleware(req), (error, stdout, stderr) => {
        if (stdout !== null)
            return res.send(stdout);
        else if (stderr !== null)
            res.send(stderr);
        else
            return res.send(error);
    })
})

app.listen(port, () => console.log(`listening on port ${port}`));