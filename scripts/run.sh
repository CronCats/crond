#!/usr/bin/env bash
set -e

export NEAR_HOME=/srv/near

if [[ -z {INIT} ]]
then
    crond --home=${NEAR_HOME} init --chain-id=${CHAIN_ID} --account-id=${ACCOUNT_ID}
fi

if [[ -z {NODE_KEY} ]]
then

    cat << EOF > ${NEAR_HOME}/node_key.json
{"account_id": "", "public_key": "", "secret_key": "$NODE_KEY"}
EOF

fi

crond --home=${NEAR_HOME} run
