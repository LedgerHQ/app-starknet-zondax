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
import { APP_DERIVATION, cartesianProduct, curves, defaultOptions, models } from './common'
import TemplateApp, { Curve } from '@zondax/ledger-template-app'

const ed25519 = require('ed25519-supercop')

describe.each(models)('Standard', function (m) {
  test('can start and stop container', async function () {
    const sim = new Zemu(m.path)
    try {
      await sim.start({ ...defaultOptions, model: m.name })
    } finally {
      await sim.close()
    }
  })

  test('main menu', async function () {
    const sim = new Zemu(m.path)
    try {
      await sim.start({ ...defaultOptions, model: m.name })
      await sim.navigateAndCompareSnapshots('.', `${m.prefix.toLowerCase()}-mainmenu`, [1, 0, 0, 4, -5])
    } finally {
      await sim.close()
    }
  })

  test('get app version', async function () {
    const sim = new Zemu(m.path)
    try {
      await sim.start({ ...defaultOptions, model: m.name })
      const app = new TemplateApp(sim.getTransport())
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

describe.each(models)('Standard [%s] - pubkey', function (m) {
  test.each(curves)(
    'get pubkey and addr %s',
    async function (curve) {
      const sim = new Zemu(m.path)
      try {
        await sim.start({ ...defaultOptions, model: m.name })
        const app = new TemplateApp(sim.getTransport())
        const resp = await app.getAddressAndPubKey(APP_DERIVATION, curve)

        console.log(resp, m.name)

        expect(resp.returnCode).toEqual(0x9000)
        expect(resp.errorMessage).toEqual('No errors')
        expect(resp).toHaveProperty('publicKey')
        expect(resp).toHaveProperty('hash')
      } finally {
        await sim.close()
      }
    },
  )
})

const SIGN_TEST_DATA = cartesianProduct(curves, [
  {
    name: 'blind sign',
    nav: { s: [2, 0], x: [3, 0] },
    op: Buffer.from('hello@zondax.ch'),
  },
])

describe.each(models)('Standard [%s]; sign', function (m) {
  test.each(SIGN_TEST_DATA)('sign operation', async function (curve, data) {
    const sim = new Zemu(m.path)
    try {
      await sim.start({ ...defaultOptions, model: m.name })
      const app = new TemplateApp(sim.getTransport())
      const msg = data.op
      const respReq = app.sign(APP_DERIVATION, curve, msg)

      await sim.waitUntilScreenIsNot(sim.getMainMenuSnapshot(), 20000)

      const navigation = m.name == 'nanox' ? data.nav.x : data.nav.s
      await sim.navigateAndCompareSnapshots('.', `${m.prefix.toLowerCase()}-sign-${data.name}-${curve}`, navigation)

      const resp = await respReq

      console.log(resp, m.name, data.name, curve)

      expect(resp.returnCode).toEqual(0x9000)
      expect(resp.errorMessage).toEqual('No errors')
      expect(resp).toHaveProperty('hash')
      expect(resp).toHaveProperty('signature')

      const resp_addr = await app.getAddressAndPubKey(APP_DERIVATION, curve)

      let signatureOK = true
      switch (curve) {
        case Curve.Ed25519:
          signatureOK = ed25519.verify(resp.signature, resp.hash, resp_addr.publicKey.slice(1, 33))
          break

        default:
          throw Error('not a valid curve type')
      }
      expect(signatureOK).toEqual(true)
    } finally {
      await sim.close()
    }
  })
})
