// https://adventofcode.com/2024/day/5
import { readFile } from 'fs/promises'
const __dirname = import.meta.url.substring('file://'.length, import.meta.url.lastIndexOf('/'))

const [rules, updates] = String(await readFile(__dirname + '/5.txt')).trim()
  .split(/\n\n/g).map(section => section.split(/\n/g).map(line => line.split(/[|,]/g).map(Number))) as [[number, number][], number[][]]

const [ko, ok] = updates.reduce((bins, upd) => (bins[+ordered(upd)].push(upd), bins), [[], []] as [number[][], number[][]])

console.log(ok
  .reduce((total, update) => total + update[Math.floor(update.length / 2)], 0))

console.log(ko
  .map(update => update.toSorted((a, b) => correct(a, b) ? 1 : -1))
  .reduce((total, update) => total + update[Math.floor(update.length / 2)], 0))

function correct(a: number, b: number): boolean {
  return rules.filter(rule => rule.includes(a) && rule.includes(b)).every(([l, r]) => l === a && r === b)
}

function ordered(update: number[]): boolean {
  return update.every((page, i) => [...new Array(update.length - i - 1).keys()].every(j => correct(page, update[i + j + 1])))
}
