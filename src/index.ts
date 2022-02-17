import { Hono } from 'hono'
import { poweredBy } from 'hono/powered-by'

const app = new Hono()

app.use('*', poweredBy())
app.get('/', (c) => c.html('<h1>Hello Hono!</h1>'))

app.fire()
