## Build web3 authentication header with Crust   
https://wiki.crust.network/docs/en/buildFileStoringWithGWDemo   

#### Code converts it to rust   

```js
const { Keyring } = require('@polkadot/keyring');

const seeds = "caution juice atom organ advance problem want pledge someone senior holiday very";

// 2. Construct auth header
const keyring = new Keyring();
const pair = keyring.addFromUri(seeds);
const sig = pair.sign(pair.address);
const sigHex = '0x' + Buffer.from(sig).toString('hex');

const authHeader = Buffer.from(`sub-${pair.address}:${sigHex}`).toString('base64');
```