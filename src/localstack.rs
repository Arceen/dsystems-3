use aws_config::{BehaviorVersion, Region};
use std::time::Duration;
async fn does_bucket_exist(client: &aws_sdk_s3::Client, bucket_name: &str) -> bool {
    let buckets = client.list_buckets().send().await.unwrap();
    let target_bucket = buckets.buckets().iter().find(|bucket| bucket.name() == Some(bucket_name));

    match target_bucket {
        Some(_) => true,
        None => false
    }
}

// just to experiment
// in real case use head_bucket that sends HEAD request (lightweight, efficient) provided by aws s3 sdk
async fn create_bucket(client: &aws_sdk_s3::Client, bucket_name: &str) {
    client
        .create_bucket()
        .bucket(bucket_name.to_owned())
        .send()
        .await
        .unwrap();
}
async fn init_localstack() {
    let config = aws_config::from_env()
        .test_credentials()
        .region(Region::new("us-east-1"))
        .endpoint_url("http://127.0.0.1:4566")
        .behavior_version(BehaviorVersion::latest())
        .load()
        .await;

    let bucket_name = "my-buccccket";
    let my_file = "image.jpg";
    let client = aws_sdk_s3::Client::new(&config);
    create_bucket(&client, bucket_name).await;

    println!("Bucket exists? {}", does_bucket_exist(&client, bucket_name).await);
    println!("Bucket exists? {}", does_bucket_exist(&client, "ninja").await);
    println!("Bucket exists? {}", does_bucket_exist(&client, "bucket-name").await);
    // println!("{}", generate_presigned_url(&client, bucket_name, my_file).await);
    println!("{}", download_presigned_url(&client, bucket_name, my_file).await);
}
async fn generate_presigned_url(client: &aws_sdk_s3::Client, bucket_name: &str, key: &str) -> String {
    let presigning_config = aws_sdk_s3::presigning::PresigningConfig::builder().expires_in(Duration::from_secs(150)).build().unwrap();
    let presigned_request = client.put_object().bucket(bucket_name.to_owned()).key(key).presigned(
        presigning_config
    ).await.unwrap();
    presigned_request.uri().to_string()
}

async fn download_presigned_url(client: &aws_sdk_s3::Client, bucket_name: &str, key: &str) -> String {
    let presigning_config = aws_sdk_s3::presigning::PresigningConfig::builder().expires_in(Duration::from_secs(60)).build().unwrap();
    let presigned_url = client.get_object().bucket(bucket_name).key(key).presigned(presigning_config).await.unwrap();
    presigned_url.uri().to_string()
}