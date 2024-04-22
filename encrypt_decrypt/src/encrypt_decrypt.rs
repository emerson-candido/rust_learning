use rusoto_core::Region;
use rusoto_kms::{Kms, KmsClient, EncryptRequest, DecryptRequest};

async fn encrypt_data(plaintext: &str) -> Result<String, rusoto_kms::Error> {
    let client = KmsClient::new(Region::UsEast1); // Choose the appropriate region

    let encrypt_request = EncryptRequest {
        key_id: "arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012".to_string(), // Replace with your CMK ARN
        plaintext: plaintext.to_string().into_bytes(),
        ..Default::default()
    };

    let result = client.encrypt(encrypt_request).await?;
    Ok(base64::encode(&result.ciphertext_blob.unwrap()))
}

async fn decrypt_data(ciphertext: &str) -> Result<String, rusoto_kms::Error> {
    let client = KmsClient::new(Region::UsEast1); // Choose the appropriate region

    let decrypt_request = DecryptRequest {
        ciphertext_blob: base64::decode(ciphertext).unwrap(),
        ..Default::default()
    };

    let result = client.decrypt(decrypt_request).await?;
    Ok(String::from_utf8(result.plaintext.unwrap())?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let plaintext = "Sensitive data";

    // Encrypt the data
    let encrypted_data = encrypt_data(plaintext).await?;
    println!("Encrypted data: {}", encrypted_data);

    // Decrypt the data
    let decrypted_data = decrypt_data(&encrypted_data).await?;
    println!("Decrypted data: {}", decrypted_data);

    Ok(())
}
