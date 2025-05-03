export interface MiningOutput {
    duration: number;
    gas_price: string;
}
export declare function mineGasForTransaction(gas_amount: number, address: string, nonce: number): Promise<MiningOutput>;
