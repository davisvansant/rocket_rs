#### Rocket_rs

 - https://rocket.rs/

```
 docker build -t rocket_rs_hello_world -f Dockerfile.hello_world .
 docker run -d --rm -p 8000:8000 rocket_rs_hello_world
```

```
curl localhost:8000  
Hello world!
```
