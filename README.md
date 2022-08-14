# Sleme

## Development

### Repo structure

The repo has two important branches:

* `develop` - The main branch for production. This branch contains the latest merged changes.
* `main` - The currently deployed state on production.

### Forwarding the development worker

> This chapter is outdated and needs to be reworked. You cannot share a locally set-up cloudflare tunnel between
> multiple development machines.

<details>
When running `yarn dev`, wrangler runs a development server.
Cloudflare temporarily hosts this server to allow access to KV, Durable Objects and other Cloudflare services.
It forwards the service to the local address `http://localhost:8787`.
By default, there is no public endpoint available to access your local
server ([this is a missing feature][expose-wrangler]).

In order for your development slack bot to talk with your remotely running,
locally forwarded worker, you need a public endpoint.
Some solutions include tools like `ngrok`, however, in the free version, you cannot have a static host name.
You need to refresh the URL in the Slack developer tools on every run of`ngrok`.

If you have a paid plan or are willing to change the URL everytime, you can skip the rest of this chapter.

For everyone else, Cloudflare offers its own service, called Cloudflare Tunnels,
which has free persistent tunnels as long as you host your domain on Cloudflare DNS.
As you are looking at a Cloudflare Worker project here,
I am going to assume you have at least one domain hosted at Cloudflare, so you can use the approach described here.
Otherwise, you are out of luck and Cloudflare will also autogenerate a random, non-persistent URL for you.

1. Make sure to install [cloudflared]
2. Login: `cloudflared tunnel login`
3. Set the `TUNNEL_HOSTNAME` environment variable (see [./env.template])
4. Run `yarn tunnel`
   By default the tunnel runs in UI mode and after a short wait, you see your tunnel becoming available.
   If you don't have a DNS entry pointing towards your tunnel yet, `cloudflared` sets it up for you.
5. Your development worker will now be available at your provided hostname.
</details>

[expose-wrangler]: https://github.com/cloudflare/wrangler2/issues/696

[cloudflared]: https://github.com/cloudflare/cloudflared
