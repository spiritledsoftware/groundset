# Groundset

Groundset is the working name for open, domain-neutral knowledge infrastructure and its reference service. It preserves versioned source material, records attributed interpretations, and gives AI systems small evidence packets instead of unfiltered retrieval dumps.

The current design is documented in [`docs/data-model.md`](docs/data-model.md) and [`docs/product-boundary.md`](docs/product-boundary.md).

## Runtime

The reference service runs as a TypeScript Cloudflare Worker. TypeScript is Cloudflare's first-class language; Rust/Wasm remains available for isolated compute-heavy modules when profiling justifies it.

```sh
pnpm install
pnpm typegen
pnpm check
pnpm dev
```

No persistence product has been selected yet. R2, D1, Durable Objects, and Queues will be added only when the evidence workflow requires them.

## Research benchmarks

The benchmarks currently target version-sensitive Axum 0.8 tasks. Each has two conditions:

- `no-evidence`: the agent receives the task and repository only
- `oracle`: the agent also receives a hand-curated packet from Axum's official documentation

Available tasks:

- `axum-user-lookup`
- `axum-custom-path-rejection`
- `axum-websocket-message`

## Run the first benchmark

Prepare a clean candidate repository:

```sh
python3 benchmark.py prepare axum-user-lookup no-evidence /tmp/axum-no-evidence
python3 benchmark.py prepare axum-user-lookup oracle /tmp/axum-oracle
```

Give the agent `PROMPT.md` and let it edit that candidate repository. Then run the hidden checks:

```sh
python3 benchmark.py verify axum-user-lookup /tmp/axum-no-evidence
python3 benchmark.py verify axum-user-lookup /tmp/axum-oracle
```

`verify` evaluates a temporary copy, so it does not add hidden tests to the candidate.

Trial reports live in [`results/`](results/).

## Protocols

- `agentic`: the model may inspect the candidate and run Cargo checks before hidden verification
- `one-shot`: the task, starter source, and manifest are supplied up front; the model may only edit the source before hidden verification

The one-shot protocol measures the evidence packet's effect without compiler repair or dependency-source inspection. Agentic runs measure whether evidence reduces tool use and repair work.
