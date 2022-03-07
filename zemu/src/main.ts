import TransportNodeHid from "@ledgerhq/hw-transport-node-hid";
import StarkwareApp from '@zondax/ledger-starkware-app'

import { ec as stark_ec } from 'starknet'

const APP_DERIVATION: string = "m/2645'/579218131'/0'/0'"

const FELT = Buffer.alloc(32, 0x01);

async function main() {
  const transport = await TransportNodeHid.create();
  const app = new StarkwareApp(transport);

  const resp = await app.signFelt(APP_DERIVATION, FELT);
  console.log(resp);

  const resp_addr = await app.getPubKey(APP_DERIVATION);
  console.log(resp_addr);

  let signatureOK = true
  const keypair = stark_ec.getKeyPairFromPublicKey('0x' + resp_addr.publicKey.toString('hex'));

  signatureOK = stark_ec.verify(keypair, '0x' + FELT.toString('hex'), ['0x' + resp.r.toString('hex'), '0x' + resp.s.toString('hex')]);

  console.log(signatureOK)
}

main()
  .catch(console.error)
  .finally(() => process.exit())
