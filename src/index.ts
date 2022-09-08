import { Hono } from 'hono'
import { poweredBy } from 'hono/powered-by'

const app = new Hono()

// Builtin middleware
app.use('*', poweredBy())

// Custom middleware
app.use('*', async (c, next) => {
  await next()
  c.header('X-message', 'Scan da wordpress')
})

// Routing
app.get('/', (c) => c.html('<h1>Hello WP!</h1>'))
app.get('/:url', async (c) => {
  const url = decodeURIComponent(decodeURIComponent(c.req.param('url')))
  const res = await fetch(url)
  return c.html(await res.text())
})

export default app
