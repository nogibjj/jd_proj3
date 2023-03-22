use aws_lambda_events::event::s3::S3Event;
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::Client;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use photon_rs::native::open_image_from_bytes;
use photon_rs::native::save_image;
use serde::Serialize;
use std::path::Path;
use watermark::process_image;

#[derive(Serialize)]
struct Output {
    message: String,
}

async fn handler(event: LambdaEvent<S3Event>) -> Result<Output, Error> {
    // get config from env;
    let config = aws_config::load_from_env().await;
    let src_ap = "arn:aws:s3:us-east-1:760091102987:accesspoint/passport".to_string();
    let trgt_ap = "arn:aws:s3:us-east-1:760091102987:accesspoint/watermark".to_string();
    let client = Client::new(&config);

    // get the key of the new image
    let key = event.payload.records[0]
        .s3
        .object
        .key
        .as_ref()
        .unwrap()
        .as_str();

    // get new image from s3
    let result = client
        .get_object()
        .bucket(src_ap.clone())
        .key(key)
        .response_content_type("application/json")
        .send()
        .await
        .expect("Failed to get object");
    let bytes = result.body.collect().await?.into_bytes();
    let img = open_image_from_bytes(&bytes).expect("Opening image failed");

    // get watermark image from s3
    let wm_res = client
        .get_object()
        .bucket(src_ap)
        .key("watermark.png")
        .response_content_type("application/json")
        .send()
        .await
        .expect("Failed to get object");
    let wm_bytes = wm_res.body.collect().await?.into_bytes();
    let wm_img = open_image_from_bytes(&wm_bytes).expect("Opening image failed");

    // resize and watermark new image
    let new_img = process_image(wm_img, img);
    save_image(new_img.clone(), "/tmp/test.jpg").expect("Save failed");
    let body = ByteStream::from_path(Path::new("/tmp/test.jpg"))
        .await
        .expect("Failed to read file");

    let new_key = format!("{}_watermarked.jpg", key.split('.').next().unwrap());
    let resp = client
        .put_object()
        .bucket(trgt_ap)
        .key(new_key)
        .body(body)
        .send()
        .await
        .expect("Failed to put object");

    // message of completion
    let message = format!("Upload success. Version: {:?}", resp.version_id);
    Ok(Output { message })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let args = Args::parse();
    // process_image(args.file_name);
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(handler)).await
}
