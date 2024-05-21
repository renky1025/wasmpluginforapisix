## 自定义APISIX WASM 插件

### 创建 Rust 项目
```shell
cargo new --lib my_proxy_wasm_project
```

### 在Cargo.toml文件中添加 proxy-wasm作为依赖项

```shell
[lib]
path = "src/lib.rs"
crate-type = ["cdylib"]

[dependencies]
proxy-wasm="0.2.1"
serde = {version = "1.0", features =["derive"]}
serde_json= "1.0"
log="0.4"
```

### 编写你的 Wasm 插件
lib.rs 基本结构如下
```shell
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
        ## todo
    }

    fn on_http_response_headers(&mut self, _num_headers: usize, _end_of_stream: bool) ->  wasm::types::Action {
        ## todo
     }
}
```

### 构建wasm

```shell
## 下载windows 依赖lib
rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasi
## 正式构建
cargo build --target wasm32-wasi --release
## 找到 目录， .\target\wasm32-wasi\release\ *.wasm 文件就是最后需要发布插件

```

### 添加apisix conf.yaml

```shell
##最后添加（如果有wasm 组，直接添加）
wasm:
  plugins:
    - name: my_proxy_wasm_project # the name of the plugin
      priority: 7999 # priority
      file: /usr/local/apisix/conf/my_proxy_wasm_project.wasm # the path of `.wasm` file
      http_request_phase: access # default to "access", can be one of ["access", "rewrite"]

```

### 导出apisix的schema.json

```shell
curl 127.0.0.1:9092/v1/schema > schema.json

```

### 复制schema.json 到 apisix-dashboard 服务
```shell
# 我这里用的docker 起的服务，直接 cp进去容器 重启服务
docker cp schema.json apisix-dashboard-1:/usr/local/apisix-dashboard/conf/
# 如果你是原生安装，直接 把文件覆盖到/usr/local/apisix-dashboard/conf/即可
```

### 验证
![插件](/images/1.png "自定义插件")
![结果](/images/2.png "结果")