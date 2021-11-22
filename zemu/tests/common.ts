import { DEFAULT_START_OPTIONS, DeviceModel } from '@zondax/zemu'
import { Curve } from '@zondax/ledger-template-app'

const Resolve = require('path').resolve

export const APP_SEED = 'equip will roof matter pink blind book anxiety banner elbow sun young'

const APP_PATH_S = Resolve('../build/output/app_s.elf')
const APP_PATH_X = Resolve('../build/output/app_x.elf')

export const models: DeviceModel[] = [
  { name: 'nanos', prefix: 'S', path: APP_PATH_S },
  { name: 'nanox', prefix: 'X', path: APP_PATH_X },
]

export const curves: Curve[] = [Curve.Ed25519]

export const defaultOptions = {
  ...DEFAULT_START_OPTIONS,
  logging: true,
  custom: `-s "${APP_SEED}"`,
}

export const APP_DERIVATION = "m/44'/0'/0'/0'"

type MapCartesian<T extends any[][]> = {
  [P in keyof T]: T[P] extends Array<infer U> ? U : never
}

export const cartesianProduct = <T extends any[][]>(...arr: T): MapCartesian<T>[] =>
  arr.reduce((a, b) => a.flatMap(c => b.map(d => [...c, d])), [[]]) as MapCartesian<T>[]
