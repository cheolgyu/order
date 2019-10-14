var express = require('express');
var app = express();
const path = require('path');

app.use(express.static('public'));

app.get('/', function (req, res) {
    //res.send('Hello World!');
    res.sendFile(path.join(__dirname + '/index.html'));
});
app.get('/index.html', function (req, res) {
    //res.send('Hello World!');
    res.sendFile(path.join(__dirname + '/index.html'));
});

app.listen(3005, function () {
    console.log('Example app listening on port 3000!');
});
