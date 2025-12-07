// https://adventofcode.com/2025/day/6
import { readFile } from 'node:fs/promises'
const __dirname = import.meta.url.substring('file://'.length, import.meta.url.lastIndexOf('/'))
const data = String(await readFile(__dirname + '/2025-6.txt')).trimEnd().split(/\n/)
const ops = data.pop().match(/\S/g)
const d1 = data.map(l => l.match(/\S+/g))
console.log(ops.reduce((total, o, i) => total + d1.map(r => Number(r[i])).reduce((t, d) => o === '*' ? t * d : t + d, +(o === '*')), 0))

const d2 = transpose(data)
console.log(ops.reduce((total, o, i) => total + d2[i].reduce((t, d) => o === '*' ? t * d : t + d, +(o === '*')), 0))

function transpose(str: string[]) {
  return [...str[0]].map((_, i) => Number(str.reduce((ss, s) => ss + s[i], '')))
    .join(',').split(',0,').map(r => r.split(',').map(Number)) // yuck, but heh
}
