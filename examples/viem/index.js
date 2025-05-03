const { mine_gas_for_transaction } = require('../../dist/index.node');
console.log("Mine: ", mine_gas_for_transaction)


// import { createWalletClient, http } from "viem";
// import { skaleCalypsoTestnet } from "viem/chains";
// import { generatePrivateKey, privateKeyToAccount } from "viem/accounts";
// import * as gasless from "gasless-rs"
// import * as native from "../../dist/index.node";
// const gasless = require("../../dist/index.node");
// console.log("gasless: ", Object.keys(gasless))
// async function main() {
//     const privateKey = generatePrivateKey();
    
//     const wallet = createWalletClient({
//         chain: skaleCalypsoTestnet,
//         transport: http(),
//         account: privateKeyToAccount(privateKey)
//     });

//     const magicValue = await mine_gas_for_transaction(100_000, wallet.account.address, 0);
//     console.log("Magic Value: ", magicValue);

//     await wallet.sendTransaction({
//         to: "0x62Fe932FF26e0087Ae383f6080bd2Ed481bA5A8A",
//         data: `0x0c11dedd000000000000000000000000${wallet.account.address.substring(2)}`,
//         gas: BigInt(100_000),
//         gasPrice: magicValue
//     })
// }

// main()
//     .catch((err) => {
//         console.error("err: ", err);
//         throw err;
//     })