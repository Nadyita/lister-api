# Ultimate Lister API (Rust)

A RESTful API for shopping and wishlist management, written in Rust with Axum and SQLx.
`
## Features

- ✅ **RESTful API Design** with proper HTTP methods (GET, POST, PUT, DELETE)
- ✅ **Type-safe** database queries with SQLx
- ✅ **Async/await** with Tokio runtime
- ✅ **CORS enabled** for cross-origin requests
- ✅ **Structured logging** with tracing
- ✅ **Connection pooling** for PostgreSQL
- ✅ **Comprehensive CRUD** for Lists, Items, and Categories

## Tech Stack

- **Framework:** [Axum](https://github.com/tokio-rs/axum) 0.7
- **Database:** PostgreSQL with [SQLx](https://github.com/launchbadge/sqlx)
- **Runtime:** [Tokio](https://tokio.rs/)
- **Serialization:** [Serde](https://serde.rs/)

## Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- PostgreSQL 14+
- Database already initialized with the schema (from `../dump.sql`)

## Setup

### 1. Copy environment configuration

```bash
cp .env.example .env
```

Edit `.env` with your database credentials:

```env
DATABASE_URL=postgresql://app:password@127.0.0.1:5432/postgres
HOST=127.0.0.1
PORT=8080
RUST_LOG=info,ultimatelister_api=debug
```

### 2. Build the project

```bash
cargo build --release
```

### 3. Run the server

```bash
cargo run --release
```

Or for development with auto-reload:

```bash
cargo watch -x run
```

The server will start on `http://127.0.0.1:8080`

## API Endpoints

### Lists

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/lists` | Get all lists with item counts |
| `GET` | `/api/lists/:id` | Get a specific list |
| `POST` | `/api/lists` | Create a new list |
| `PUT` | `/api/lists/:id` | Update list name |
| `DELETE` | `/api/lists/:id` | Delete a list |

### Items

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/lists/:list_id/items` | Get all items in a list |
| `GET` | `/api/items/:id` | Get a specific item |
| `POST` | `/api/lists/:list_id/items` | Create a new item |
| `PUT` | `/api/items/:id` | Update an item |
| `PATCH` | `/api/items/:id/toggle` | Toggle item's inCart status |
| `DELETE` | `/api/items/:id` | Delete an item |

### Categories

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/categories` | Get all categories |
| `GET` | `/api/categories/:id` | Get a specific category |
| `POST` | `/api/categories` | Create a new category |
| `PUT` | `/api/categories/:id` | Update a category |
| `DELETE` | `/api/categories/:id` | Delete a category |

### Search

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/search` | Get all item names for autocomplete |
| `GET` | `/api/search/category-mappings` | Get product→category mappings |

### Health Check

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/health` | Health check endpoint |

## Example Requests

### Create a new list

```bash
curl -X POST http://localhost:8080/api/lists \
  -H "Content-Type: application/json" \
  -d '{"name": "Weekend Shopping"}'
```

### Add an item to a list

```bash
curl -X POST http://localhost:8080/api/lists/1/items \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Milk",
    "amount": 2,
    "amountUnit": "l",
    "category": "Kühlregal"
  }'
```

### Update an item

```bash
curl -X PUT http://localhost:8080/api/items/123 \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Whole Milk",
    "amount": 3,
    "amountUnit": "l"
  }'
```

### Toggle item in cart

```bash
curl -X PATCH http://localhost:8080/api/items/123/toggle
```

### Delete an item

```bash
curl -X DELETE http://localhost:8080/api/items/123
```

## Response Formats

### Success Responses

**List:**
```json
{
  "id": 1,
  "name": "Supermarkt"
}
```

**Item:**
```json
{
  "id": 123,
  "name": "Milk",
  "amount": 2.0,
  "amountUnit": "l",
  "inCart": false,
  "list": 1,
  "category": "Kühlregal"
}
```

**Category:**
```json
{
  "id": 7,
  "name": "Kühlregal"
}
```

### Error Responses

```json
{
  "error": "Resource not found"
}
```

HTTP Status Codes:
- `200 OK` - Success
- `201 Created` - Resource created
- `204 No Content` - Success with no body (deletes)
- `400 Bad Request` - Invalid input
- `404 Not Found` - Resource not found
- `500 Internal Server Error` - Server error

## Database Schema

The API uses the existing PostgreSQL schema with 4 tables:

- **lists** - Shopping/wish lists
- **items** - Items in lists
- **categories** - Product categories
- **names** - Item name autocomplete with usage counts

See `../dump.sql` for the complete schema.

## Development

### Run tests

```bash
cargo test
```

### Format code

```bash
cargo fmt
```

### Lint code

```bash
cargo clippy
```

### Watch mode (auto-reload on changes)

```bash
# Install cargo-watch first
cargo install cargo-watch

# Run in watch mode
cargo watch -x run
```

## Building a Static Binary

For production deployments, you can create a fully statically linked binary that has no dependencies and can run on any Linux system.

### Prerequisites

Install the musl target:

```bash
rustup target add x86_64-unknown-linux-musl
```

On Fedora/RHEL, install musl-gcc:

```bash
sudo dnf install musl-gcc musl-devel
```

On Debian/Ubuntu:

```bash
sudo apt install musl-tools
```

### Build Static Binary

```bash
cargo build --release --target x86_64-unknown-linux-musl
```

The static binary will be at: `target/x86_64-unknown-linux-musl/release/ultimatelister-api`

### Verify Static Linking

```bash
ldd target/x86_64-unknown-linux-musl/release/ultimatelister-api
```

Expected output: `not a dynamic executable`

### Alternative: Using Cross

If you encounter issues with musl-gcc, use the `cross` tool:

```bash
cargo install cross
cross build --release --target x86_64-unknown-linux-musl
```

### Benefits

- **Portable**: Runs on any Linux distro without dependencies
- **Minimal**: Final binary is ~15-25 MB
- **Secure**: No shared library vulnerabilities
- **Easy deployment**: Just copy one file

## Deployment

### Using Docker (Automated)

**GitHub Actions automatically builds and pushes Docker images** on every push to main or version tag.

The project uses a minimal `FROM scratch` Docker image (~20 MB) containing only the statically linked binary.

#### Pull from GitHub Container Registry:

```bash
docker pull ghcr.io/Nadyita/lister-api:latest
docker run -p 8080:8080 \
  -e DATABASE_URL=postgresql://... \
  -e HOST=0.0.0.0 \
  -e PORT=8080 \
  ghcr.io/Nadyita/lister-api:latest
```

#### Build locally:

```bash
docker build -t ultimatelister-api .
docker run -p 8080:8080 --env-file .env ultimatelister-api
```

#### Available tags:

- `latest` - Latest build from main branch
- `v1.0.0`, `v1.0`, `v1` - Semantic version tags
- `main-<sha>` - Specific commit from main branch

### Using systemd

Create `/etc/systemd/system/ultimatelister-api.service`:

```ini
[Unit]
Description=Ultimate Lister API
After=network.target postgresql.service

[Service]
Type=simple
User=www-data
WorkingDirectory=/opt/ultimatelister-api
EnvironmentFile=/opt/ultimatelister-api/.env
ExecStart=/opt/ultimatelister-api/ultimatelister-api
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl enable ultimatelister-api
sudo systemctl start ultimatelister-api
```

## Differences from Node.js API

This Rust implementation provides several improvements over the original Node.js API:

1. **RESTful Design:**
   - Proper HTTP methods (PUT/DELETE instead of POST for everything)
   - Resource-based URLs (`/api/lists/1` instead of query parameters)
   - Consistent naming conventions

2. **Type Safety:**
   - Compile-time type checking
   - No runtime type errors
   - SQL queries validated at compile time (with sqlx prepare)

3. **Performance:**
   - Faster response times
   - Lower memory usage
   - Better concurrent request handling

4. **New Features:**
   - List renaming (PUT /api/lists/:id)
   - Full CRUD for categories
   - Health check endpoint
   - Structured error responses

## License

This project is licensed under the GNU Affero General Public License v3.0 (AGPL-3.0).

See the [LICENSE](LICENSE) file for the full license text.

### What does this mean?

- ✅ You can use, modify, and distribute this software
- ✅ You must provide source code when distributing
- ✅ **If you modify and run this on a server**, you must make your modified source code available to users
- ✅ Derivative works must also be licensed under AGPL-3.0

The AGPL is specifically designed for network services - if you host a modified version of this API, you must provide the source code to your users.

## Author

Mark Reidel