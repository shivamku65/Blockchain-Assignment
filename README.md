# ICP Token Wallet

## Setup

1. Install Rust:
   
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh


2. Install DFINITY SDK

   sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"


DEPLOYEMENT  :
 
1. Start the local ICP test network:

   dfx start --background

2. Deploy the smart contracts:

   dfx canister create --all
   dfx build
   dfx canister install --all

3. Testing

   cargo test
