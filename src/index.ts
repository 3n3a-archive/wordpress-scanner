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
app.get('/', (c) => {
  return c.html(`
  <body>
    <label for="url">Enter a URL</label>
    <input type="text" id="url" name="url" />
    <button id="button">Scan</button>
    <script>
      let url = document.getElementById("url")
      let btn = document.getElementById("button")
      btn.addEventListener('click', () => {
        window.location = window.location.origin + '/' + encodeURIComponent(url.value)
      })
    </script>
  </body>
  `)
})
app.get('/:url', async (c) => {
  const url = decodeURIComponent(decodeURIComponent(c.req.param('url')))
  const res = await fetch(url)
  return c.html(await res.text())
})

export default app
