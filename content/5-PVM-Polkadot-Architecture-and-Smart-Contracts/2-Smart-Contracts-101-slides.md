---
title: Smart contracts fundamentals
description: Introduction to fundamentals smart conctracts concepts
duration: 45min
url: http://localhost:1948/syllabus/3-PVM-Polkadot-Architecture-and-Smart-Contracts/2-Smart-Contracts-101-slides.md
---

<style>
    code {
        overflow: hidden!important;
        width: 120%!important;
    }
</style>

## Smart contracts fundamentals

Notes:
The goal of the lecture is to define the basic definition of what a SC is
and explain the advantage and trade off of a smart contract chain

---

# WHY?

Notes:

- Always start with the why!
- 2 things to answers here:
  - One, The benefits of smart contracts as a technology: Smart Contracts are a huge improvement over traditional legal contracts, that are slow, expensive, and error-prone.
    With smart contracts, you can automate the execution of agreements, enforce them automatically, and remove the need for intermediaries. E.g crop insurance.
  - Two, For the Polkadot ecosystem, it's important to embrace a technology that is widely adopted by the Ethereum
    community, so that we can leverage the tooling, and easily onramp existing dapps, but also users and developers.

---v

## Crop Insurance contract

<img style="width: 80%"  src="img/smart-contracts-101/crop-insurance.png" />

Notes:
See <https://www.owlexplains.com/en/podcast/ep-8-how-lemonade-is-using-smart-contracts-to-revolutionize-crop-insurance/>

---v

## Ecosystem

<img style="width: 80%"  src="img/smart-contracts-101/ecosystem.png" />

---

## Blockchain

<img style="width: 90%"  src="img/smart-contracts-101/stf.png" />

Notes:

Before defining what a smart contract (SC) is, we need to understand the environment in which it operates: the blockchain.
We won’t dive deep into what a blockchain is, that will be covered in detail during the PBA. For now, it's enough to define a blockchain as:
A distributed system where all participants (nodes) execute a common set of transactions contained in a block.
These transactions are processed using a state transition function, which updates the blockchain’s state from block n to block n+1.

---v

### Bitcoin

<img style="width: 90%"  src="img/smart-contracts-101/stf-btc.png" />

Notes:

In Bitcoin, the state transition function primarily processes transactions, which are mostly transfer transactions.

---v

### Polkadot

<img style="width: 90%"  src="img/smart-contracts-101/stf-dot.png" />

Notes:

Blockchains built with Substrate run a WebAssembly (Wasm) runtime, which defines how the blockchain's state is updated.
This runtime is built using Rust modules, called pallets, each defining specific transaction types.

Different pallets allow developers to introduce specialized logic into their blockchain:
Assets & NFT pallets → Define fungible and non-fungible tokens and their operations.
Democracy pallet → Enables on-chain governance, allowing proposals to be submitted, voted on, and enacted when approved.
Other pallets can introduce staking mechanisms, cross-chain messaging, DAOs, and more.

Polkadot's state transition function can execute any business logic defined in the runtime.
However, the runtime itself is not permissionless—it must be explicitly defined and deployed by the chain’s developers or governance
To allow arbitrary logic execution, a Smart Contracts module must be embedded in the runtime.
Otherwise, only predefined transaction types (e.g., assets, governance, staking) can be executed.

---v

### Smart contracts chain

<img style="width: 90%"  src="img/smart-contracts-101/stf-evm.png" />

Notes:

Now, we can finally define what a smart contract is.
A smart contract is a special type of account that is not controlled by a keypair, but instead by the code it defines.
A smart contract blockchain allows users to do essentially 3 things:

- Transfer value between accounts.
- Deploy contracts on-chain.
- Call these contracts to execute their logic.

You can think of smart contracts as dormant programs stored on the blockchain at a specific address.
These programs can access and store data on the blockchain, and execute logic based on the transactions they receive.

For example, the USDC smart contract, is an ERC-20 token that maintains a mapping of balances, associating each user’s address with the amount of USDC they hold. When a user transfers USDC, the contract updates the sender’s balance by subtracting the amount, adds the same amount to the recipient’s balance, and stores the new state on-chain to ensure all nodes remain synchronized

---

## Bytecode & Virtual Machines

<img style="width: 100%; " src="img/smart-contracts-101/bytecode-vm.png" />

Notes:

When we deploy a smart contract on-chain, we first compile it into bytecode, which can be executed by a virtual machine.
Different smart contract blockchains use different bytecode formats.

The state transition function of the blockchain runs the Virtual Machine to execute the instructions in this compiled bytecode, and update the state of the chain.

---

### A simple Example

```solidity[4-9|11-15|18-24]
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract PiggyBank {
    address public immutable owner;

    constructor() {
        owner = msg.sender;
    }

    function getBalance() public view returns (uint256) {
      return address(this).balance;
    }

    function deposit() public payable {}


    function withdraw(uint256 withdrawAmount) public {
        require(msg.sender == owner, "You are not the owner");
        require(address(this).balance >= withdrawAmount, "Insufficient balance");

        (bool success, ) = payable(msg.sender).call{value: withdrawAmount}("");
        require(success, "Transfer failed");
    }
}
```

Notes:
On EVM-compatible chains, the most widely used language is Solidity, that is compiled to bytecode using the `solc` compiler.
Let's go through a simple `PiggyBank` solidity contract to illustrate it.

- When a contract is deployed, the constructor code is executed
- The owner can call the deposit function to deposit some balance
- The view function `GetBalance` is a 'readonly' function that can be queried through a JSON-RPC call
- If you send ETH directly to the contract (without calling a function like deposit), it will fail unless the contract has a receive or fallback function.
  Without one of these, any direct transfer (e.g., sendTransaction from a wallet) will be rejected with an error.

---

## Core Features of Smart Contracts

---v

### Gas

<pba-flex center>

<img rounded style="width: 400px"  src="img/smart-contracts-101/gas.jpg" />

```solidity
    while (true) {
        // ...
        // This loop will consume all gas and revert
    }
}
```

<!-- .element: class="fragment" -->
</pba-flex>

Notes:

You might be wondering—if a smart contract can execute any arbitrary logic, what prevents it from defining an infinite loop that could stall the entire blockchain?
To prevent this and to protect the network from spam, virtual machines are metered. Every instruction executed by the VM has an associated gas cost, which represents the computational resources required to process it.

When you submit a transaction or when a contract calls another contract, you must specify the maximum amount of gas you’re willing to pay for execution. The contract’s code will either:

- Run to completion if there is enough gas.
- Run out of gas and revert, undoing any changes made to the contract storage, but you will still pay for the gas that was consumed before the failure.

Additionally, the blockchain itself imposes limits on gas usage:
It defines a block gas limit, which sets the maximum amount of gas that can be used across all transactions in a single block.
It also defines a gas price, which determines how much fees will be paid for a given amount of gas.
blockchains usually adjust gas prices dynamically based on network demand, ensuring fees reflect current congestion levels
This system ensures that no contract can consume unlimited resources, execution remains bounded, and transaction fees dynamically adjust based on network demand.

---v

#### Metered calls in EVM

<diagram class="mermaid">
%%{init: {'theme': 'dark', 'themeVariables': { 'darkMode': true }}}%%
flowchart LR
    Start --> FetchNextOpcode[Fetch next opcode]
    FetchNextOpcode --> LookupCost[Look up opcode cost]
    LookupCost --> CheckGas{Enough gas?}
    CheckGas -- No --> OutOfGas[Throw Out of Gas]
    CheckGas -- Yes --> DeductGas[Deduct gas]
    DeductGas --> ExecuteOpcode[Execute Opcode]
    ExecuteOpcode --> FetchNextOpcode
</diagram>

Notes:

- In pallet-revive the graph is slightly different, regular opcode are not metered but the VM injects gas metering
  instructions per basic block
- Checks before each opcode to make sure gas can be paid
- Safe: prevents unpaid work from being done
- Deterministic: results are unambiguous
- Very inefficient: lots of branching and extra work

---v

#### Weighted calls in substrate

```rust[1-8]
#[pallet::weight(T::WeightInfo::set_metadata(name.len(), symbol.len()))]
pub fn set_metadata(
    origin: OriginFor<T>,
    id: T::AssetIdParameter,
    name: Vec<u8>,
    symbol: Vec<u8>,
    decimals: u8,
) -> DispatchResult {
    let origin = ensure_signed(origin)?;
    let id: T::AssetId = id.into();
    Self::do_set_metadata(id, &origin, name, symbol, decimals)
}
```

Notes:
In Substrate, each call defines a pre-dispatch weight, which can depend on the input parameters. Accounts must pay the estimated execution fee upfront, and any excess is refunded after execution.

---v

#### Metered VM Execution vs. Weighted Calls in Substrate

| Feature             | EVM                            | Substrate                                |
| ------------------- | ------------------------------ | ---------------------------------------- |
| **Execution Model** | Metered                        | Pre-dispatch weights                     |
| **Cost**            | Dynamic                        | Static, determined pre-dispatch          |
| **Performance**     | Runtime overhead               | Optimized execution                      |
| **Flexibility**     | Supports arbitrary computation | Requires (benchmnarked) weights per call |

Notes:

In VM-based blockchains, execution is metered using gas.
This makes execution flexible but introduces runtime overhead due to dynamic metering.

Contract execution performance is **less predictable**, as total costs depend on actual execution flow.
Wallet usually need to dry-run the execution to define how much gas is required for the execution.

In Substrate-based chains, execution is handled differently

- Instead of metering each instruction at runtime, calls have predefined weights based on computational complexity.
- This approach enables more efficient execution compared to metered VM, as the chain doesn’t need to meter each instruction dynamically, reducing runtime overhead.

---

### Composability

<img rounded style="width: 500px"  src="img/smart-contracts-101/composability.jpg" />

Notes:
Smart contracts are highly composable, meaning they can interact with each other to execute complex workflows. A contract call is always initiated by an Externally Owned Account (EOA) through a transaction.

---v

### Call types

- Normal Call: Contract A calls Contract B
- Static Call: Contract A reads data from Contract B
- Delegate Call: Contract A executes Contract B in its execution context

Notes:
Methods for contract interaction include:

- **Normal Call**:

  - Contract A calls Contract B.
  - Contract B is pushed to the call stack, executes, potentially updates its state, and may call other contracts.
  - Contract B returns control to Contract A along with the result and status (reverted or successful).
    An example for this would be a contract that calls another contract to transfer tokens.

- **Static Call**:

  - Contract A calls Contract B.
  - Contract B executes but cannot make state changes.
    An example for this would be a contract that reads data from another contract, e.g a price feed.

- **Delegate Call**:
  - Contract A calls Contract B in its execution context. (Storage reads/writes affect A, not B)
  - Contract B can read and write to A's storage, akin to a library executing within A.
    An example for this would be to use a Math library to perform calculations.

A good mental model is to think of smart contracts as the API layer of web3. Your contract can tap into other contracts
to access their functionality, read and write data and execute complex workflows. A good example of that are flash loans
on the Aave protocol, where a contract can borrow funds from the protocol, execute a series of transactions and repay
the loan in the same transaction.

---

### Call types

<img style="height: 90%"  src="img/smart-contracts-101/call-stack-1.png" />

---v

### Call types

<img style="height: 90%"  src="img/smart-contracts-101/call-stack-2.png" />

---v

### Call types

<img style="height: 90%"  src="img/smart-contracts-101/call-stack-3.png" />

---v

### Precompiled Contracts

<img rounded style="width: 500px"  src="img/smart-contracts-101/precompiles.jpg" />

Notes:

Another important feature of smart contracts is the ability to interact with precompiled contracts.
A precompile contract is a contract whose code is defined in the client software directly.
On Ethereum, that has a slow VM, it's used to perform computation intensive operations like elliptic curve cryptography,
outside the EVM, to improve performance.
In Substrate, a Smart-Contract pallet, can leverage this to expose other features of the runtime (like staking, xcm, governance, assets) to smart contracts

---v

### Immutability

<img rounded style="width: 500px"  src="img/smart-contracts-101/Immutability.jpg" />

Notes:
Contracts are immutable by design, however in some circumstances, you might want to upgrade to fix a bug or add or
improve existing features. There are several patterns to achieve this, one of the most common is the Proxy pattern.
Essentially, the proxy pattern involves creating a proxy contract that delegates calls to an implementation contract.
When you want to upgrade the contract, you deploy a new implementation contract and update the proxy to point to the new implementation.

---v

#### Smart Contract vs. Substrate Upgrades

|                   | Smart Contracts                       | Substrate                              |
| ----------------- | ------------------------------------- | -------------------------------------- |
| **Upgradability** | Immutable (requires proxy)            | Yes through Runtime Upgrade            |
| **Governance**    | Contract owner / DAO                  | Sudo / On-chain OpenGov                |
| **Overhead**      | Gas costs for proxy & state migration | No gas cost (protocol-level execution) |

---

## Security

<img rounded style="width: 500px"  src="img/smart-contracts-101/security.jpg" />

Notes:

Permissionless Deployment is Risky

Anyone can deploy a smart contract, but if "code is law," any bug or vulnerability can be exploited.
Attackers actively search for vulnerabilities in deployed contracts.
Even small logic errors or gas inefficiencies can be exploited for financial gain.

- Security audits are essential before deploying contracts that manage funds.
- Use battle-tested smart contract libraries (e.g., OpenZeppelin).
- Follow established design patterns to avoid common vulnerabilities (e.g., reentrancy).
- Implement proper access control to prevent unauthorized actions.

---v

### Reentrency bug

```solidity[1-2|4-12]
contract Vulnerable {
    mapping(address => uint256) public balances;

    function withdraw() external {
        uint256 amount = balances[msg.sender];

        //❗ Sends ETH before updating balance
        (bool success, ) = msg.sender.call.value(amount)("");
        require(success, "Transfer failed.");

        //❗ Balance Update after Transfer - Allows Reentrancy!
        balances[msg.sender] = 0;
    }
}
```

Note:
see <https://blog.chain.link/reentrancy-attacks-and-the-dao-hack/>

---v

### Activity: Reproduce the DAO hack

> Deploy and execute the DAO hack on a local chain

Note:
see <https://github.com/paritytech/contracts-boilerplate>

---

## JSON-RPC

- JSON-RPC is a **remote procedure call (RPC) protocol** using JSON for encoding requests and responses.
- It allows **external applications, wallets, and scripts** to interact with blockchain nodes.
- Most chains expose a **JSON-RPC API** for querying blockchain data and sending transactions.

---v

### Common Ethereum JSON-RPC Methods

| Method                      | Description                                     |
| --------------------------- | ----------------------------------------------- |
| `eth_call`                  | Executes a read-only contract call.             |
| `eth_estimateGas`           | Estimates the gas required for a transaction.   |
| `eth_sendRawTransaction`    | Sends a raw, signed transaction to the network. |
| `eth_getTransactionReceipt` | Retrieves transaction execution details.        |

Note:

- There are two types of transactions: read-only and state-changing transactions.
  When you want to read data from a contract, you use `eth_call` to execute a read-only contract call.

- When you want to send a transaction to the network, you will usually follow this flow:
- Estimate the gas required for the transaction using `eth_estimateGas`.
- Sign and submit the transaction using `eth_sendRawTransaction`.
- Finally, you can poll `eth_getTransactionReceipt` with the transaction hash to retrieve the transaction execution details.

The receipt is an important object, used by wallet and JS libraries, it will contain

- The transaction status
- Gas used, and logs generated during execution.

---v

### Common Substrate JSON-RPC Methods

| Method                   | Description                              |
| ------------------------ | ---------------------------------------- |
| `state_call`             | Calls a runtime API exposed by a pallet. |
| `author_submitExtrinsic` | Submits a signed transaction             |

---v

### Example sending a raw transaction

```json
curl https://westend-asset-hub-eth-rpc.polkadot.io \
-H 'content-type: application/json' \
-d '{
  "method":"eth_sendRawTransaction",
  "params" ["0x02f8b3018313c1..."],
  "id":2
  ,"jsonrpc":"2.0"
}'
```

---v

## Structure of a Transaction

```sh
> cast decode-tx 0x02f8b3018313c17...
```

<!--https://etherscan.io/getRawTx?tx=0xcd58fbee0f90c4b7136a5af85876090dd1593e4580f840bcf0a7b9219772a5d4-->

```json
{
  "type": "0x2",
  "chainId": "0x1",
  "nonce": "0x13c174",
  "gas": "0x249f0",
  "maxFeePerGas": "0x746a528800",
  "maxPriorityFeePerGas": "0x878415",
  "to": "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
  "value": "0x0",
  "input": "0x..."
  "r": "0xc330502a046982553df56842433dfb1f318c980724bfd30be53e6461cea620ac",
  "s": "0x25217d80ae9538009b3b24ab83fdac6df67982b433f74488d2c14fee41ca2d79",
  "yParity": "0x0",
  "v": "0x0",
}
```

<a target="_blank" href="https://etherscan.io/tx/0xcd58fbee0f90c4b7136a5af85876090dd1593e4580f840bcf0a7b9219772a5d4">Etherscan</a>

Notes:
A few things to note in the transaction:

- First of all the transaction is encoded using RLP (Recursive Length Prefix) encoding, the first byte is the type of the
  transaction, and defines the format of the transaction, that can be decoded using rlp-decode.
- The type here is 0x2, which means it is an EIP-1559 transaction introduced by the London fork, new fork can sometimes introduce new transaction type
  to define new transaction format, this type field is used to distinguish between them.
- The chainId defines the network on which the transaction is being sent, this makes sure that the transaction is not replayed on another network.
- The nonce is a very important component of the transaction, it is used to prevent replay attacks, it is incremented for each transaction sent by an account, your transaction will only be executed if the nonce is the next in line.
- gas, maxFeePerGas, maxPriorityFeePerGas are used to define the cost and fees generated for the transaction.
- value is the amount of ether being sent in the transaction.
- input is the ABI encoded data of the function being called, in this case, it is the transfer function of an ERC20 token, we will see how to decode this later.
- r, s, v are the signature of the transaction, used to verify the transaction was signed by the sender.

You will notice that the transaction does not have a 'from' field, this is because the origin can be recovered from the signature.

---v

### ABI decoding

```sh[1-7|8-13]
INPUT="0xa9059cbb000000000000000000000000ba04f1c1e4577165dd2297d3fbedf956b0e4c8a70000000000000000000000000000000000000000000000000000000004cc7c30"

# Get the selector for a function
> cast keccak "transfer(address,uint256)" | cut -c 1-10 # or cast sig "..."

0xa9059cbb

# Lookup the function signature and input data
> cast 4byte-decode $INPUT

1) "transfer(address,uint256)"
0xBA04f1c1E4577165dD2297D3FbEdF956B0e4C8a7
80510000 [8.051e7]


```

Notes:
Now that we have decoded the transaction, let's look at how the input data is encoded

- The first 4 bytes of the input data are the function selector
- They are derived from the first 4 bytes of the keccak hash of the function signature.
- The VM use the selector to lookup the address of the function being called (here the transfer function), and jump to it.
- The rest of the input are the ABI encoded parameter of the function

---v

### Encoding ABI parameters

```sh
cast abi-encode \
  "test(bool, string, address)" \
  "true"  "hello" "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48" \
| xxd -r -p | xxd -c 32
```

```hexdump[1 | 2 | 4 | 5 | 3]
00: 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0001
20: 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0060
40: 0000 0000 0000 0000 0000 0000 a0b8 6991 c621 8b36 c1d1 9d4a 2e9e b0ce 3606 eb48
60: 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0005
80: 6865 6c6c 6f00 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000
```

Notes:

Let's look at how the parameters are encoded using EVM ABI encoding standard.
The ABI defines the standard way to interact with contracts both from outside the blockchain and for contract-to-contract interaction

The EVM operates on 32-byte words, meaning that all storage, memory, and stack operations use 32-byte slots.
Arguments are padded and aligned to 32 bytes, to make it easy to read or load them from memory during execution.

For this example signature the data is encoded as follows:

- The first element is a bool. Even though a bool is just 1 byte, like everything in the EVM, it is padded to 32 bytes.
- The string is dynamic type, so instead of being stored inline, its offset (0x60) is stored next.
- The address is a fixed-size type, so it’s stored inline immediately after the offsets.
- At offset 0x60, we find the length of the string, and right after that, the string content itself.

See <https://docs.soliditylang.org/en/latest/abi-spec.html>

---

## Beyond Smart Contracts

- 🔎 Block Explorers (e.g., Etherscan, Subscan)
- 🔮 Oracles (e.g., Chainlink, Redstone)
- 📊 Indexers (e.g., The Graph, Subsquid)

Notes:
Bock explorers:

- Track transactions and smart contract states
- Enable transparency & debugging tools for developers
- Examples: Etherscan, Subscan, Blockscout
  Orcales:
- Enable hybrid on-chain/off-chain contracts
- Examples: Chainlink, Redstone
- Deliver real-world data on-chain for smart contracts (e.g., price feeds, weather, events)
- Two models
  - Push: Data is pushed by node operators at specific interval and made available to contracts
  - Pull: Signed data package is attached to the transaction and verified by a contract on chain
    Indexers:
- Blockchain are write optimized, and querying data can be slow
- Indexers subscribe to the blockchain and store the data in a more queryable format
- Allow fast and structured access to blockchain records
- Improve UX for dApps by reducing raw node queries
- Examples: The Graph, Subsquid
