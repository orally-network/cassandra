
all: local_deploy_cassandra

update_candid:
	cargo test update_candid



local_deploy_cassandra: update_candid 
ifndef GOOGLE_OAUTH_CLIENT_ID
	$(error GOOGLE_OAUTH_CLIENT_ID ENV is undefined)
endif

ifndef GOOGLE_OAUTH_CLIENT_SECRET
	$(error GOOGLE_OAUTH_CLIENT_SECRET ENV is undefined)
endif

ifndef GOOGLE_OAUTH_REDIRECT_URL
	$(error GOOGLE_OAUTH_REDIRECT_URL ENV is undefined)
endif

ifndef GITHUB_OAUTH_CLIENT_ID
	$(error GITHUB_OAUTH_CLIENT_ID ENV is undefined)
endif

ifndef GITHUB_OAUTH_CLIENT_SECRET
	$(error GITHUB_OAUTH_CLIENT_SECRET ENV is undefined)
endif


	dfx canister create cassandra && dfx build cassandra && gzip -f -1 ./.dfx/local/canisters/cassandra/cassandra.wasm
	dfx canister install --wasm ./.dfx/local/canisters/cassandra/cassandra.wasm.gz --argument \
		"(\"dfx_test_key\", \"https://rpc.orally.network/?rpc=\", \"${GOOGLE_OAUTH_CLIENT_ID}\", \"${GOOGLE_OAUTH_CLIENT_SECRET}\", \"${GOOGLE_OAUTH_REDIRECT_URL}\", \
			\"${GITHUB_OAUTH_CLIENT_ID}\", \"${GITHUB_OAUTH_CLIENT_SECRET}\")" cassandra




local_upgrade: local_upgrade_cassandra 

local_upgrade_cassandra: update_candid 
	dfx build cassandra 
	gzip -f -1 ./.dfx/local/canisters/cassandra/cassandra.wasm
	dfx canister install --mode upgrade --wasm ./.dfx/local/canisters/cassandra/cassandra.wasm.gz cassandra


ic_upgrade: ic_upgrade_cassandra


ic_upgrade_cassandra: update_candid
	dfx build cassandra --network ic && gzip -f -1 ./.dfx/ic/canisters/cassandra/cassandra.wasm
	dfx canister install --mode upgrade --wasm ./.dfx/ic/canisters/cassandra/cassandra.wasm.gz --network ic cassandra
	dfx canister call cassandra upgrade_chains --ic 

ic_deploy_cassandra: update_candid 
ifndef GOOGLE_OAUTH_CLIENT_ID
	$(error GOOGLE_OAUTH_CLIENT_ID ENV is undefined)
endif

ifndef GOOGLE_OAUTH_CLIENT_SECRET
	$(error GOOGLE_OAUTH_CLIENT_SECRET ENV is undefined)
endif

ifndef GOOGLE_OAUTH_REDIRECT_URL
	$(error GOOGLE_OAUTH_REDIRECT_URL ENV is undefined)
endif

ifndef GITHUB_OAUTH_CLIENT_ID
	$(error GITHUB_OAUTH_CLIENT_ID ENV is undefined)
endif

ifndef GITHUB_OAUTH_CLIENT_SECRET
	$(error GITHUB_OAUTH_CLIENT_SECRET ENV is undefined)
endif
	dfx canister create cassandra
	dfx build cassandra && gzip -f -1 ./.dfx/local/canisters/cassandra/cassandra.wasm
	dfx canister install --wasm ./.dfx/local/canisters/cassandra/cassandra.wasm.gz --argument \
		"(\"key_1\", \"https://rpc.orally.network/?rpc=\", \"${GOOGLE_OAUTH_CLIENT_ID}\", \"${GOOGLE_OAUTH_CLIENT_SECRET}\", \"${GOOGLE_OAUTH_REDIRECT_URL}\", \
			\"${GITHUB_OAUTH_CLIENT_ID}\", \"${GITHUB_OAUTH_CLIENT_SECRET}\")" cassandra --ic 


