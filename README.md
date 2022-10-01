# GooseBumpsBackend

## build

```sh
cargo build
```

## run

```sh
cargo run
```

## docker

```sh
docker-compose build
docker-compose up
```

## build solidity contract

```sh
npm install @openzeppelin/contracts
solc @openzeppelin/=$(pwd)/node_modules/@openzeppelin/ src/contracts/*.sol --abi --bin -o contracts
```

