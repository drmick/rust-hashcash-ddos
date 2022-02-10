### Rust hashcash POW anti-ddos

To prevent a DDOS attack, we use the Hashcash algorithm for confirming the work of the client. The client will not be able to make a DDoS attack, since for each request it needs to calculate the hash, which takes time.

1. Client generates a message
2. Client appends a sequence to the message and hashes it using SHA256 until the first 20 bits of the hash are zero. This takes about a second. Because it is necessary to sort out several million hashes
3. When the hash is solved, the client sends a message to the server.
4. The server calculates a hash from the message. If the hash has the first 20 bits set to zero, then the message is considered valid.
5. The server writes the hash from the message to the store so that the client cannot use it a second time
6. The server returns a response to the client

#### Launch server

```
docker-compose -f docker-compose.server.yml build
docker-compose -f docker-compose.server.yml up
```


#### Launch client
```
docker-compose -f docker-compose.client.yml build
docker-compose -f docker-compose.client.yml up
```


1. [server](server) Server side
2. [client](client) Client side
3. [lib](lib) Common lib
