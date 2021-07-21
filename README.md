<h1 align="center"><br>
    <a href="https://perun.network/"><img src=".assets/go_perun.png" alt="Perun" width="30%"></a>
<br></h1>

<h4 align="center">Perun Polkadot Node</h4>

<p align="center">
  <a href="https://www.apache.org/licenses/LICENSE-2.0.txt"><img src="https://img.shields.io/badge/license-Apache%202-blue" alt="License: Apache 2.0"></a>
   <a href="https://github.com/perun-network/perun-polkadot-node/actions/workflows/rust.yml"><img src="https://github.com/perun-network/perun-polkadot-node/actions/workflows/rust.yml/badge.svg"></a>
</p>

*Perun Polkadot Nodes* demonstrates how to integrate the [Perun Pallet] into  substrate chain.  
It uses the Node and Frontend templates from substrate and configures them for Perun.
## Repo Structure

* `frontend/` contains the [frontend template]
  * [src/config/types.json] holds some type definition for the frontend
* `node/` contains the [node template]
  * [pallets/perun] contains the [Perun Pallet]
  * [runtime/Cargo.toml] configures the Node
  * [runtime/src/lib.rs] configures the *Perun Pallet*

## Usage

You can either use docker or compile it manually.  
In both cases you need to clone the repo with:  
```bash
git clone --recurse-submodules https://github.com/perun-network/perun-polkadot-node
cd perun-polkadot-node
```

### Docker

You can use docker to start the back- and front end to try out the *Perun Pallet*.

```bash
docker-compose build # This will take some time
docker-compose up
```
The frontend will be online at [172.17.0.2:8000/substrate-front-end-template](http://172.18.0.2:8000/substrate-front-end-template).  
The IP can be different if you have other docker containers running. Look into the console to find it.

<p align="center">
<a href=".assets/pallet_perun.png"><img src=".assets/pallet_perun.png" width="100%"></a>
</p>

You can try out different Extrinsic calls in the *Pallet Interactor*.  
This is currently not very practicable since hashes and off-chain signatures are required, which are hard to enter manually. Using mocked hashes and sigs could be done here.

### Manual compilation

If you want to compile the source directly, you will need to adjust your rust toolchain:  

```bash
rustup default stable
rustup update nightly
rustup update stable
rustup target add wasm32-unknown-unknown --toolchain nightly
```

…then build + start the backend it with:

```bash
cd node
cargo run -- --dev --tmp
```
and start the frontend in a second console:
```bash
cd frontend
yarn install
yarn start
```

This should automatically open your browser with [localhost:8000/substrate-front-end-template]([localhost:8000](http://localhost:8000/substrate-front-end-template)).

## Funding

This project is developed for an [Open Grant] from the [Web3 Foundation] [Open Grants Program].  
It is additionally supported by the the German Ministry of Education and Science (BMBF) through a Startup Secure grant.  
<p align="center">
<a href="https://web3.foundation/about/"><img src=".assets/supported.png" width="30%"></a>
<a href="https://www.bmbf.de/"><img src=".assets/bmbf.svg" width="30%"></a>
</p>

## Security Disclaimer

This software is still under development.
The authors take no responsibility for any loss of digital assets or other damage caused by the use of it.
## Copyright

Copyright 2021 PolyCrypt GmbH, see subfolder documents for more information.    
Use of the source code is governed by the Apache 2.0 license that can be found in the [LICENSE file](LICENSE).


<!--- Links -->

[Perun Pallet]: https://github.com/perun-network/perun-polkadot-pallet
[node template]: https://github.com/substrate-developer-hub/substrate-node-template
[frontend template]: https://github.com/substrate-developer-hub/substrate-front-end-template
[src/config/types.json]: frontend/src/config/types.json
[runtime/Cargo.toml]: node/runtime/Cargo.toml
[runtime/src/lib.rs]: node/runtime/src/lib.rs#L280
[pallets/perun]: node/pallets/perun
[Open Grant]: https://github.com/perun-network/Open-Grants-Program/blob/master/applications/perun_channels.md#w3f-open-grant-proposal
[Web3 Foundation]: https://web3.foundation/about/
[Open Grants Program]: https://github.com/w3f/Open-Grants-Program#open-grants-program-
