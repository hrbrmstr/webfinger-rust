webfinger
================

Bare bones Rust [WebFinger](https://www.rfc-editor.org/rfc/rfc7033)
client.

``` bash
cargo run --release acct:hrbrmstr@rud.is
```

        Finished release [optimized] target(s) in 0.05s
         Running `target/release/webfinger 'acct:hrbrmstr@rud.is'`


    {"subject":"hrbrmstr@rud.is","aliases":["https:\/\/rud.is\/b\/author\/hrbrmstr\/","https:\/\/mastodon.social\/@hrbrmstr","https:\/\/mastodon.social\/users\/hrbrmstr","https:\/\/infosec.exchange\/@hrbrmstr","https:\/\/infosec.exchange\/users\/hrbrmstr"],"links":[{"rel":"self","type":"application\/activity+json","href":"https:\/\/rud.is\/b\/author\/hrbrmstr\/"},{"rel":"http:\/\/webfinger.net\/rel\/profile-page","type":"text\/html","href":"https:\/\/rud.is\/b\/author\/hrbrmstr\/"}]}

``` bash
cargo run --release -- --pretty acct:hrbrmstr@rud.is 
```

        Finished release [optimized] target(s) in 0.05s
         Running `target/release/webfinger --pretty 'acct:hrbrmstr@rud.is'`
    {
      "subject": "hrbrmstr@rud.is",
      "aliases": [
        "https:\/\/rud.is\/b\/author\/hrbrmstr\/",
        "https:\/\/mastodon.social\/@hrbrmstr",
        "https:\/\/mastodon.social\/users\/hrbrmstr",
        "https:\/\/infosec.exchange\/@hrbrmstr",
        "https:\/\/infosec.exchange\/users\/hrbrmstr"
      ],
      "links": [
        {
          "rel": "self",
          "type": "application\/activity+json",
          "href": "https:\/\/rud.is\/b\/author\/hrbrmstr\/"
        },
        {
          "rel": "http:\/\/webfinger.net\/rel\/profile-page",
          "type": "text\/html",
          "href": "https:\/\/rud.is\/b\/author\/hrbrmstr\/"
        }
      ]
    }
