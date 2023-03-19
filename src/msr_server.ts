//import express from "express";
import { exec } from 'child_process';
const express = require("express")

const app = express();

app.get('/', async (req:any, res: { send: (arg0: any) => void; }) => {
    await exec('sudo msr_gen -o', (error, stdout, stderr) => {
        if( stdout !== null) return res.send(stdout);
        else if ( stderr !== null) res.send(stderr)
        else return res.send(error);
    });
});

app.listen(3230, () => console.log('listening on port 3230'));