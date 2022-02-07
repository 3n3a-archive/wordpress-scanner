import { Hono } from 'hono'

const app = new Hono()
app.get('/', (c) => c.html('<h1>Hello Hono!</h1>'))

app.fire()
