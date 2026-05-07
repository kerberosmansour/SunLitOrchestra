# Nettacker Location Resolution

Use this reference before writing commands or authoring modules. The skill must know which Nettacker installation it is targeting, because users may have Nettacker installed with pipx, pip, Poetry, Docker, or as a local checkout anywhere on disk.

## What Local Sources Say

The local Nettacker checkout declares a console script in `pyproject.toml`:

```toml
[tool.poetry.scripts]
nettacker = "nettacker.main:run"
```

The local installation docs show these supported runner forms:

- `pipx install nettacker`, then `nettacker --help`
- `pip3 install nettacker` inside a virtualenv, then `nettacker --help`
- cloned repo plus `python3 nettacker.py --help`
- cloned repo plus `poetry run nettacker --help`
- Docker image `owasp/nettacker`

Therefore, a PATH command named `nettacker` is expected after pipx/pip/Poetry-script installs, but it is not guaranteed on every machine.

## Resolution Order

1. **Explicit user path wins**: if the user gives `--nettacker-bin`, `--nettacker-dir`, a full path to `nettacker.py`, or a checkout path, validate that path first.
2. **Current/sibling checkout**: if cwd or a nearby path such as `../Nettacker` contains `nettacker.py`, `pyproject.toml`, and `nettacker/modules/`, prefer `python3 <checkout>/nettacker.py` for CLI runs and that checkout for module authoring.
3. **PATH console script**: check `command -v nettacker`. If present, run `nettacker --help` or `nettacker --version` as a harmless probe and record the absolute path.
4. **Poetry checkout**: if a checkout exists but direct dependencies are missing, try `poetry run nettacker --help` from that checkout when Poetry is installed.
5. **Docker**: check `docker image inspect owasp/nettacker` or use `docker run --rm owasp/nettacker --help` only when Docker is available and the user accepts Docker use. When Docker is selected, record host architecture and image architecture. A quick image probe is `docker image inspect owasp/nettacker --format '{{.Architecture}}'`; compare it with `uname -m` or the host's equivalent. If they differ, record the emulation risk and prefer a local checkout for long scans when practical.
6. **Not found**: stop and ask for the install path or permission to install/setup. Do not continue with guessed commands.

## Runner Record

Record this block before any assessment command:

```text
Nettacker runner:
- kind: checkout | path-cli | poetry | docker | api
- command prefix: <exact argv prefix>
- source path/image: <absolute path or image tag>
- validation command: <help/version probe>
- validation result: pass | fail
- image architecture: <architecture> (Docker only)
- host architecture: <architecture> (Docker only)
- emulation risk: none | possible | observed (Docker only)
```

For module authoring, `kind: checkout` is required because writing a custom module needs the source tree under `nettacker/modules/`. A PATH-only or Docker-only install can run scans, but it is not enough to add or test local YAML modules unless the user also provides a checkout or writable source tree.

## Command Examples

These are runner prefixes, not complete scans:

```bash
nettacker
python3 /absolute/path/to/Nettacker/nettacker.py
poetry run nettacker
docker run --rm -v "$PWD/evidence:/evidence" owasp/nettacker
```

When composing commands, append assessment flags after the resolved prefix. Keep target strings in files for larger scopes and preserve the authorization gate from `SKILL.md`.
