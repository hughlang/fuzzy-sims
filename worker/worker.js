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
    case '/slots':
        const { slots } = wasm_bindgen;
        await wasm_bindgen(wasm)
        const nums = slots()
        return new Response(nums, {status: 200})
        // return new Response(
        //   JSON.stringify({
        //       nums
        //   }),
        //   { headers: { 'Content-type': 'application/json' } },
        //   )
        break;
      default:
        return new Response('Not found', { status: 404 });
    }
}
