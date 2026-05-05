# receipes

Ready to go templates/starter-kits for commonly used tech-stacks. Just `bunx degit spinspire/recipes/something myprj`

_Note 1: I'm trying to keep this README short. For a more detailed explanation, see [this article](https://spinspire.com/article/project-recipes)._

_Note 2: These templates have features that you probably don't need or use. So remember trim down whatever you borrow. The goal is to avoid reinventing the wheel._

- `docker-compose`
  - `entrypoint.sh` script to initialize a container before use.
  - `traefik` reverse proxy `labels`, router rules, and `networks` provided so that your container can receive HTTP requests.
  - Main `docker-compose.yml` file + `.env` and `docker-compose.override.yml` files.
- `sk`: Svelte-Kit
  - Static frontend with `adapter-static`, SSR optional
  - HMR for live dev
  - Vite proxy to backend APIs
  - Component testing route at /components (works standalone and can be pulled into the route folder of any sveltekit project)
- `pb`: PocketBase / Go
  - Live dev with `modd`
  - Go ↔ TypeScript type generation with `tygo`
  - PocketBase ↔ TypeScript type generation with `pocketbase-typegen`
  - SPA mode serving `../sk/build`
  - JS migrations
- `py`: Python / FastAPI
  - Configurable prefix routing (e.g. `/apy`)
  - Live dev with `--reload`
- `rs`: Rust / Axum
  - REST API with SQLite backend
  - Live View support (axum-live-view)
  - File upload handling
  - Serves static frontend from `fe/build` with SPA fallback
- `mb`: Metabase

- `ai`: OpenCode agent
  - Custom tools in `.opencode/tools/` (sql.ts for PostgreSQL)
  - Custom skills in `.agents/skills/` (git-commit for conventional commits)

Coming soon ...

- `dr`: Drupal
- `ng`: Nginx
- `pg`: PostGres
- `my`: MySQL/MariaDB

Read the README files the respective folders to understand the details of a specific stack component.

# How to use

Basically use [`degit`](https://github.com/Rich-Harris/degit).

For example, use a specific template ...

- `bunx degit spinspire/receipes/sk myprj/sk`
- `bunx degit spinspire/receipes/pb myprj/pb`
- `bunx degit spinspire/receipes/rs myprj/rs`
- `bunx degit spinspire/receipes/mb myprj/mb`
- `bunx degit spinspire/receipes/py myprj/py`

Or the whole project ...

- `bunx degit spinspire/receipes myprj`