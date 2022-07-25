# Sleme

## Developing

### Forwarding the development worker

When running `yarn dev`, wrangler runs a development server. This server is
temporarily hosted by Cloudflare to allow access to KV, Durable Objects and
other Cloudflare services. It also forwards the service to the local address
`http://localhost:8787`. By default there is no public endpoint available to
access your local server ([this is a missing feature][expose-wrangler]).

In order for your development slack bot to talk with your remotely running,
locally forwarded worker, you need a public endpoint. Some solutions include
tools like `ngrok`, however in the free version, you cannot have a static
host name. You need to refresh the URL in the Slack developer tools on every run
of`ngrok`.

if you have a paid plan or are willing to change the URL everytime, you can
skip the rest of this chapter.

For everyone else, Cloudflare offers it's own service, called Cloudflare
Tunnels, which has free persistent tunnels as long as your domain is hosted
on Cloudflare DNS. As you are looking at a Cloudflare Worker project here,
I'm going to assume you have at least one domain hosted at Cloudflare so you can
use the approach described here. Otherwise you are out of luck and Cloudflare
will also autogenerate a random, non-persistent URL for you.

1. Make sure to install [cloudflared]
2. Login: `cloudflared tunnel login`
3. Set the `TUNNEL_HOSTNAME` environment variable (see [./env.template])
4. Run `yarn tunnel`  
   By default it will run the tunnel in UI mode and after
   a short wait you will see your tunnel becoming available. If you don't
   have a DNS entry pointing towards your tunnel yet, `cloudflared` will set
   it up for you.
5. Your development worker will now be available at your provided hostname.

[expose-wrangler]: https://github.com/cloudflare/wrangler2/issues/696

[cloudflared]: https://github.com/cloudflare/cloudflared
