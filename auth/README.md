# dispatch calls

Run `make simulate`.

Console:

```
➜ curl "http://localhost:10000/"
Unauthorized⏎
~
➜ curl "http://localhost:10000/" -H "Authorization: I am authorized"
Authorized⏎
~
➜
```

Logs

```
envoy-1    | [2024-04-30 14:46:41.030][24][debug][wasm] [source/extensions/common/wasm/context.cc:1184] wasm log proxy-wasm-playground: Unauthorized
envoy-1    | [2024-04-30 14:46:43.243][24][debug][wasm] [source/extensions/common/wasm/context.cc:1184] wasm log proxy-wasm-playground: Authorized
```
