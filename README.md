# Introduction
This is a generic template about creating a trustless auto card game that runs in the ZKWASM VM and performs settlement onchain. In general, implementing an PVE game between players and the server needs some special design to make sure the game is fair. We abstract these special stages into 4 stages

0. Admin generates a random seed and commit the seed into the state by commitment = hash(seed)
1. Player sign the commitment of the random seed and provide a signature SR
2. Admin reveal the seed by providing the seed to the server
3. Server check the seed has correct hash by hash(seed) = commitment
4. A random number is then generated for the Player by seed xor SR

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

3. Specify your deploy environment by filling the ```.env``` file with empty content
```
#IMAGE="784A58A49E44A85B1C508BC796166CBA"
#DEPLOY=TRUE
#USER_PRIVATE_ACCOUNT="..."
#USER_ADDRESS="..."
#SETTLEMENT_CONTRACT_ADDRESS="..."
#RPC_PROVIDER="..."
#SETTLER_PRIVATE_ACCOUNT="..."
```
where IMAGE is the image hash in the ZKWASM explorer

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
