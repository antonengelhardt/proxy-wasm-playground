# dispatch calls

Run `make simulate` and then curl localhost:10000 or visit this url in your browser

The console will output something like:

```
on_http_call_response: [(":status", "200"), ("date", "Tue, 30 Apr 2024 14:19:40 GMT"), ("expires", "-1"), ("cache-control", "private, max-age=0"), ("content-type", "text/html; charset=ISO-8859-1"), ("content-security-policy-report-only", "object-src 'none';base-uri 'self';script-src 'nonce-oMP_MLPHgY4UWUoIN23_dg' 'strict-dynamic' 'report-sample' 'unsafe-eval' 'unsafe-inline' https: http:;report-uri https://csp.withgoogle.com/csp/gws/other-hp"), ("p3p", "CP=\"This is not a P3P policy! See g.co/p3phelp for more info.\""), ("server", "gws"), ("x-xss-protection", "0"), ("x-frame-options", "SAMEORIGIN"), ("set-cookie", "AEC=AQTF6HzVXixq3GklApm4vThiyevjI0GcizRcoA3onXzBAvlBw7sgIBvKfw; expires=Sun, 27-Oct-2024 14:19:40 GMT; path=/; domain=.google.com; Secure; HttpOnly; SameSite=lax"), ("set-cookie", "__Secure-ENID=19.SE=r2fFOoPtkXHcN6-N6vEJefM6mio823Hd7jmYuRXMzy2XCW5zdYioFD_L2p-oo8rqsAE3GMVLb-JUHeiaJM3bawMQHBofPJhUIvkpiKPeAFPy4FMwI4ccIIbCJoU6ATKglCrvlWPAS73oWio4VY3ZYmzq6nPdahI9ZXTDxd_NzrQhtDiiNX4; expires=Sat, 31-May-2025 06:37:58 GMT; path=/; domain=.google.com; Secure; HttpOnly; SameSite=lax"), ("alt-svc", "h3=\":443\"; ma=2592000,h3-29=\":443\"; ma=2592000"), ("accept-ranges", "none"), ("vary", "Accept-Encoding"), ("transfer-encoding", "chunked"), ("x-envoy-upstream-service-time", "415")]
```
