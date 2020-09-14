import express from 'express'

const app = express()
const PORT = 8080

app.get('/api/hello', (req, res) => {
  const start = process.hrtime()
  const end = process.hrtime(start)
  res.json({
    result: "Hello world!",
    performance: `${end[0]}s and ${end[1]}ns`
  })
})

app.get('/api/fib', (req, res) => {
  let f0 = 0n
  let f1 = 1n
  const start = process.hrtime()

  const number = BigInt(req.query.n)

  for (let i = 0; i <= number; i++) {
    let f2 = f0 + f1
    f0 = f1
    f1 = f2
  }

  const end = process.hrtime(start)

  res.json({
    result: f0.toString(),
    performance: `${end[0]}s and ${end[1]}ns`
  })
})

app.get('/api/dumb-sum', (req, res) => {
  const start = process.hrtime()
  const number = BigInt(req.query.n)
  const arr = []
  for (let i = 0n; i <= number; i++) {
    let sum = 0n
    for (let j = 0n; j <= i; j++) {
      sum += j
    }
    arr.push(sum)
  }

  const end = process.hrtime(start)

  res.json({
    result: arr[arr.length - 1].toString(),
    performance: `${end[0]}s and ${end[1]}ns`
  })
})

app.listen(PORT, () => {
  console.log(`Servers started at http://localhost:${PORT}`)
})