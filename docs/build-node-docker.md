## first

```
docker buildx create --use
```
## build tscs-node

```
docker buildx build --platform linux/amd64,linux/arm64 -t wjyask/tscs-node:1.0.0 -t wjyask/tscs-node:latest --push -f tscs-node.Dockerfile .
```

## build scs-node
```
docker buildx build --platform linux/amd64,linux/arm64 -t wjyask/scs-node:1.0.0 -t wjyask/scs-node:latest  --push -f scs-node.Dockerfile .

```