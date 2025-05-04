// import { mineGasForTransaction } from '../../index.js';
// console.log("Mine: ", mineGasForTransaction)
import { mineGasForTransaction } from "@eidolon-labs/gasless";


import { createPublicClient, createWalletClient, http } from "viem";
import { skaleCalypsoTestnet } from "viem/chains";
import { generatePrivateKey, privateKeyToAccount } from "viem/accounts";
async function main() {
    const privateKey = generatePrivateKey();
    
    const client = createPublicClient({
        chain: skaleCalypsoTestnet,
        transport: http()
    });

    const wallet = createWalletClient({
        chain: skaleCalypsoTestnet,
        transport: http(),
        account: privateKeyToAccount(privateKey)
    });

    const { gasPrice } = await mineGasForTransaction(100_000, wallet.account.address, 0);
    console.log("Magic Value: ", gasPrice);

    const res = await wallet.sendTransaction({
        to: "0x62Fe932FF26e0087Ae383f6080bd2Ed481bA5A8A",
        data: `0x0c11dedd000000000000000000000000${wallet.account.address.substring(2)}`,
        gas: BigInt(100_000),
        gasPrice: BigInt(gasPrice)
    });

    console.log("Res: ", res);

    const receipt = await client.waitForTransactionReceipt({
        hash: res
    });

    console.log("Receipt: ", receipt);
}

main()
    .catch((err) => {
        console.error("err: ", err);
        throw err;
    })