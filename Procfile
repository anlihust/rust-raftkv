raft1: env RUST_LOG=error,raftkv=debug,raft=info ./bin/raftkv --cluster 1=127.0.0.1:20171:8081,2=127.0.0.1:20172:8082,3=127.0.0.1:20173:8083 -I 1 --data ./tmp/data1
raft2: env RUST_LOG=error,raftkv=debug,raft=info ./bin/raftkv --cluster 1=127.0.0.1:20171:8081,2=127.0.0.1:20172:8082,3=127.0.0.1:20173:8083 -I 2 --data ./tmp/data2
raft3: env RUST_LOG=error,raftkv=debug,raft=info ./bin/raftkv --cluster 1=127.0.0.1:20171:8081,2=127.0.0.1:20172:8082,3=127.0.0.1:20173:8083 -I 3 --data ./tmp/data3
