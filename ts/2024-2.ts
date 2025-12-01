import { readFile } from 'fs/promises'
const __dirname = import.meta.url.substring('file://'.length, import.meta.url.lastIndexOf('/'))
const input = String(await readFile(__dirname + '/2.txt'))
  .trim().split(/\n/g).map(line => line.match(/\d+/g).map(Number))

console.log(input
  .map(report => [report]
    // comment out for part I
    .concat(Array.from(report, (_, i) => report.slice(0, i).concat(report.slice(i + 1))))
  )
  .map(perms => perms.map(report => report.slice(1).map((r, i) => [report[i], r])
    .map(([l, r]) => ({ delta: Math.abs(r - l), dir: Math.sign(r - l) }))
    .reduce(
      ({ prev, ok }, { delta, dir }) => ({ prev: dir, ok: ok && (!prev || prev === dir) && (0 < delta && delta <= 3) }),
      { prev: 0, ok: true })))
  .filter(perms => perms.some(({ ok }) => ok))
  .length
)
