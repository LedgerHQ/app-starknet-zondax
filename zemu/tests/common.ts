import Zemu, { DEFAULT_START_OPTIONS, DeviceModel } from '@zondax/zemu'

const Resolve = require('path').resolve

export const APP_SEED = 'equip will roof matter pink blind book anxiety banner elbow sun young'

const APP_PATH_S = Resolve('../build/output/app_s.elf')
const APP_PATH_X = Resolve('../build/output/app_x.elf')
const APP_PATH_SP = Resolve('../build/output/app_sp.elf')

export const models: DeviceModel[] = [
  { name: 'nanos', prefix: 'S', path: APP_PATH_S },
  { name: 'nanox', prefix: 'X', path: APP_PATH_X },
  { name: 'nanosp', prefix: 'SP', path: APP_PATH_SP },
]

export const defaultOptions = {
  ...DEFAULT_START_OPTIONS,
  logging: true,
  custom: `-s "${APP_SEED}"`,
  startText: 'DO NOT USE',
}

export const APP_DERIVATION = "m/2645'/579218131'/0'/0'"

type MapCartesian<T extends any[][]> = {
  [P in keyof T]: T[P] extends Array<infer U> ? U : never
}

export const cartesianProduct = <T extends any[][]>(...arr: T): MapCartesian<T>[] =>
  arr.reduce((a, b) => a.flatMap(c => b.map(d => [...c, d])), [[]]) as MapCartesian<T>[]

export async function enableBlindSigning(sim: Zemu, testcase: string): Promise<void> {
  await sim.navigateAndCompareUntilText('.', testcase, "Signing mode");
}
