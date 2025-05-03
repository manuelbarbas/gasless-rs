const { mineGasForTransaction } = require("./dist/lib.js");
async function main() {
    console.log("Res: ", await mineGasForTransaction(42, "0x742d35Cc6634C0532925a3b844Bc454e4438f44e", 21000));
}

main()