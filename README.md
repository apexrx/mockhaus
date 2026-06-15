# Mockhaus

*Schema-first API mocking. Self-hosted.*

---

## What is Mockhaus?

Mockhaus is a self-hosted mock API server. You define your endpoints — method, path, status code, response body — and Mockhaus makes them real. Point your frontend at it and it behaves like a live backend. No cloud account, no shared environment, no waiting on someone else's server to be ready.

The design premise is that a mock server should be as close to the real thing as possible without any of the volatility. Mockhaus runs locally, persists its schema to SQLite, and routes requests through a dynamic matching engine that handles parameterized paths the same way a real router would. Every request to a mock endpoint is logged and viewable in a browser UI served by the same binary as everything else.

There is no separate frontend process, no Node runtime, no build step. `cargo run` starts the server. `docker-compose up` starts the whole stack. The UI is Askama-rendered HTML with vanilla JS modules — no framework, no bundler, nothing to install.

---

## Running Mockhaus

### With Docker (recommended)

```bash
git clone https://github.com/apexrx/mockhaus
cd mockhaus
docker compose up --build -d
```

The server starts at `http://localhost:7070`. The SQLite database is stored on a named volume and persists across container restarts.

If you want to inspect or export the database, it lives at `/data/mockhaus.db` inside the container.

### Locally

```bash
git clone https://github.com/apexrx/mockhaus
cd mockhaus/server
cargo run
```

This requires a Rust toolchain (stable). The server runs migrations automatically on startup, so no manual setup is needed. The database is created at the path in your `DATABASE_URL` environment variable, defaulting to `sqlite:mockhaus.db` in the current directory.

---

## Using Mockhaus

### Projects

A project is a named collection of endpoints that all share a URL prefix. When you create a project, Mockhaus assigns it a UUID. Every mock request for that project routes through `/{project_id}/{your-path}`.

Projects are created and managed through the browser UI at `localhost:7070`, or directly via the management API:

```bash
curl -X POST localhost:7070/admin/projects \
  -H "Content-Type: application/json" \
  -d '{"name": "my-api"}'
# → {"id": "abc123", "name": "my-api", "created_at": 1718000000}
```

### Endpoints

Each endpoint belongs to a project and defines a single mock route: a method, a path pattern, a response status code, and a response body stored as a JSON string.

Path patterns support named parameters using `:param` syntax. A pattern like `/users/:id` matches `/users/42`, `/users/apex`, or any other single segment — exactly the same semantics as Express or Axum.

```bash
curl -X POST localhost:7070/admin/projects/abc123/endpoints \
  -H "Content-Type: application/json" \
  -d '{
    "method": "GET",
    "path": "/users/:id",
    "status_code": 200,
    "response_body": "{\"id\": \"placeholder\", \"name\": \"Test User\"}"
  }'
```

Once created, the endpoint is live immediately:

```bash
curl localhost:7070/abc123/users/42
# → {"id": "placeholder", "name": "Test User"}
```

### Request Inspector

Every request that hits a mock endpoint is logged — method, path, headers, body, and timestamp. The browser UI at `/projects/:id/inspector` polls the log and displays incoming requests in real time. You can also query logs directly:

```bash
curl localhost:7070/admin/projects/abc123/logs?limit=50
```

Logs can be cleared via `DELETE /admin/projects/:id/logs`.

---

## Management API

The full surface of the management API, for scripting or integration:

```
POST   /admin/projects                          create project
GET    /admin/projects                          list all projects
GET    /admin/projects/:id                      get project with endpoints
DELETE /admin/projects/:id                      delete project (cascades to endpoints and logs)

POST   /admin/projects/:id/endpoints            create endpoint
PUT    /admin/projects/:id/endpoints/:eid       update endpoint
DELETE /admin/projects/:id/endpoints/:eid       delete endpoint

GET    /admin/projects/:id/logs                 list request logs (supports ?limit=N)
DELETE /admin/projects/:id/logs                 clear request logs
```

All request and response bodies are JSON. All timestamps are Unix seconds. The `GET /admin/projects/:id` response nests the endpoint list under an `"endpoints"` key.

---

## Design Principles

**The binary serves everything.** There is no separate frontend server, no static file CDN, no reverse proxy needed in development. The Rust binary handles the management API, the mock routing engine, the Askama-rendered HTML, and the static JS and CSS assets. One process, one port.

**Schema changes take effect immediately.** The routing engine queries SQLite on every incoming request rather than caching endpoints in memory. This means you can update a response body in the UI and the next curl will reflect it — no reload, no restart, no cache invalidation to worry about.

**Routing is exact where it needs to be, flexible where it doesn't.** Method matching is strict. Path matching uses segment-by-segment comparison: literal segments must match exactly, `:param` segments match any single segment and capture the value. A `GET` pattern does not match a `POST` request. An unregistered path returns 404.

**Logs are non-blocking.** Request logging is handled by middleware that buffers the body, writes to SQLite asynchronously, and passes the request to the mock handler without waiting. Logging latency never adds to mock response latency.

---

## Project Structure

```
mockhaus/
├── server/
│   ├── src/
│   │   ├── main.rs
│   │   ├── db/
│   │   ├── router/          # dynamic routing engine
│   │   ├── logger/          # request logging middleware
│   │   ├── api/             # management API handlers
│   │   └── templates/       # Askama templates
│   ├── static/
│   │   ├── app.css
│   │   └── js/
│   │       ├── api/         # fetch wrappers
│   │       ├── views/       # page-level controllers
│   │       ├── components/  # toast, modal, endpoint-list
│   │       └── utils/       # dom, time, clipboard, state
│   ├── Cargo.toml
│   └── migrations/
├── docker-compose.yml
└── README.md
```

The frontend uses native ES modules loaded directly from Askama templates. There is no bundler, transpiler, or frontend build step. Client-side state and API interactions are implemented with small vanilla JavaScript modules.

---

## License

MIT
