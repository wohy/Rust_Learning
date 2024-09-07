mod pb;
use anyhow::Result;
use axum::{
    extract::{Extension, Path},
    handler::get,
    http::{HeaderMap, HeaderValue, StatusCode},
    AddExtensionLayer, Router,
};
use bytes::Bytes;
use lru::LruCache;
use pb::*;
use percent_encoding::{percent_decode_str, percent_encode, NON_ALPHANUMERIC};
use serde::Deserialize;
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    sync::Arc,
};
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tracing::{info, instrument};
mod engine;
use engine::{Engine, Photon};
use image::ImageOutputFormat;

#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}

type Cache = Arc<Mutex<LruCache<u64, Bytes>>>;

///#[instrument] 宏是 tracing 库提供的一个特性，tracing 是 Rust 中一个流行的日志和跟踪库。
///属性宏的参数解释：
///level：指定记录的日志级别。常见的日志级别包括 error、warn、info、debug 和 trace。在这个例子中，日志级别被设置为 info。
///skip：告诉 instrument 宏跳过记录某些参数的值。这可以避免在日志中记录敏感信息或大量数据，从而保持日志的清晰和简洁。在这个例子中，cache 参数被跳过，不会记录其值。
#[instrument(level = "info", skip(cache))]
async fn retrieve_image(url: &str, cache: Cache) -> Result<Bytes> {
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    // 当前图片 url 转换成 hash 后的值
    let key = hasher.finish();
    // 当前LRU 缓存池
    let g = &mut cache.lock().await;
    let data = match g.get(&key) {
        // 匹配到了相应的 key
        Some(v) => {
            info!("Match cache {}", key);
            v.to_owned()
        }
        // 未匹配到相应的key
        None => {
            info!("Retrieve url");
            // 则通过 get 请求去获取
            let resp = reqwest::get(url).await?;
            let data = resp.bytes().await?;
            // 更新缓存
            g.put(key, data.clone());
            data
        }
    };
    Ok(data)
}

async fn generate(
    Path(Params { spec, url }): Path<Params>,
    Extension(cache): Extension<Cache>,
) -> Result<(HeaderMap, Vec<u8>), StatusCode> {
    // decode 图片地址
    let url = percent_decode_str(&url).decode_utf8_lossy();
    // protobuf 字符串转化成 ImageSpec 结构体
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    // Ok(format!("url: {}\n spec: {:#?}", url, spec))
    let data = retrieve_image(&url, cache)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    // 使用 image engine 处理
    let mut engine: Photon = data
        .try_into()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    engine.apply(&spec.specs);

    let image = engine.generate(ImageOutputFormat::Jpeg(85));

    info!("Finished processing: image size {}", image.len());
    let mut headers = HeaderMap::new();

    headers.insert("content-type", HeaderValue::from_static("image/jpeg"));
    Ok((headers, image))
}

// 调试辅助函数 生成请求的 url
fn print_test_url(url: &str) {
    use std::borrow::Borrow;
    // 裁剪图片大小为 500 * 800
    let spec1 = Spec::new_resize(500, 800, resize::SampleFilter::CatmullRom);
    // 相对图片的坐标位置添加水印处理
    let spec2 = Spec::new_watermark(20, 20);
    // 添加 Marine 滤镜处理
    let spec3 = Spec::new_filter(filter::Filter::Marine);
    let image_spec = ImageSpec::new(vec![spec1, spec2, spec3]);
    // spec 转换为 protobuf 对应 base64 编码 字符串
    let s: String = image_spec.borrow().into();
    // encode 原图片地址
    let test_image = percent_encode(url.as_bytes(), NON_ALPHANUMERIC).to_string();
    println!("test url: http://localhost:3000/image/{}/{}", s, test_image);
}

#[tokio::main]
async fn main() {
    // 初始化 tracing
    tracing_subscriber::fmt::init();
    // 初始化缓存
    let cache: Cache = Arc::new(Mutex::new(LruCache::new(1024)));
    // 构建路由
    let app = Router::new()
        .route("/image/:spec/:url", get(generate))
        .layer(
            ServiceBuilder::new()
                .layer(AddExtensionLayer::new(cache))
                .into_inner(),
        );
    let addr = "127.0.0.1:3000".parse().unwrap();

    print_test_url("https://images.pexels.com/photos/1562477/pexels-photo-1562477.jpeg?auto=compress&cs=tinysrgb&dpr=3&h=750&w=1260");

    info!("Listening on {}", &addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
