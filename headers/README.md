# dispatch calls

Run `make simulate` and then curl localhost:10000 or visit this url in your browser

The console will output something like:

```
[source/extensions/common/wasm/context.cc:1184] wasm log wasm-oidc-plugin: on_http_request_headers: [(":authority", "localhost:10000"), (":path", "/static/favicon.ico"), (":method", "GET"), (":scheme", "http"), ("sec-ch-ua", "\"Not-A.Brand\";v=\"99\", \"Chromium\";v=\"124\""), ("dnt", "1"), ("sec-ch-ua-mobile", "?0"), ("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36"), ("sec-ch-ua-platform", "\"macOS\""), ("accept", "image/avif,image/webp,image/apng,image/svg+xml,image/*,*/*;q=0.8"), ("sec-fetch-site", "same-origin"), ("sec-fetch-mode", "no-cors"), ("sec-fetch-dest", "image"), ("referer", "http://localhost:10000/"), ("accept-encoding", "gzip, deflate, br, zstd"), ("accept-language", "en-US,en;q=0.9"), ("cookie", "cookies redacted by author!"), ("if-none-match", "\"1540364205.0-22382-333649982\""), ("if-modified-since", "Wed, 24 Oct 2018 06:56:45 GMT"), ("x-forwarded-proto", "http"), ("x-request-id", "1401092d-187c-4561-87b7-80d68b994589")]
```
