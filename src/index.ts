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

let versions = []
app.get('/:url', async (c) => {
  const url = decodeURIComponent(decodeURIComponent(c.req.param('url')))
  const res = await fetch(url)
  const not_used = await new HTMLRewriter()
    .on('script', new ScriptTagHandler())
    .on('link', new ScriptTagHandler('href'))
    .transform(res).text();
  return c.html(`Version: ${JSON.stringify(versions)}`)
})

class ScriptTagHandler {
  constructor(attr='src') {
    this.attr = attr
  }
  element(e) {
    const src = e.getAttribute(this.attr)
    versions.push(src)
  }
}

export default app
