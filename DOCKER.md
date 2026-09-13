# Docker Usage Guide

`bump2version` provides a heavily optimized Docker container based on Alpine Linux.

## Running the Container

The default entrypoint is set to the CLI (`bump`).

```sh
# Display help menu
docker run --rm wiseaidev/bump2version --help

# Bump patch version (dry-run)
docker run --rm \
  -v "$(pwd):/workspace" \
  wiseaidev/bump2version \
  --config-file /workspace/.bumpversion.toml \
  --bump patch \
  --dry-run

# Bump with commit (mounts .git for write access)
docker run --rm \
  -v "$(pwd):/workspace" \
  -v "$(pwd)/.git:/workspace/.git" \
  wiseaidev/bump2version \
  --config-file /workspace/.bumpversion.toml \
  --bump patch \
  --commit
```

## Pulling the Image

You can pull the official pre-built image from Docker Hub:

```sh
docker pull wiseaidev/bump2version:latest
```

## Building the Container Locally

Install the [docker buildx plugin](https://docs.docker.com/build/concepts/overview/):

```sh
sudo apt-get update
sudo apt-get install docker-buildx-plugin
```

Once installed, you can use BuildKit natively:

```sh
docker buildx build -t local/bump2version .
docker run --rm local/bump2version -h
```

You can alias the command for convenience in your shell profile (`~/.bashrc` or `~/.zshrc`):

```sh
alias bump="docker run --rm -v \"\$(pwd):/workspace\" wiseaidev/bump2version"
```

## Available Tags

| Tag      | Description                |
| -------- | -------------------------- |
| `latest` | Most recent stable release |
| `0.2.2`  | Specific patch version     |
| `0.2`    | Latest 0.2.x patch         |

## Image Details

- **Base**: `alpine:3.22.4`
- **Binary**: `/usr/local/bin/bump` (statically linked, stripped)
- **Alias**: `/usr/local/bin/cargo-bump` → symlink to `bump`
- **User**: runs as non-root `bump2version` user
- **Size**: ~8 MB compressed
