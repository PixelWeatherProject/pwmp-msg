# PWMP - Messages library
This is a core crate that contains the types which represent all the different message types in the PixelWeather Messaging Protocol *(PWMP)*.
Serialization and deserialization is also handled by this library.

# Message structure
A "message" is a simple `enum` that can be one of two variants:
- `Request`
    - Server requested a resource from the client (node) or vice-versa.
- `Response`
    - Contains the resource requested by the other party.

Requests and responses contain deeper level variants.

```mermaid
graph TD;
    Message-->Request
    Message-->Response

    Request-.->Ping
    Request-.->Hello
    Request-.->PostResults
    Request-.->PostStats
    Request-.->Bye
    Request-.->GetSetting
    Request-.->GetSettings

    Response-.->Pong
    Response-.->Ok
    Response-.->Reject
    Response-.->Setting
    Response-.->Settings
```

### Introduction message (`Hello`)
The `Hello` message is the first message sent by the client (node) to the server. It contains the MAC address of the client. The server will respond with an `Ok` message if the client is authorized to communicate with the server.

Message structure:
```mermaid
graph LR;
    Hello-->MAC
```

### Settings request meeting (`GetSettings`)
The `GetSettings` message is sent by the client (node) to the server to request the settings for the node. The server will respond with a `Settings` message.

Message structure:
```mermaid
graph LR;
    GetSettings-->SN[Setting names...]
```

### Settings response message (`Settings`)
The `Settings` message is sent by the server to the client (node) as a response to a `GetSettings` message. It contains the settings for the node.

Message structure:
```mermaid
graph LR;
    Settings-->SV[Setting values...]
```

The setting values are in the same order as requested in the `GetSettings` message.

### Results posting message (`PostResults`)
The `PostResults` message is sent by the client (node) to the server to post measurement results of the node. The server will respond with an `Ok` message if the results were successfully received.

Message structure:
```mermaid
graph LR;
    PostResults-->Temperature
    PostResults-->Humidity
    PostResults-->AP[Air Pressure]
```

### Statistics posting message (`PostStats`)
The `PostStats` message is sent by the client (node) to the server to post statistics of the node. The server will respond with an `Ok` message if the statistics were successfully received.

Message structure:
```mermaid
graph LR;
    PostStats-->V[Battery Voltage]
    PostStats-->SSID[WiFi ESSID]
    PostStats-->RSSI[WiFi RSSI]
```

# Example communication sequence
```mermaid
sequenceDiagram
    Node->>Server: Hello (incl. MAC address)
    Server->>Node: Ok
    Node->>Server: GetSettings [...]
    Server->>Node: Settings [...]
    Node->>Server: PostResults [temperature, humidity, ...]
    Server->>Node: Ok
    Node->>Server: PostStats (node statistics)
    Server->>Node: Ok
    Node->>Server: Bye
```

If the node's MAC address is not in the database, it's not authorized to communicate with the server.
```mermaid
sequenceDiagram
    Node->>Server: Hello (incl. MAC address)
    Server->>Node: Reject
```

It's also possible to configure the server to abruptly close the socket if the device is unauthorized, instead of sending a `Reject` response.

# Message rules
The node shall only send **one** `PostResults` message, duplicates will be rejected and the socket will be abruptly closed. The communication between nodes and the server should be exactly as specified in the diagram above. No more messages should be exchanged.

When the client (node) is done communicating with the server, it shall **always**:
1. Send a `Bye` request to the server.
2. **Wait** until the server closes the connection.

The [client library](../pwmp-client/) will guarantee the last two two requirements, but not the first one.

# Usage of `Box<T>` types
Message variants use `Box<>`-ed types for optimizing the size of messages. Boxed types do not have a capacity property, making them up to 8 bytes smaller than their non-boxed counterparts.

# Limitations
The PWMP protocol is designed with the following restrictions:
- Maximum number of nodes in a network **cannot be more** than $2^{16}$.
    - This is because node IDs are represented as 16-bit unsigned integers.
    - A network of more than `65536` nodes would require major optimizations. Using smaller integer types also helps reducing firmware sizes.
- Messages do not have any kind of error-correcting codes.
    - Since PWMP uses TCP, there is already some error-correction happening on a lower level.
    - If a message gets corrupted during transfer, it's very likely that the deserialization would fail anyway.