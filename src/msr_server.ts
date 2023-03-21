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

type msr_obj = {
    cpu: {
        vendor: string,
        name: string,
        power: number,
        voltage: number,
        usage: number,
        temperature: number,
        hyper_threading: number,
        logical_cores: number,
        physical_cores: number,
    }
}

app.get('/json/*', (req, res) => {
    exec('sudo msr_gen -j', (error, stdout, stderr) => {
        let obj: msr_obj = JSON.parse(stdout)
        if (stdout !== null) {
            if (req.url === "/json" || req.url === "/json/")
                res.write(stdout);
            if (req.url.includes('/usage')) res.write(obj.cpu.usage.toString()+" ");
            if (req.url.includes('/power')) res.write(obj.cpu.power.toString()+" ");
            if (req.url.includes('/temperature')) res.write(obj.cpu.temperature.toString()+" ");
            if (req.url.includes('/voltage')) res.write(obj.cpu.voltage.toString()+" ");
            return res.send()
        }
        else if (stderr !== null)
            res.send(stderr);
        else
            return res.send(error);
    });
});

const execute_middleware =(req: { body: string; }): string => {
    let json = JSON.parse(JSON.stringify(req.body))
    if (json.key !== null && process.env.MSR_KEY !== null) {
        if (json.command !== null && process.env.MSR_KEY === json.key)
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