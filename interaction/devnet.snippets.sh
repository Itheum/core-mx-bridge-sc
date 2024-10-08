PROXY=https://devnet-gateway.multiversx.com
CHAIN_ID="D"

WALLET="wallet_deployer.pem"

ADDRESS=$(mxpy data load --key=address-devnet)
DEPLOY_TRANSACTION=$(mxpy data load --key=deployTransaction-devnet)

TOKEN="ITHEUM-fce905"
TOKEN_HEX="0x$(echo -n ${TOKEN} | xxd -p -u | tr -d '\n')"

# to deploy from last reprodubible build, we need to change or vice versa
# --bytecode output/core-mx-bridge-sc.wasm \
# to 
# --bytecode output-docker/core-mx-bridge-sc/core-mx-bridge-sc.wasm \
deploy(){
    mxpy --verbose contract deploy \
    --bytecode output-docker/core-mx-bridge-sc/core-mx-bridge-sc.wasm \
    --outfile deployOutput \
    --metadata-payable-by-sc \
    --metadata-payable \
    --pem ${WALLET} \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --gas-limit 150000000 \
    --send \
    --recall-nonce \
    --outfile="./interaction/deploy-devnet.interaction.json" || return

    TRANSACTION=$(mxpy data parse --file="./interaction/deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(mxpy data parse --file="./interaction/deploy-devnet.interaction.json" --expression="data['contractAddress']")

    mxpy data store --key=address-devnet --value=${ADDRESS}
    mxpy data store --key=deployTransaction-devnet --value=${TRANSACTION}
}

# any change to code or property requires a full upgrade 
# always check if you are deploy via a reprodubible build and that the code hash is the same before and after upgrade (that is if you are only changing props and not code.. for code, the RB will be different)
# if only changing props, you can't just "append" new props. you have to add the old ones again and then add a new prop you need. i.e. it's not append, it's a whole reset
# for upgrade, --outfile deployOutput is not needed
# in below code example we added --metadata-payable to add PAYABLE to the prop of the SC and removed --metadata-not-readable to make it READABLE
upgrade(){
    mxpy --verbose contract upgrade ${ADDRESS} \
    --bytecode output-docker/core-mx-bridge-sc/core-mx-bridge-sc.wasm \
    --metadata-payable-by-sc \
    --metadata-payable \
    --pem ${WALLET} \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --gas-limit 150000000 \
    --recall-nonce \
    --send || return
}

# if you interact without calling deploy(), then you need to 1st run this to restore the vars from data
restoreDeployData() {
  TRANSACTION=$(mxpy data parse --file="./interaction/deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
  ADDRESS=$(mxpy data parse --file="./interaction/deploy-devnet.interaction.json" --expression="data['contractAddress']")

  # after we upgraded to mxpy 8.1.2, mxpy data parse seems to load the ADDRESS correctly but it breaks when used below with a weird "Bad address" error
  # so, we just hardcode the ADDRESS here. Just make sure you use the "data['contractAddress'] from the latest deploy-devnet.interaction.json file
  ADDRESS="erd1qqqqqqqqqqqqqpgqfjp567v5k9a6sd9yqn0a5hv5y4vefhn3fsxstq2kuk"
}

setAdministrator(){
    # $1 = address

    address="0x$(mxpy wallet bech32 --decode ${1})"

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setAdministrator" \
    --arguments $address \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

setRelayer(){
    # $1 = address

    address="0x$(mxpy wallet bech32 --decode ${1})"

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setRelayer" \
    --arguments $address \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

addTokenToWhitelist(){
    # $1 = token
    # $2 = decimals

    token="0x$(echo -n ${1} | xxd -p -u | tr -d '\n')"

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "addTokenToWhitelist" \
    --arguments $token $2 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return

}

setFeeCollector(){
    # $1 = address

    address="0x$(mxpy wallet bech32 --decode ${1})"

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setFeeCollector" \
    --arguments $address \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

setFeeValue(){
    # $1 = value

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setFeeValue" \
    --arguments $1 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

setDepositLimits(){
    # $1 = min
    # $2 = max

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setDepositLimits" \
    --arguments $1 $2 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return

}

setWegldTokenIdentifier(){
    # $1 = token

    token="0x$(echo -n ${1} | xxd -p -u | tr -d '\n')"

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setWegldTokenIdentifier" \
    --arguments $token \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

setPublicStateActive(){
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setPublicStateActive" \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

setPublicStateInactive(){
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setPublicStateInactive" \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

setRelayerStateActive(){
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setRelayerStateActive" \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

setRelayerStateInactive(){
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setRelayerStateInactive" \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

setWhitelistStateActive(){
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setWhitelistStateActive" \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

setWhitelistStateInActive(){
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setWhitelistStateInactive" \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

addToWhitelist(){
    # $1 = address

    address="0x$(mxpy wallet bech32 --decode ${1})"

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "addToWhitelist" \
    --arguments $address \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

removeFromWhitelist(){
    # $1 = address

    address="0x$(mxpy wallet bech32 --decode ${1})"

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "removeFromWhitelist" \
    --arguments $address \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}
