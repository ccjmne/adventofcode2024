// https://adventofcode.com/2024/day/1
import { readFile } from 'fs/promises'
const __dirname = import.meta.url.substring('file://'.length, import.meta.url.lastIndexOf('/'))
const { L, R } = String(await readFile(__dirname + '/1.txt')).trim()
  .split(/\n/).map(line => line.match(/\d+/g)!.map(Number))
  .reduce(({ L, R }, [l, r]) => ({ L: [...L, l], R: [...R, r] }), { L: [] as number[], R: [] as number[] })

L.sort()
R.sort()
console.log(L.reduce((total, l, i) => total + Math.abs(l - R[i]), 0))

const Rbins = R.reduce((bins, r) => ({ ...bins, [r]: (bins[r] ?? 0) + 1 }), {} as { [bin: number]: number })
console.log(L.reduce((total, l) => total + l * (Rbins[l] ?? 0), 0))
