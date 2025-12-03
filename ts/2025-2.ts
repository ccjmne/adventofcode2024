// https://adventofcode.com/2025/day/2
import { readFile } from 'node:fs/promises'
const __dirname = import.meta.url.substring('file://'.length, import.meta.url.lastIndexOf('/'))
console.log(String(await readFile(__dirname + '/2025-2.txt')).trim()
  .split(/,/).map(range => range.split(/-/))
  .flatMap(([lo, hi]) => range(lo.length, hi.length).flatMap((len, i, { length }) => {
    const lol = ((i === 0          && lo.length === len) ? lo.substring(lo.length - len) : 1 + '0'.repeat(len - 1))
    const hil = ((i === length - 1 && hi.length === len) ? hi.substring(hi.length - len) : '9'.repeat(len))
    return by(2) // comment out for step 2
    return uniq(range(2, len).flatMap(by))
    function by(n: number): number[] {
      if (lol.length % n !== 0) return []
      return range(lol.substring(0, len / n), hil.substring(0, len / n))
        .map(m => Number(String(m).repeat(n)))
        .filter(x => x >= lo && x <= hi)
    }
  }))
  .reduce((sum, n) => sum + n, 0))

function range(lo: any, hi: any): number[] {
  return Array.from({ length: hi - lo + 1}).map((_, i) => Number(lo) + i)
}

function uniq(arr: number[]): number[] {
  return Object.keys(arr.reduce((mem, v) => Object.assign(mem, { [v]: true }), {})).map(Number)
}
