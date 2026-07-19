# backend

backend for atcoder-solutions

## Local Commands

Run from `backend/`.

```bash
just compose-up-db
just migrate
just run
just test
```

The local PostgreSQL container uses a Docker named volume. `docker compose down` keeps data, while `docker compose down -v` removes it.

## Response Format

Successful responses are wrapped as:

```json
{
  "ok": true,
  "status": 200,
  "data": {}
}
```

Error responses are wrapped as:

```json
{
  "ok": false,
  "status": 400,
  "error": "Bad Request: ...",
  "errorCode": "BAD_REQUEST"
}
```

## Authentication

Authenticated endpoints require:

```http
Authorization: Bearer <Firebase ID token>
```

Auth modes:

- `Public`: no authentication required.
- `VerifiedUser`: verifies the Firebase ID token only. Used before the app user row may exist.
- `AuthUser`: verifies the Firebase ID token and rejects revoked tokens.

## API Paths

### Health

| Method | Path | Auth | Query | Body | Description |
|---|---|---|---|---|---|
| `GET` | `/health` | Public | - | - | Process health check. |
| `GET` | `/health/db` | Public | - | - | Database health check. |

### Contests

| Method | Path | Auth | Query | Body | Description |
|---|---|---|---|---|---|
| `GET` | `/contests/{contest_code}/problems` | Public | - | - | Lists problems for one contest. |

### Problems

| Method | Path | Auth | Query | Body | Description |
|---|---|---|---|---|---|
| `GET` | `/problems/{problem_id}` | Public | - | - | Gets one problem. |
| `GET` | `/problems/{problem_id}/solutions` | Public | `sortBy`, `limit` | - | Lists solutions for one problem. |

Notes:

- `sortBy` is optional and must be `latest` or `votes`.
- `limit` on `/problems/{problem_id}/solutions` is optional and must be greater than 0.

### Series

| Method | Path | Auth | Query | Body | Description |
|---|---|---|---|---|---|
| `GET` | `/series/{series}/contests` | Public | - | - | Lists contests in one contest series. |
| `GET` | `/series/{series}/problem-groups` | Public | `q`, `limit`, `offset` | - | Lists grouped problems by contest series. |

Notes:

- `series` must be one of `ABC`, `ARC`, `AGC`, `AHC`, `OTHER`.
- `q` is optional and must be at most 100 characters.
- `offset` is optional and must be at most 5000.

### Users

| Method | Path | Auth | Query | Body | Description |
|---|---|---|---|---|---|
| `POST` | `/users` | VerifiedUser | - | `{ "userName": string }` | Creates the current app user. |
| `GET` | `/users/me` | AuthUser | - | - | Gets the current app user. |
| `DELETE` | `/users/me` | AuthUser | - | - | Deletes the current app user. |
| `POST` | `/users/me/revoke` | AuthUser | - | - | Revokes current user's tokens from this app's perspective. |
| `GET` | `/users/{user_name}/solutions` | Public | `sortBy` | - | Lists solutions written by one user. |

`sortBy` is optional and must be `latest` or `votes`.

### Solutions

| Method | Path | Auth | Query | Body | Description |
|---|---|---|---|---|---|
| `GET` | `/solutions` | Public | `sortBy`, `limit` | - | Lists latest solutions. |
| `POST` | `/solutions` | AuthUser | - | create solution body | Creates a solution. |
| `GET` | `/solutions/{solution_id}` | Public | - | - | Gets one solution. |
| `PATCH` | `/solutions/{solution_id}` | AuthUser | - | update solution body | Updates own solution. |
| `DELETE` | `/solutions/{solution_id}` | AuthUser | - | - | Deletes own solution. |

`GET /solutions` currently accepts only `sortBy=latest` when `sortBy` is provided. `limit` is optional and must be greater than 0.

Create solution body:

```json
{
  "problemId": "abc300_d",
  "title": "解説タイトル",
  "bodyMd": "# 解説",
  "submitUrl": "https://atcoder.jp/contests/abc300/submissions/...",
  "tags": ["math"]
}
```

Update solution body:

```json
{
  "title": "解説タイトル",
  "bodyMd": "# 解説",
  "submitUrl": "https://atcoder.jp/contests/abc300/submissions/...",
  "tags": ["math"]
}
```

### Votes

| Method | Path | Auth | Query | Body | Description |
|---|---|---|---|---|---|
| `GET` | `/solutions/{solution_id}/votes` | Public | - | - | Gets vote count for one solution. |
| `GET` | `/solutions/{solution_id}/votes/me` | AuthUser | - | - | Gets current user's vote status for one solution. |
| `PUT` | `/solutions/{solution_id}/votes/me` | AuthUser | - | - | Votes for one solution as current user. |
| `DELETE` | `/solutions/{solution_id}/votes/me` | AuthUser | - | - | Removes current user's vote from one solution. |

### Comments

| Method | Path | Auth | Query | Body | Description |
|---|---|---|---|---|---|
| `GET` | `/solutions/{solution_id}/comments` | Public | - | - | Lists comments for one solution. |
| `POST` | `/solutions/{solution_id}/comments` | AuthUser | - | `{ "bodyMd": string }` | Creates a comment on one solution. |
| `PATCH` | `/comments/{comment_id}` | AuthUser | - | `{ "bodyMd": string }` | Updates own comment. |
| `DELETE` | `/comments/{comment_id}` | AuthUser | - | - | Deletes own comment. |

`bodyMd` must be a non-empty markdown string.
