// dev server
// serve static file
Bun.serve({
  port: 1949,
  async fetch(req) {
    const path = new URL(req.url).pathname;
    const file = Bun.file(path == "/" ? "html-book/index.html" : "html-book" + path);
    return new Response(file);
  },
});

process.on("SIGINT", function() {
  // console.log( "\nGracefully shutting down from SIGINT (Ctrl-C)" );
  // some other closing procedures could go here
  process.exit(0);
});
