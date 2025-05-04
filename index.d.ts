export interface MiningOutput {
    duration: number;
    gasPrice: string;
}
export declare function mineGasForTransaction(gas_amount: number, address: string, nonce: number): Promise<MiningOutput>;
