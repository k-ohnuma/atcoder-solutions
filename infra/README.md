# Infra

Infrastructure for the backend API is managed with CDK Terrain (`cdktn`).

## Prerequisites

Run commands from `infra/`.

```sh
pnpm install
```

Use the infra Nix shell when running deployment commands.

```sh
nix develop .#infra
```

## Generated Providers

Provider bindings are generated into `.gen/` by CDK Terrain.

```sh
pnpm run get
```

`.gen/` is intentionally not committed. CI also runs `pnpm run get` before TypeScript compilation.

## Checks

```sh
pnpm run ci
```

This runs:

- `cdktn get`
- TypeScript compile
- Biome check
- Biome format check
- Biome lint

Root `just before-push` also runs this infra CI.

## Secrets

Secret Manager secret metadata is managed by infra code, but secret values are not stored in Terraform state.

Current secrets:

| Secret ID | Cloud Run env | Purpose |
|---|---|---|
| `app-database-url-{env}` | `APP_DATABASE_URL` | Backend database URL |
| `turnstile-secret-key-{env}` | `TURNSTILE_SECRET_KEY` | Cloudflare Turnstile siteverify secret |

Add a new database URL version:

```sh
just setsecret "postgresql://..." dev atcoder-solutions
```

Add a new Turnstile secret key version:

```sh
just setturnstilesecret "TURNSTILE_SECRET_KEY_VALUE" dev atcoder-solutions
```

Secret versions are selected by Terraform variables:

- `TF_VAR_app_database_url_secret_version`
- `TF_VAR_turnstile_secret_key_version`

Defaults are defined in `config/config.ts`.

## First Deploy Notes

The stack creates Secret Manager secret metadata, but it does not create secret versions.

For a fresh environment, deploy may need to be staged:

1. Apply once so Secret Manager secrets are created.
2. Add secret values with `just setsecret` and `just setturnstilesecret`.
3. Deploy the Cloud Run service with the intended secret versions.

## Deploy

Build and push the backend image:

```sh
just push <tag> dev
```

Preview infra changes:

```sh
just cdkdiff <tag> dev
```

Deploy:

```sh
just cdkdeploy <tag> dev
```

The deploy commands pass:

- `CDKTF_ENV`
- `TF_VAR_api_container_image`
- `TF_VAR_project_id`

Override other Terraform variables by exporting `TF_VAR_*` values before running the command.

