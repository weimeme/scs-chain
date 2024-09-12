
## **1. local**

### install
[install environment](./install-environment.md)

### build

```
cargo build --release --features tscs
```

### run

```
./scs --chain staging --database auto
```

### **2. docker**
```
docker run -id --name tscs-node wjyask/tscs-node:latest ""
```