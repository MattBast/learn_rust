```bash
curl http://127.0.0.1:3030
```

```bash
curl -X POST -H "Content-Type: application/json" -d '{"type": "page_view"}' http://127.0.0.1:3030/event
```