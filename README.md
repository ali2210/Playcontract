# Playcontract
Play with contract 


Installation contract 
    
    NPM Package 
    npm version required > 8
    
    
    rust installed [if not]
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

    near installed for development purpose 
    npm install -g near-cli

    [validate version]
    near -v

Running Near Blockchain 

    [terminal]
    $ near 

Login Near Account or Create one 
    $ near login

Additional commands for near contract
    
    near view abc.testnet
    $ near view abc.testnet
    
    near create-account contract_name.testnet_name --master_account testnet_name
    $ near create-account ledge.abc.testnet --master_account abc.testnet

    near state abc.testnet
    $ near state abc.testnet

    near keys abc.testnet
    near deploy --accountId abc.testnet --wasmFile res/wasm_Project_file
    
    near contract_name.abc.testnet  abc.testnet 
    $ near delete ledge.playcontract.testnet playcontract.testnet
    
    near call contract_name.abc.testnet function_name '{"Param_1", "value"}'
    $ near call contract_name.abc.testnet init '{"account", "HTBab46sAUq7uFbVdmZ3FYwu6HYMp4Z5hjTBXRCiaXBi"}'

Contract Explorer
    https://explorer.testnet.near.org/transactions/HTBab46sAUq7uFbVdmZ3FYwu6HYMp4Z5hjTBXRCiaXBi
    https://explorer.testnet.near.org/transactions/AkhdyoypmKHtEZTc9KfyK8mC8Mt4c9myuYpLPCwBcdht

Limitation 
    There are other ways to optimise the contract, for the learn purpose it is good. However, it's little complex more https://www.near-sdk.io/building/basic-build .
