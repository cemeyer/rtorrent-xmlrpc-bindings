# rtorrent-xmlrpc-bindings
Typed, Rust-ey bindings for the XMLRPC rtorrent API

## Interface

The top-level structure is `Server`, which represents a logical
XMLRPC endpoint.  (For now, only HTTP endpoints are supported, but that
could be expanded relatively easily.)

One can get a list of loaded torrents via `Server::download_list()`.
`Download` objects represent a loaded torrent (identified by SHA1
digest, in hex).

For each `Download`, there are a number of accessors for attributes on
that loaded torrent.  (Accessors on `Download` correspond to the `d.*`
methods in the rtorrent API.)  Additionally, one can get a list of
trackers for that download with `Download::trackes()`.

`Tracker` objects represent a specific tracker for
a given download.  (Accessors on `Tracker` correspond to the
`t.*` methods in the rtorrent API.)  One example is
`Tracker::url()`.

We can get the `Peer`s for a loaded torrent via the
`Download::peers()` method.  Peers represent other participants in the
swarm for that particular torrent.  (Accessors on `Peer` correspond to
the `p.*` methods in the rtorrent API.)  One example is
`Peer::address()`.

`File` objects represent an individual file associated with a download.
Downloads may have one or more `Download::files()`.  Accessors on `File`
correspond to the `f.*` methods in the rtorrent API.  An example is
`File::path()`.

## Example

```rust
use rtorrent_xmlrpc_bindings as rtorrent;

fn main() -> Result<()> {
    let handle = rtorrent::Server::new("http://1.2.3.4/RPC2");
    for dl in handle.download_list()? {
        println!("{}: {}", dl.name()?, if dl.is_active()? { "active" } else { "inactive" });
    }
    Ok(())
}
```
