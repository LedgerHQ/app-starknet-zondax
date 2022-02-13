/** ******************************************************************************
 *  (c) 2020 Zondax GmbH
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 ******************************************************************************* */

import Zemu from '@zondax/zemu'
import { APP_DERIVATION, cartesianProduct, defaultOptions, models } from './common'
import StarkwareApp from '@zondax/ledger-starkware-app'

import { ec as stark_ec } from 'starknet'

describe.each(models)('Standard', function(m) {
  test('can start and stop container', async function() {
    const sim = new Zemu(m.path)
    try {
      await sim.start({ ...defaultOptions, model: m.name })
    } finally {
      await sim.close()
    }
  })

  test('main menu', async function() {
    const sim = new Zemu(m.path)
    try {
      await sim.start({ ...defaultOptions, model: m.name })
      await sim.navigateAndCompareSnapshots('.', `${m.prefix.toLowerCase()}-mainmenu`, [1, 0, 0, 4, -5])
    } finally {
      await sim.close()
    }
  })

  test('get app version', async function() {
    const sim = new Zemu(m.path)
    try {
      await sim.start({ ...defaultOptions, model: m.name })
      const app = new StarkwareApp(sim.getTransport())
      const resp = await app.getVersion()

      console.log(resp)

      expect(resp.returnCode).toEqual(0x9000)
      expect(resp.errorMessage).toEqual('No errors')
      expect(resp).toHaveProperty('testMode')
      expect(resp).toHaveProperty('major')
      expect(resp).toHaveProperty('minor')
      expect(resp).toHaveProperty('patch')
    } finally {
      await sim.close()
    }
  })
})

describe.skip.each(models)('Standard [%s] - pubkey', function(m) {
  test(
    'get pubkey and addr',
    async function() {
      const sim = new Zemu(m.path)
      try {
        await sim.start({ ...defaultOptions, model: m.name })
        const app = new StarkwareApp(sim.getTransport())
        const resp = await app.getPubKey(APP_DERIVATION)

        console.log(resp, m.name)

        expect(resp.returnCode).toEqual(0x9000)
        expect(resp.errorMessage).toEqual('No errors')
        expect(resp).toHaveProperty('publicKey')
      } finally {
        await sim.close()
      }
    },
  )
})

const SIGN_TEST_DATA = [
  {
    name: 'blind sign',
    nav: { s: [2, 0], x: [3, 0] },
    op: Buffer.from('hello@zondax.ch'),
  },
]

describe.skip.each(models)('Standard [%s]; sign', function(m) {
  test.each(SIGN_TEST_DATA)('sign operation', async function(data) {
    const sim = new Zemu(m.path)
    try {
      await sim.start({ ...defaultOptions, model: m.name })
      const app = new StarkwareApp(sim.getTransport())
      const msg = data.op
      const respReq = app.sign(APP_DERIVATION, msg)

      await sim.waitUntilScreenIsNot(sim.getMainMenuSnapshot(), 20000)

      const navigation = m.name == 'nanox' ? data.nav.x : data.nav.s
      await sim.navigateAndCompareSnapshots('.', `${m.prefix.toLowerCase()}-sign-${data.name}`, navigation)

      const resp = await respReq

      console.log(resp, m.name, data.name)

      expect(resp.returnCode).toEqual(0x9000)
      expect(resp.errorMessage).toEqual('No errors')
      expect(resp).toHaveProperty('hash')
      expect(resp).toHaveProperty('r')
      expect(resp).toHaveProperty('s')

      const resp_addr = await app.getPubKey(APP_DERIVATION)

      let signatureOK = true
      const keypair = stark_ec.getKeyPairFromPublicKey('0x' + resp_addr.publicKey.toString('hex'));

      signatureOK = stark_ec.verify(keypair, '0x' + resp.hash, ['0x' + resp.r.toString('hex'), '0x' + resp.s.toString('hex')]);

      expect(signatureOK).toEqual(true)
    } finally {
      await sim.close()
    }
  })
})

const FELT_TEST_DATA = [
  {
    name: 'random data',
    nav: { s: [2, 0], x: [3, 0] },
    felt: Buffer.alloc(32, 0x01), //no particular significance
  }
]

describe.skip.each(models)('Standard [%s]; felt sign', function(m) {
  test.each(FELT_TEST_DATA)('sign felt', async function(data) {
    const sim = new Zemu(m.path)
    try {
      await sim.start({ ...defaultOptions, model: m.name })
      const app = new StarkwareApp(sim.getTransport())
      const msg = data.felt
      const respReq = app.signFelt(APP_DERIVATION, msg)

      await sim.waitUntilScreenIsNot(sim.getMainMenuSnapshot(), 20000)

      const navigation = m.name == 'nanox' ? data.nav.x : data.nav.s
      await sim.navigateAndCompareSnapshots('.', `${m.prefix.toLowerCase()}-sign-felt-${data.name}`, navigation)

      const resp = await respReq

      console.log(resp, m.name, data.name)

      expect(resp.returnCode).toEqual(0x9000)
      expect(resp.errorMessage).toEqual('No errors')
      expect(resp).toHaveProperty('r')
      expect(resp).toHaveProperty('s')

      const resp_addr = await app.getPubKey(APP_DERIVATION)

      let signatureOK = true
      const keypair = stark_ec.getKeyPairFromPublicKey('0x' + resp_addr.publicKey.toString('hex'));

      signatureOK = stark_ec.verify(keypair, '0x' + msg.toString('hex'), ['0x' + resp.r.toString('hex'), '0x' + resp.s.toString('hex')]);

      expect(signatureOK).toEqual(true)
    } finally {
      await sim.close()
    }
  })
})
