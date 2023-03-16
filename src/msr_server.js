const express = require("express")
const fs = require("fs")

const app = express();

app.get('/', (req, res) => {
    fs.readFile('/msr_data.toml', 'utf-8', (err, data) => {
        if (err) return res.send("err")
        return res.send(data);
    })
});

app.listen(3230, () => console.log('listening on port 3230'));