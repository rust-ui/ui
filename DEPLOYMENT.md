# DEPLOYMENT

How rust-ui.com and its subdomains are built, shipped and routed. Keep this
current when any pipeline, port or domain changes.

## Servers

| Name | Host | Access |
|---|---|---|
| shared-apps | Hetzner, `2a01:4f8:c012:31b3::1` | `ssh shared-apps` (root, `~/.ssh/hetzner-mac`) |

All rust-ui apps are containers on shared-apps behind the host nginx
(`/etc/nginx/conf.d/`). Cloudflare sits in front (proxied, SSL mode Full).

```
Browser
  -> Cloudflare (proxied)
     -> shared-apps host nginx :80 / :443
        -> 127.0.0.1:<port> -> app container
```

## Domain -> container map

| Domain | Host port | Container | Framework | Repo / pipeline |
|---|---|---|---|---|
| rust-ui.com, www | 4002 | leptos-ui-app-1 | Leptos SSR | rust-ui/leptos-ui |
| leptos.rust-ui.com | 4002 | leptos-ui-app-1 | Leptos SSR | rust-ui/leptos-ui |
| dioxus.rust-ui.com | 5000 | dioxus-ui-app-1 | Dioxus SSR | rust-ui/dioxus-ui |

`rust-ui.com` + `www` + `leptos.rust-ui.com` share **one** container (the Leptos
app is host-agnostic: same SSR server answers any `Host`). nginx forwards `$host`
untouched so canonical URLs / SEO still see the real domain.

Retired after cutover: `root-app-1` on port 4001 (old Leptos container),
`~/docker-compose.prod.yml` on the server,
`rust_ui_internals/deploy_prod_vps.sh`.

## Pipelines (both: manual trigger only)

### leptos-ui  (rust-ui/leptos-ui)

```
leptos-ui/deploy_prod.sh
  gates: typos, cargo fmt + leptosfmt, clippy (tw_merge), semver-checks,
         cargo audit, cargo nextest
  -> git tag deploy_<ts> + push
  -> gh workflow run prod-vps.yml --repo rust-ui/leptos-ui
       build: everlabs/leptos-ui:latest   (Dockerfile, musl, cargo-leptos)
              build-args RESEND_TOKEN, RESEND_AUDIENCE_ID, RUSTIFY_API_URL,
              BUG_REPORTS_API_KEY
       deploy: scp docker-compose.shared-server.yml -> ~/leptos-ui/ on shared-apps
               docker compose up -d  ->  leptos-ui-app-1  @ 127.0.0.1:4002:3000
       smoke:  curl -sf localhost:4002

  redeploy current image without rebuild:
    gh workflow run prod-vps.yml --repo rust-ui/leptos-ui -f skip_build=true
```

Repo Variables: `DOCKER_USERNAME`, `SERVER_IP`, `RUSTIFY_API_URL`.
Repo Secrets: `DOCKER_TOKEN`, `RESEND_TOKEN`, `RESEND_AUDIENCE_ID`,
`BUG_REPORTS_API_KEY`, `SERVER_SSH_KEY`.

### dioxus-ui  (rust-ui/dioxus-ui)

Same shape. `dioxus-ui/deploy_prod.sh` -> `prod-vps.yml` -> `everlabs/dioxus-ui:latest`
-> `~/dioxus-ui/docker-compose.shared-server.yml` -> `dioxus-ui-app-1` @ 127.0.0.1:5000:8080.

## nginx

Live file: `shared-apps:/etc/nginx/conf.d/rust-ui.conf`
Source-of-truth copy: `rust_ui_internals/setup__remote/rust-ui.conf`
Edit the copy, scp it up, then `nginx -t && systemctl reload nginx`.

## TLS

One Let's Encrypt cert for the Leptos names:

```
ssh shared-apps 'certbot --nginx -d rust-ui.com -d www.rust-ui.com -d leptos.rust-ui.com'
# -> /etc/letsencrypt/live/rust-ui.com/  (dir named after the first -d)
```

`dioxus.rust-ui.com` keeps its own cert. Certbot auto-renews via its systemd timer.
HTTP-01 works through the Cloudflare proxy (ACME challenge passes through).

## DNS (Cloudflare)

| Record | Value | Proxy |
|---|---|---|
| `rust-ui.com` A/AAAA | shared-apps IP | on |
| `www` | -> rust-ui.com | on |
| `dioxus` | shared-apps IP | on |
| `leptos` AAAA | `2a01:4f8:c012:31b3::1` | on |

`leptos` mirrors the `dioxus` record (same type + proxy state).

## Cutover runbook — move rust-ui.com from :4001 to :4002

Run in order. Steps 1-2 are yours; 3+ are on shared-apps.

```
1. Cloudflare: add the `leptos` DNS record (table above). Wait for it to resolve:
     dig +short leptos.rust-ui.com

2. Local: push leptos-ui main, then deploy:
     cd leptos-ui && git push origin main && ./deploy_prod.sh
   Wait for the workflow to go green. leptos-ui-app-1 is now up on :4002,
   rust-ui.com is still served by root-app-1 on :4001 (no downtime yet).

3. Verify the new container:
     ssh shared-apps 'curl -sf http://127.0.0.1:4002/ -o /dev/null && echo OK'

4. Issue the cert (see TLS above).

5. Install the new vhost:
     scp rust_ui_internals/setup__remote/rust-ui.conf \
         shared-apps:/etc/nginx/conf.d/rust-ui.conf
     ssh shared-apps 'nginx -t && systemctl reload nginx'

6. Check all three domains serve and are healthy, then retire the old stack:
     ssh shared-apps 'docker rm -f root-app-1 && rm -f ~/docker-compose.prod.yml'

7. Delete rust_ui_internals/deploy_prod_vps.sh (superseded by
   leptos-ui/deploy_prod.sh).
```

## Next: swap rust-ui.com apex from Leptos to Dioxus

Planned. When Dioxus is ready to own the apex:

1. In `rust_ui_internals/setup__remote/rust-ui.conf`, move `rust-ui.com
   www.rust-ui.com` off the `:4002` `server_name` lines and onto the
   `dioxus.rust-ui.com` block (or give Dioxus its own block on the apex).
2. Reissue / expand the cert to cover the apex on the Dioxus side.
3. scp + `nginx -t && reload`.
4. `leptos.rust-ui.com` stays pointed at `:4002` permanently.

The Leptos apex code lives in the parent `RUST-UI` repo today; the standalone
copy is in `leptos-ui` (see `PLAN_MOVE_LEPTOS_SITE_PHASE_*.md`). Replacing the
parent `RUST-UI` app with Dioxus does not affect `leptos.rust-ui.com`, which is
served entirely from `rust-ui/leptos-ui`.
