addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
  const url = new URL(request.url);

  switch (url.pathname) {
    case '/':
        const { greet } = wasm_bindgen;
        await wasm_bindgen(wasm)
        const greeting = greet()
        return new Response(greeting, {status: 200})
        break;
    case '/prototype':
        const { prototype } = wasm_bindgen;
        await wasm_bindgen(wasm)
        const game = prototype()
        return new Response(
          JSON.stringify({
              game
          }),
          { headers: { 'Content-type': 'application/json' } },
          )
        break;
      default:
        return new Response('Not found', { status: 404 });
    }
}
