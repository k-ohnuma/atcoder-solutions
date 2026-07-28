set shell := ["bash", "-cu"]

default:
  @just --list

taplo:
  taplo format

taplocheck:
  taplo format --check

yamlfmt:
  yamlfmt .

yamlcheck:
  yamlfmt -lint .

lint-ci: taplocheck yamlcheck

frontend-ci:
  cd frontend && just ci

backend-ci:
  cd backend && just ci

infra-ci:
  cd infra && pnpm run ci

before-push: frontend-ci lint-ci backend-ci infra-ci

gitpush: before-push
  git push origin "$(git rev-parse --abbrev-ref HEAD)"
