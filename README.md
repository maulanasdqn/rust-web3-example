# Solana Voting App

A decentralized voting application built with Rust, Axum, and Solana blockchain. Votes are recorded on-chain via Solana memo transactions, making them transparent and verifiable.

## Features

- Create polls with multiple options
- Cast votes recorded on Solana devnet
- Prevent double voting per wallet address
- View real-time voting results with percentages
- Verify votes on Solana Explorer via transaction signatures

## Architecture

This project follows Clean Architecture principles with a multi-crate workspace structure:

```
                    +------------------+
                    |  voting-server   |
                    |  (Entry Point)   |
                    +--------+---------+
                             |
              +--------------+--------------+
              |                             |
    +---------v---------+         +---------v---------+
    |   voting-polls    |         |  voting-solana    |
    |  (Feature Module) |         | (Blockchain Client)|
    +-------------------+         +-------------------+
              |
    +---------+---------+---------+
    |         |         |         |
+---v---+ +---v---+ +---v---+ +---v---+
| Domain| |  App  | | Infra | | HTTP  |
+-------+ +-------+ +-------+ +-------+
```

### Clean Architecture Layers

- **Domain**: Entities (Poll, Vote) and repository traits - no external dependencies
- **Application**: Use cases (CreatePoll, CastVote, GetResults) - business logic
- **Infrastructure**: Repository implementations, HTTP handlers, DTOs

## Project Structure

```
rust-web3-example/
├── Cargo.toml                          # Workspace configuration
├── voting-server/                      # Application entry point
│   └── src/
│       ├── main.rs                     # Server bootstrap and DI wiring
│       └── config.rs                   # Server configuration
├── voting-errors/                      # Centralized error handling
│   └── src/
│       └── lib.rs                      # AppError enum with IntoResponse
├── voting-types/                       # Shared types
│   └── src/
│       ├── lib.rs
│       └── responses.rs                # SingleResponse, ListResponse
├── voting-solana/                      # Solana blockchain integration
│   └── src/
│       ├── lib.rs
│       ├── client.rs                   # SolanaClient wrapper
│       └── config.rs                   # Solana RPC configuration
└── voting-polls/                       # Polls feature module
    └── src/
        ├── lib.rs
        ├── domain/
        │   ├── poll.rs                 # Poll entity
        │   ├── vote.rs                 # Vote entity
        │   └── repository.rs           # Repository traits
        ├── application/
        │   ├── create_poll.rs          # CreatePoll use case
        │   ├── get_poll.rs             # GetPoll use case
        │   ├── cast_vote.rs            # CastVote use case
        │   └── get_results.rs          # GetResults use case
        └── infrastructure/
            ├── http/
            │   ├── handlers.rs         # HTTP request handlers
            │   ├── routes.rs           # Route definitions
            │   └── dto.rs              # Request/Response DTOs
            └── persistence/
                ├── memory_poll_repository.rs    # In-memory poll storage
                └── solana_vote_repository.rs    # Solana-based vote storage
```

## Prerequisites

- Rust 1.75 or later
- Cargo package manager

## Installation

Clone the repository:

```bash
git clone git@github.com:maulanasdqn/rust-web3-example.git
cd rust-web3-example
```

Build the project:

```bash
cargo build --release
```

## Running the Server

Start the server:

```bash
cargo run -p voting-server
```

The server will start on `http://0.0.0.0:3000` by default.

## API Documentation

Base URL: `http://localhost:3000/api/v1`

### Create Poll

```http
POST /api/v1/polls
Content-Type: application/json

{
  "title": "Best Programming Language",
  "options": ["Rust", "Go", "TypeScript", "Python"]
}
```

Response:

```json
{
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "title": "Best Programming Language",
    "options": [
      { "index": 0, "text": "Rust" },
      { "index": 1, "text": "Go" },
      { "index": 2, "text": "TypeScript" },
      { "index": 3, "text": "Python" }
    ],
    "created_at": "2024-01-15T10:30:00Z"
  },
  "message": "Poll created"
}
```

### List All Polls

```http
GET /api/v1/polls
```

Response:

```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "title": "Best Programming Language",
      "options": [...],
      "created_at": "2024-01-15T10:30:00Z"
    }
  ],
  "count": 1
}
```

### Get Poll by ID

```http
GET /api/v1/polls/{id}
```

### Cast Vote

```http
POST /api/v1/polls/{id}/vote
Content-Type: application/json

{
  "option_index": 0,
  "voter_pubkey": "YourSolanaWalletPublicKey"
}
```

Response:

```json
{
  "data": {
    "poll_id": "550e8400-e29b-41d4-a716-446655440000",
    "option_index": 0,
    "voter_pubkey": "YourSolanaWalletPublicKey",
    "transaction_signature": "5UfDuX..."
  },
  "message": "Vote recorded"
}
```

The `transaction_signature` can be verified on [Solana Explorer](https://explorer.solana.com/?cluster=devnet).

### Get Voting Results

```http
GET /api/v1/polls/{id}/results
```

Response:

```json
{
  "data": {
    "poll": {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "title": "Best Programming Language",
      "options": [...],
      "created_at": "2024-01-15T10:30:00Z"
    },
    "results": [
      { "index": 0, "text": "Rust", "votes": 42, "percentage": 58.33 },
      { "index": 1, "text": "Go", "votes": 15, "percentage": 20.83 },
      { "index": 2, "text": "TypeScript", "votes": 10, "percentage": 13.89 },
      { "index": 3, "text": "Python", "votes": 5, "percentage": 6.94 }
    ],
    "total_votes": 72
  },
  "message": "Results retrieved"
}
```

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `SERVER_HOST` | Server bind address | `0.0.0.0` |
| `SERVER_PORT` | Server port | `3000` |
| `SOLANA_RPC_URL` | Solana RPC endpoint | `https://api.devnet.solana.com` |

Example:

```bash
SERVER_PORT=8080 SOLANA_RPC_URL=https://api.devnet.solana.com cargo run -p voting-server
```

## Tech Stack

- **Rust** - Systems programming language
- **Axum** - Web framework for Rust
- **Tokio** - Async runtime
- **Solana SDK** - Blockchain interaction
- **Serde** - Serialization/deserialization
- **UUID** - Unique identifier generation
- **Chrono** - Date and time handling

## How Solana Integration Works

1. When a user casts a vote, the server creates a memo transaction containing:
   ```json
   {"poll_id": "...", "option": 0, "voter": "..."}
   ```

2. The transaction is signed by the server's keypair and submitted to Solana devnet

3. The transaction signature is returned to the user for verification

4. Votes are cached locally for fast retrieval while remaining verifiable on-chain

## Limitations

- Poll metadata is stored in-memory (resets on server restart)
- Server keypair is auto-generated (requires SOL airdrop for transactions)
- Single server instance (no horizontal scaling without shared state)

For production use, consider:
- Persistent database for poll storage
- Dedicated funded Solana keypair
- Redis or similar for distributed caching

## License

MIT
