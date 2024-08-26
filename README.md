***[SuperEx](https://www.superex.com/)'s [Super Chain](https://scschain.com/)***

---
welcome to [SuperEx](https://www.superex.com/)

## local

### build
```
cargo build --release
```
### run 
```
./scs-node --dev --database auto --base-path db --pruning archive   --rpc-methods=Unsafe --rpc-cors=all --unsafe-rpc-external
```

## Docker

### build (if you are developer)
<!-- ```
docker buildx build create --name mybuilder --use
```
```
docker login
``` -->
```

docker buildx build --platform --load linux/amd64 -t tscs-node:latest -f normal-node.Dockerfile .
```

```
docker login
```

```
docker tag  tscs-node:latest <yourname>/tscs-node:latest

```

```
docker push <yourname>/tscs-node

```

<!-- ```
docker build -t tscs-node:latest -f normal-node.Dockerfile .
``` -->

### pull
```
docker pull tscs-node
```

### run dev network

```
docker run -id --name scs-dev --platform linux/amd64 -p 9944:9944 tscs-node:latest /usr/local/bin/scs --dev --database auto --rpc-methods=Unsafe --rpc-cors=all --unsafe-rpc-external --base-path /data
```

### run test network
todo