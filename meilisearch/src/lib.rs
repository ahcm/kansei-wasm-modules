use serde_json::{json, Map, Value};

#[no_mangle]
pub extern "C" fn alloc(size: i32) -> i32
{
    if size <= 0
    {
        return 0;
    }
    let mut buf = Vec::<u8>::with_capacity(size as usize);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr as i32
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: i32, len: i32)
{
    if ptr == 0 || len <= 0
    {
        return;
    }
    unsafe {
        let _ = Vec::from_raw_parts(ptr as *mut u8, 0, len as usize);
    }
}

fn read_string(ptr: i32, len: i32) -> Result<String, String>
{
    if len <= 0
    {
        return Ok(String::new());
    }
    let bytes = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };
    std::str::from_utf8(bytes)
        .map(|s| s.to_string())
        .map_err(|e| e.to_string())
}

fn write_string(value: &str) -> i64
{
    let mut buf = value.as_bytes().to_vec();
    let ptr = buf.as_mut_ptr() as u32;
    let len = buf.len() as u32;
    std::mem::forget(buf);
    ((len as u64) << 32 | ptr as u64) as i64
}

fn error_json(msg: &str) -> i64
{
    let payload = json!({"error": msg}).to_string();
    write_string(&payload)
}

fn build_url(base: &str, path: &str) -> String
{
    let base = base.trim_end_matches('/');
    let path = path.trim_start_matches('/');
    format!("{base}/{path}")
}

fn request_json(method: &str, url: &str, api_key: &str, body: Option<String>) -> Result<String, String>
{
    let agent = ureq::Agent::new();
    let mut req = agent.request(method, url).set("Accept", "application/json");
    if !api_key.is_empty()
    {
        req = req.set("X-Meili-API-Key", api_key);
    }

    let response = if let Some(body) = body
    {
        req.set("Content-Type", "application/json")
            .send_string(&body)
    }
    else
    {
        req.call()
    };

    match response
    {
        Ok(resp) => resp
            .into_string()
            .map_err(|e| format!("failed reading response: {e}")),
        Err(ureq::Error::Status(code, resp)) => {
            let body = resp.into_string().unwrap_or_default();
            Err(format!("http {code}: {body}"))
        }
        Err(err) => Err(format!("request failed: {err}")),
    }
}

#[no_mangle]
pub extern "C" fn meili_health_str(host_ptr: i32, host_len: i32) -> i64
{
    let host = match read_string(host_ptr, host_len)
    {
        Ok(v) => v,
        Err(err) => return error_json(&err),
    };
    let url = build_url(&host, "/health");
    match request_json("GET", &url, "", None)
    {
        Ok(body) => write_string(&body),
        Err(err) => error_json(&err),
    }
}

#[no_mangle]
pub extern "C" fn meili_version_str(host_ptr: i32, host_len: i32) -> i64
{
    let host = match read_string(host_ptr, host_len)
    {
        Ok(v) => v,
        Err(err) => return error_json(&err),
    };
    let url = build_url(&host, "/version");
    match request_json("GET", &url, "", None)
    {
        Ok(body) => write_string(&body),
        Err(err) => error_json(&err),
    }
}

#[no_mangle]
pub extern "C" fn meili_list_indexes_str(
    host_ptr: i32,
    host_len: i32,
    api_ptr: i32,
    api_len: i32,
) -> i64
{
    let host = match read_string(host_ptr, host_len)
    {
        Ok(v) => v,
        Err(err) => return error_json(&err),
    };
    let api_key = match read_string(api_ptr, api_len)
    {
        Ok(v) => v,
        Err(err) => return error_json(&err),
    };
    let url = build_url(&host, "/indexes");
    match request_json("GET", &url, &api_key, None)
    {
        Ok(body) => write_string(&body),
        Err(err) => error_json(&err),
    }
}

#[no_mangle]
pub extern "C" fn meili_create_index_str(
    host_ptr: i32,
    host_len: i32,
    api_ptr: i32,
    api_len: i32,
    uid_ptr: i32,
    uid_len: i32,
) -> i64
{
    let host = match read_string(host_ptr, host_len)
    {
        Ok(v) => v,
        Err(err) => return error_json(&err),
    };
    let api_key = match read_string(api_ptr, api_len)
    {
        Ok(v) => v,
        Err(err) => return error_json(&err),
    };
    let uid = match read_string(uid_ptr, uid_len)
    {
        Ok(v) => v,
        Err(err) => return error_json(&err),
    };
    let url = build_url(&host, "/indexes");
    let body = json!({"uid": uid}).to_string();
    match request_json("POST", &url, &api_key, Some(body))
    {
        Ok(body) => write_string(&body),
        Err(err) => error_json(&err),
    }
}

#[no_mangle]
pub extern "C" fn meili_search_str(
    host_ptr: i32,
    host_len: i32,
    api_ptr: i32,
    api_len: i32,
    index_ptr: i32,
    index_len: i32,
    query_ptr: i32,
    query_len: i32,
    limit: i32,
    offset: i32,
) -> i64
{
    let host = match read_string(host_ptr, host_len)
    {
        Ok(v) => v,
        Err(err) => return error_json(&err),
    };
    let api_key = match read_string(api_ptr, api_len)
    {
        Ok(v) => v,
        Err(err) => return error_json(&err),
    };
    let index = match read_string(index_ptr, index_len)
    {
        Ok(v) => v,
        Err(err) => return error_json(&err),
    };
    let query = match read_string(query_ptr, query_len)
    {
        Ok(v) => v,
        Err(err) => return error_json(&err),
    };

    let mut payload = Map::new();
    payload.insert("q".to_string(), Value::String(query));
    if limit >= 0
    {
        payload.insert(
            "limit".to_string(),
            Value::Number(serde_json::Number::from(limit as i64)),
        );
    }
    if offset >= 0
    {
        payload.insert(
            "offset".to_string(),
            Value::Number(serde_json::Number::from(offset as i64)),
        );
    }

    let url = build_url(&host, &format!("/indexes/{index}/search"));
    let body = Value::Object(payload).to_string();
    match request_json("POST", &url, &api_key, Some(body))
    {
        Ok(body) => write_string(&body),
        Err(err) => error_json(&err),
    }
}
