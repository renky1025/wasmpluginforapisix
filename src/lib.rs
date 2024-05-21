use log::info;
use proxy_wasm as wasm;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(wasm::types::LogLevel::Trace);
    proxy_wasm::set_http_context(
        |context_id, _root_context_id| -> Box<dyn wasm::traits::HttpContext> {
            Box::new(HelloWorld { context_id })
        },
    )
}

struct HelloWorld {
    context_id: u32,
}

impl wasm::traits::Context for HelloWorld {}

impl wasm::traits::HttpContext for HelloWorld {
    fn on_http_request_headers(&mut self, _num_headers: usize, _end_of_stream: bool) ->  wasm::types::Action {
        for(name, value) in &self.get_http_request_headers() {
            info!("::::H[{}] -> {}: {}", self.context_id, name, value);
        }
        //针对/hello 接口 返回 hello,world.
        match self.get_http_request_header(":path") {
            Some(path) if !(path.is_empty()) => {
                info!("/hello == {}", path);
                self.send_http_response(200,
                    vec![("hello", "world"), ("Powered-By", "proxy-wasm")],
                    Some(("Hello, World!\n").as_bytes().try_into().unwrap()),
                );
                wasm::types::Action::Pause
            }
            _ =>  wasm::types::Action::Continue
        }
    }
    // 修改response 返回信息
    fn on_http_response_headers(&mut self, _num_headers: usize, _end_of_stream: bool) ->  wasm::types::Action {
        // 设置一些 header
        info!("on_http_response_headers");
        self.set_http_response_header("X-Proxy-Hosts", Some("test-proxy-host"));
        self.set_http_response_header("Powered-By", Some("my-proxy-wasm"));
        wasm::types::Action::Continue
    }
}

