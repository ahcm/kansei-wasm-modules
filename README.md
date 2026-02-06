# kansei-wasm-modules
WASM modules for Kansei. Install with `kansei wasm install <name>`.

## example
Example module with numeric exports:
- `fastx_version()` -> i32
- `fastx_add_i64(a, b)` -> i64
- `fastx_mul_f64(a, b)` -> f64
- `fastx_pow_f64(a, b)` -> f64

Build target: `wasm32-wasip1`.

Target install:
```
rustup target add wasm32-wasip1
```

Kansei usage:
```ruby
load wasm::example
Example = wasm.example
puts Example.fastx_version()
puts Example.fastx_add_i64(2, 3)
puts Example.fastx_mul_f64(1.5, 4.0)
puts Example.fastx_pow_f64(2.0, 3.0)
```

## meilisearch
Minimal Meilisearch HTTP client for Kansei. These functions return JSON strings
directly from the Meilisearch API.

Exports:
- `meili_health_str(host_ptr, host_len)` -> string
- `meili_version_str(host_ptr, host_len)` -> string
- `meili_list_indexes_str(host_ptr, host_len, api_ptr, api_len)` -> string
- `meili_create_index_str(host_ptr, host_len, api_ptr, api_len, uid_ptr, uid_len)` -> string
- `meili_search_str(host_ptr, host_len, api_ptr, api_len, index_ptr, index_len, query_ptr, query_len, limit, offset)` -> string

Notes:
- `host` should be an HTTP base URL, e.g. `http://127.0.0.1:7700`.
- `api_key` may be an empty string if the server does not require one.
- `limit` and `offset` can be `-1` to omit them.

Kansei usage:
```ruby
load wasm::meilisearch
Meili = wasm.meilisearch

host = "http://127.0.0.1:7700"
api = "" # or your API key

puts Meili.meili_health_str(host)
puts Meili.meili_version_str(host)
puts Meili.meili_list_indexes_str(host, api)
puts Meili.meili_create_index_str(host, api, "books")
puts Meili.meili_search_str(host, api, "books", "hobbit", 10, 0)
```
