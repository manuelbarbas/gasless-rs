# Gasless

## Usage in Node.js

1. Install by running:

```shell

# NPM
npm add @eidolon-labs/gasless --save

# Yarn
yarn add @eidolon-labs/gasless --save

# PNPM
pnpm add @eidolon-labs/gasless --save

# Bun
bun add @eidolon-labs/gasless --save
```

2. Use the library to generate magic numbers

```typescript
// console.log("Mine: ", mineGasForTransaction)
import { mineGasForTransaction } from "@eidolon-labs/gasless";
const { gasPrice } = await mineGasForTransaction(evmGasLimit, "0xFromEvmAddress", evmWalletNonce);
...
// Send a transaction in any library or manual JSON-RPC Request using the magic number as the gas price
```

> Notice, this library may not work with libraries since it's unique to SKALE and allows for transactions to be sent WITHOUT having any gas in the wallet


## Contributing

1. Fork and clone the project: https://github.com/Eidolon-Labs/gasless-rs
2. Create your changes
3. Make a Pull Request

## Security & Liability

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.