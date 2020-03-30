use hex;
use hmac::{Hmac, Mac};
use rusoto_core::Region;
use rusoto_secretsmanager::{GetSecretValueRequest, SecretsManager, SecretsManagerClient};
use sha2::Sha256;

// Create alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;

pub(crate) async fn verify_hmac(ts: &str, body: &str, sig: &str) -> Result<(), String> {
    let secret = {
        let secrets_manager = SecretsManagerClient::new(Region::UsWest1);
        secrets_manager
            .get_secret_value(GetSecretValueRequest {
                secret_id: "SlackSigningSecret79196F0B-7FO1CvOAvN9b".to_string(),
                version_id: None,
                version_stage: None,
            })
            .await
            .map_err(|e| format!("{:?}", e))?
            .secret_string
            .unwrap()
    };

    let sig = hex::decode(&sig[3..]).unwrap();

    let mut mac = HmacSha256::new_varkey(secret.as_bytes()).unwrap();
    mac.input(format!("v0:{}:{}", ts, body).as_bytes());

    mac.verify(&sig[..]).map_err(|e| format!("{:?}", e))
}
