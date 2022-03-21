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
    
    near create-account rust_module_name.testnet_name --master_account testnet_name
    $ near create-account ledge.abc.testnet --master_account abc.testnet

    near state abc.testnet
    $ near state abc.testnet

    near keys abc.testnet
    near deploy --accountId abc.testnet --wasmFile res/wasm_Project_file

Contract Explorer
    https://explorer.testnet.near.org/transactions/HTBab46sAUq7uFbVdmZ3FYwu6HYMp4Z5hjTBXRCiaXBi
