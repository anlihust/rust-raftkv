# Rust Raft KV

A very simple example to use [Raft](https://github.com/pingcap/raft-rs) in Rust.

## Build and Start

```bash
make

# You can use goreman or other similar tools like foreman to manage the cluster
# go get github.com/mattn/goreman
goreman start 
```

## Usage

```bash
# Get status of a server, we can know the leader from status
curl http://127.0.0.1:8081/status

# Send the request to leader

# Put abc = 124
curl http://127.0.0.1:8083/kv/abc -d 123
# Get abc 
curl http://127.0.0.1:8083/kv/abc
# Delete abc
curl http://127.0.0.1:8083/kv/abc -x DELETE

# Get abc locally, not through Raft 
curl http://127.0.0.1:8083/local_kv/abc
```