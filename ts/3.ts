import { readFile } from 'fs/promises'

readFile(__dirname + '/3-input.txt').then(real => console.log([...(String(real)
  .replace(/don't\(\).*?(?:do\(\)|$)/sg, '')) // comment out for part I
  .matchAll(/mul\((?<l>\d+),(?<r>\d+)\)/g)
].reduce((sum, { groups: { l, r } }) => sum + l * r, 0)))
