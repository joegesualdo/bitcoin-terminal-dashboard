☠️⚠️ Work In Progress ⚠️☠️

# Bitcoin Terminal Dashboard
> Bitcoin Dashboard in the Terminal

![Screenshot](screenshot_3.png)

This library provides helpful functions to query common information about the bitcoin network.

## Install
> Add package to Cargo.toml file
```bash
$ cargo install bitcoin-terminal-dashboard
```

## Setup:
> Must have these environment variable set for the terminal to work. Could go in your `.zshrc` or `.bashrc`:
```
export BITCOIND_PASSWORD="..." 
export BITCOIND_USERNAME="..." 
export BITCOIND_URL="127.0.0.1:8332"
```
## Usage
``` bash
 $ bitcoin-terminal-dashboard
```

> Could optionally pass the environment variable to the script:
```
 BITCOIND_PASSWORD=... BITCOIND_USERNAME=...BITCOIND_URL=... bitcoin-terminal-dashboard
```

## Related
- [bitcoind-request](https://github.com/joegesualdo/bitcoind-request) - Type-safe wrapper around bitcoind RPC commands
- [bitcoin-node-query](https://github.com/joegesualdo/bitcoin-node-query) - Query information about the bitcoin network

## License
MIT © [Joe Gesualdo]()

