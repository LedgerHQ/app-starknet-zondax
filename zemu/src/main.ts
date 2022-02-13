import TransportNodeHid from "@ledgerhq/hw-transport-node-hid";
import StarkwareApp from '@zondax/ledger-starkware-app'

const APP_DERIVATION: string = "m/2645'/579218131'/0'/0'"

async function main() {
  const transport = await TransportNodeHid.create();
  const app = new StarkwareApp(transport);

  const resp = await app.signFelt(APP_DERIVATION, Buffer.alloc(32, 0xA0));

  console.log(resp)
}

main()
  .catch(console.error)
  .finally(() => process.exit())
