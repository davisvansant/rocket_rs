#### Rocket_rs

 - https://rocket.rs/

###### Hello World

```
 docker build -t rocket_rs_hello_world -f Dockerfile.hello_world .
 docker run -d --rm -p 8000:8000 rocket_rs_hello_world
```

```
curl localhost:8000  
Hello world!
```

###### Simple JSON

```
docker build -t rocket_rs_simple_json -f Dockerfile.simple_json .
docker run -d --rm -p 8000:8000 rocket_rs_simple_json

```

```
curl -i localhost:8000
HTTP/1.1 200 OK
Content-Type: application/json
Server: Rocket
Content-Length: 40
Date: Sat, 21 Mar 2020 21:46:28 GMT

{"awesomesauce":"stuff","hello":"world"}
```
