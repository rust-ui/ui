# SOP: Deploy Rust/UI to production

Use this for the Dioxus SSR app at `rust-ui.com`, hosted on the shared VPS. The
source of truth is `rust_ui_internals/deploy_prod_vps.sh` plus
`.github/workflows/prod-vps.yml`. Deploy only after an explicit deploy request.

## 1. Confirm repository state

Run from the `RUST-UI` checkout:

```bash
git status --short --branch
git branch --show-current
```

The branch must be `main`, and the worktree and index must be clean. Review and
commit intended changes before deploying. Do not stash or discard someone else's
changes to satisfy this check. The script rejects staged or unstaged changes.

Confirm the deployment inputs are the intended ones:

```bash
git diff -- .github/workflows/prod-vps.yml docker-compose.shared-server.yml Dockerfile
sed -n '1,240p' rust_ui_internals/deploy_prod_vps.sh
```

The workflow uses GitHub secrets `SERVER_IP`, `SERVER_SSH_KEY`,
`DOCKER_USERNAME`, and `DOCKER_PASSWORD`. Never print secret values. The local
script also needs the configured SSH alias `shared-apps`, `gh` authentication
for `rust-ui/ui`, and these commands installed: `typos`, `cargo`,
`cargo-deny`, and `cargo-nextest`.

## 2. Understand what deploy changes

`deploy_prod_vps.sh` first copies `rust_ui_internals/setup__remote/rust-ui.conf`
to `shared-apps`, validates it with `nginx -t`, and reloads nginx. This happens
before the local branch, clean-tree, and quality checks, so inspect that config
diff before running the script.

The script then runs typos, `cargo fmt --all`, strict Clippy across all targets
and features, `cargo deny check`, and `cargo nextest run`. `--skip-tests` skips
only `cargo nextest run`; use it only when intentionally bypassing tests. If
formatting changes files, the script stages and commits them automatically.
Review the script's output and resulting commit before proceeding if this
happens.

On success, it creates and pushes a timestamped `deploy_*` tag and starts the
`prod-vps.yml` GitHub Actions workflow. The workflow builds and pushes the
Docker image unless `skip_build=true`, then replaces the app container on the
VPS and checks `http://localhost:5000/`. A redeploy without rebuilding uses:

```bash
gh workflow run prod-vps.yml --repo rust-ui/ui -f skip_build=true
```

Use that only when the current `latest` image is the intended release.

## 3. Run deploy with a persistent log

From the `RUST-UI` checkout, use a separate log file outside the repository:

```bash
mkdir -p /tmp/rust-ui-deploy
rust_ui_internals/deploy_prod_vps.sh > /tmp/rust-ui-deploy/run.log 2>&1 &
echo $!
tail -f /tmp/rust-ui-deploy/run.log
```

Keep visibility on each stage: preflight, nginx update, typos, formatting,
Clippy, dependency audit, tests, deploy tag, workflow build, VPS rollout, and
smoke test. Wait for the local script to exit, then inspect the GitHub Actions
run it started:

```bash
gh run list --repo rust-ui/ui --workflow prod-vps.yml --limit 5
gh run watch <run-id> --repo rust-ui/ui --exit-status
```

Do not report success until the workflow is green and production responds:

```bash
curl -fsS https://rust-ui.com/ >/dev/null
ssh shared-apps 'docker compose -f ~/dioxus-ui/docker-compose.shared-server.yml ps'
```

## 4. Failure loop

When a stage fails:

1. Stop at the first failing stage. Read its complete log and identify the
   failing command or GitHub Actions step.
2. Check live state before retrying. Nginx may already have changed; the deploy
   workflow may have stopped the old container before a new one became healthy.
3. Fix the root cause in the correct source file or configuration. Do not use
   `--skip-clippy` or `skip_build` just to turn a failed check green.
4. Re-run the relevant local check. For source changes, commit and push them to
   `main`, confirm a clean worktree, then run the complete deploy script again.
5. Watch the new GitHub Actions run through its production smoke test. Repeat
   this loop until all stages pass.

Never retry blindly after a partial deploy. The workflow calls `docker compose
down` before pulling and starting the new app, so inspect container status and
logs if rollout or smoke test fails:

```bash
ssh shared-apps 'docker compose -f ~/dioxus-ui/docker-compose.shared-server.yml ps'
ssh shared-apps 'docker compose -f ~/dioxus-ui/docker-compose.shared-server.yml logs app --tail=100'
```

## 5. Recovery

If nginx validation fails, the script restores its backup. If the app rollout
fails, use the logs and container state above, then restore the last known-good
immutable image tag (the prior successful run's commit SHA) by setting
`IMAGE_TAG` to that tag and recreating the app with the same compose file.
Verify the site with HTTPS after recovery. Keep the failed and recovery run IDs,
deployment tag, and image tag in the incident notes.

## Reference

- `rust_ui_internals/!!!_DEPLOYMENT/DEPLOY_VPS.md`
- `.github/workflows/prod-vps.yml`
- `docker-compose.shared-server.yml`
- `/Users/user/dev/1-RUST/RUSTIFY-APP/__SKILLS_SOP/deploy-prod.md` for the
  stage visibility and diagnose-fix-retry loop pattern.
