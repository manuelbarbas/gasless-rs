"use strict";
Object.defineProperty(exports, "__esModule", { value: true });

const path = require("path");
const os = require('os');
const fs = require("fs");

function native() {

    let binaryPath;
    const platform = os.platform();

    switch (platform) {
        case 'darwin':
            binaryPath = path.join(__dirname, 'dist', 'darwin', 'index.node');
            break;
        case 'win32':
            binaryPath = path.join(__dirname, 'dist', 'win32', 'index.node');
            break;
        case 'linux':
            binaryPath = path.join(__dirname, 'dist', 'linux', 'index.node');
            break;
        default:
            throw new Error(`Unsupported platform: ${platform}`);
        }

    const modulePath = path.resolve(binaryPath);
    if (!fs.existsSync(modulePath)) {
        throw new Error(`Fail to find native module in: ${modulePath}`);
    }
    return require(modulePath);
}
const nativeModuleRef = native();

const { mineGasForTransaction } = nativeModuleRef;
exports.mineGasForTransaction = mineGasForTransaction;
