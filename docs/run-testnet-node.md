
## **1. local**

### install
[install environment](./install-environment.md)

### build

```
cargo build --release --features tscs
```

### run

```
./scs --chain staging --database auto --pruning archive
```

### **2. docker**
```
docker run -id --name tscs-node --platform linux/amd64 tscs-node:latest /usr/local/bin/scs --chain staging --database auto
```