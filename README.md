<h1 align="center">cast-gpt</h1>
<div align="center">
 <strong>
    A simple CLI for generating cast commands for foundry
 </strong>
</div>
<br/>

## Command Line Interface Example

cargo run --bin cast-gpt <address> "generate all the cast commands based on the abi provided"

cargo run --bin cast-gpt 0x6ADb2E268de2aA1aBF6578E4a8119b960E02928F "generate all the cast commands based on the abi provided"

1inch router contract: 0x1111111254EEB25477B68fb85Ed929f73A960582

ens base registrar: 0x57f1887a8BF19b14fC0dF6Fd9B2acc9Af147eA85

chainlink token: 0x514910771af9ca656af840dff83e8264ecf986ca
cast cmd: [
"cast call 0x514910771AF9Ca656af840dff83E8264EcF986CA name --rpc-url https://rpc.mevblocker.io",
"cast call 0x514910771AF9Ca656af840dff83E8264EcF986CA approve(address,uint256) --rpc-url https://rpc.mevblocker.io",
"cast call 0x514910771AF9Ca656af840dff83E8264EcF986CA totalSupply --rpc-url https://rpc.mevblocker.io",
"cast call 0x514910771AF9Ca656af840dff83E8264EcF986CA transferFrom(address,address,uint256) --rpc-url https://rpc.mevblocker.io",
"cast call 0x514910771AF9Ca656af840dff83E8264EcF986CA decimals --rpc-url https://rpc.mevblocker.io",
"cast call 0x514910771AF9Ca656af840dff83E8264EcF986CA transferAndCall(address,uint256,bytes) --rpc-url https://rpc.mevblocker.io",
"cast call 0x514910771AF9Ca656af840dff83E8264EcF986CA decreaseApproval(address,uint256) --rpc-url https://rpc.mevblocker.io",
"cast call 0x514910771AF9Ca656af840dff83E8264EcF986CA balanceOf(address) --rpc-url https://rpc.mevblocker.io",
"cast call 0x514910771AF9Ca656af840dff83E8264EcF986CA symbol --rpc-url https://rpc.mevblocker.io",
"cast call 0x514910771AF9Ca656af840dff83E8264EcF986CA transfer(address,uint256) --rpc-url https://rpc.mevblocker.io",
"cast call 0x514910771AF9Ca656af840dff83E8264EcF986CA increaseApproval(address,uint256) --rpc-url https://rpc.mevblocker.io",
"cast call 0x514910771AF9Ca656af840dff83E8264EcF986CA allowance(address,address) --rpc-url https://rpc.mevblocker.io",
]

uma voting token: 0x04Fa0d235C4abf4BcF4787aF4CF447DE572eF828

## License

`cast-gpt` is licensed under the MIT license. Please read the [LICENSE](LICENSE) file in this repository for more information.
