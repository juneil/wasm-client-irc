const WebSocket = require('ws');
const net = require('net');
const ws = new WebSocket.Server({ port: 8081 });
 
ws.on('connection', function connection(s) {

    const client = new net.Socket();

    client.connect(6667, 'irc.europnet.org', function() {
        console.log('Connected');
    });
    
    client.on('data', function(data) {
        s.send(data.toString());
        console.log('Received: ' + data);
    });
    
    client.on('close', function() {
        s.close();
        console.log('Connection closed');
    });

    s.on('message', function incoming(message) {
        client.send(message);
        console.log('WS received: %s', message);
    });

    s.on('close', function() {
        client.destroy();
        console.log('WS closed');
    })
});