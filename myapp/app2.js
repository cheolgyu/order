var express = require('express');
var app = express();
const https = require('https');
const path = require('path');
const fs = require('fs');
const options = {
    key: fs.readFileSync('./privateKey.pem'),
    cert: fs.readFileSync('./caKey.pem')
};


app.use(express.static('public'));

app.get('/', function (req, res) {
    //res.send('Hello World!');
    res.sendFile(path.join(__dirname + '/index.html'));
});
https.createServer(options, app).listen(3005, function () {
    console.log("HTTPS server listening on port " + 3005);
});

