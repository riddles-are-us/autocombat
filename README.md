# Introduction

This is a generic template for creating a trustless auto card game that runs in the ZKWASM VM and performs on-chain settlement. To ensure fairness in a PVE game between players and the server, some special design steps are needed. These steps have been abstracted into five key stages:

1. **Seed Commitment**: The admin generates a random seed and commits to it by storing its hash, `commitment = hash(seed)`, in the game state.
2. **Player Signature**: The player signs the commitment of the random seed and provides their signature (`SR`).
3. **Seed Revelation**: The admin reveals the seed by submitting it to the server.
4. **Verification**: The server verifies that the revealed seed is correct by checking that `hash(seed) = commitment`.
5. **Random Number Generation**: Then, a random number for the player is generated using the formula `seed xor SR`.

This process ensures the integrity of the game's randomness while maintaining fairness through verifiable inputs from both the admin and the player.

## Install ZKWASM-MINI-ROLLUP

1. Get the ZKWASM MINI service
```
git clone https://github.com/DelphinusLab/zkwasm-mini-rollup.git
```

2. Start assisting services using docker-compose
```
zkwasm-mini-rollup: docker-compose up
```

## Compile and test game image
1. Install application server under ts
```
ts: npm install
```

There is a Makefile in the root directory that will compile the rust code into WASM and copy the compiled WASM binary into the application folder
```
INSTALL_DIR=./ts/node_modules/zkwasm-ts-server/src/application
RUNNING_DIR=./ts/node_modules/zkwasm-ts-server

.PHONY: deploy

build:
	wasm-pack build --release --out-name application --out-dir pkg
	#wasm-opt -Oz -o $(INSTALL_DIR)/application_bg.wasm pkg/application_bg.wasm
	cp pkg/application_bg.wasm $(INSTALL_DIR)/application_bg.wasm
	cp pkg/application.d.ts $(INSTALL_DIR)/application.d.ts
	cp pkg/application_bg.js $(INSTALL_DIR)/application_bg.js
	cp pkg/application_bg.wasm.d.ts $(INSTALL_DIR)/application_bg.wasm.d.ts
	cd $(RUNNING_DIR) && npx tsc && cd -

clean:
	rm -rf pkg
	rm -rf $(INSTALL_DIR)/application_bg.wasm
	rm -rf $(INSTALL_DIR)/application.d.ts
	rm -rf $(INSTALL_DIR)/application_bg.js
	rm -rf $(INSTALL_DIR)/application_bg.wasm.d.ts

run:
	node ./ts/node_modules/zkwasm-ts-server/src/service.js
```

2. Compile the WASM image:
```
make
```
3. Specify Your Deployment Environment
   
Fill the .env file with the following content:
#IMAGE="784A58A49E44A85B1C508BC796166CBA"
#DEPLOY=TRUE
#USER_PRIVATE_ACCOUNT="..."
#USER_ADDRESS="..."
#SETTLEMENT_CONTRACT_ADDRESS="..."
#RPC_PROVIDER="..."
#SETTLER_PRIVATE_ACCOUNT="..."

Here’s what each variable means:

IMAGE: This is the hash of the image from the ZKWASM explorer, which corresponds to the compiled game code or application. Replace the placeholder with the actual hash from the ZKWASM explorer.

DEPLOY: Setting this to TRUE indicates that the deployment process should be executed when the script is run.

USER_PRIVATE_ACCOUNT: This is the private key of the user or player account. Replace this with the appropriate value to authorize the user’s participation in the game or deployment.

USER_ADDRESS: The public address associated with the user's private account. This is needed for identifying the user on the blockchain.

SETTLEMENT_CONTRACT_ADDRESS: This is the address of the smart contract handling the game's settlement on-chain. You’ll replace this with the actual contract address where the game’s final state will be verified and settled.

RPC_PROVIDER: The Remote Procedure Call (RPC) provider's URL. This is used to interact with the blockchain network where the game is deployed.

SETTLER_PRIVATE_ACCOUNT: The private key of the settler account, responsible for submitting and verifying game settlements on-chain.

4. Run the RPC server
```
make run
```

## Deploy your RPC server:
1. Specify your deploy environment by filling the ```.env``` file with empty content
```
IMAGE="Your published image hash"
DEPLOY=TRUE
```
where IMAGE is the image hash in the ZKWASM explorer

2. Run the RPC server
```
make run
```
