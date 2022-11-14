// use chrono::prelude::*;
use rand::thread_rng;
use sha2::{Digest, Sha256};

use rsa::{
    pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey},
    pkcs1v15::{SigningKey, VerifyingKey},
    pkcs8::{DecodePrivateKey, DecodePublicKey},
    RsaPrivateKey, RsaPublicKey,
};
use signature::{DigestVerifier, RandomizedSigner, Signature};
use std::error::Error;

// pub const REQUEST_TARGET: &str = "(request-target)";

pub fn sign_string_with_private_key(
    private_key: RsaPrivateKey,
    to_sign: &String,
) -> Result<String, Box<dyn Error>> {
    let signing_key = SigningKey::<Sha256>::new_with_prefix(private_key);
    let signature = signing_key.try_sign_with_rng(thread_rng(), to_sign.as_bytes())?;

    Ok(base64::encode(signature.as_ref()))
}

pub fn parse_private_key(private_key_string: &String) -> Result<RsaPrivateKey, Box<dyn Error>> {
    if let Ok(pk) = RsaPrivateKey::from_pkcs1_pem(private_key_string) {
        Ok(pk)
    } else if let Ok(pk) = RsaPrivateKey::from_pkcs8_pem(private_key_string) {
        Ok(pk)
    } else {
        Err(Box::from("private key parsing failed"))
    }
}

pub fn verify_signature_with_signing_string_and_public_key(
    public_key: RsaPublicKey,
    signature: &String,
    signing_string: &String,
) -> Result<(), Box<dyn Error>> {
    let decoded = base64::decode(signature)?;
    let signature = Signature::from_bytes(&decoded)?;
    let hashed = Sha256::new_with_prefix(signing_string.as_bytes());
    let verifying_key = VerifyingKey::new_with_prefix(public_key);
    verifying_key.verify_digest(hashed, &signature)?;

    Ok(())
}

pub fn parse_public_key(public_key_string: &String) -> Result<RsaPublicKey, Box<dyn Error>> {
    if let Ok(pk) = RsaPublicKey::from_pkcs1_pem(public_key_string) {
        Ok(pk)
    } else if let Ok(pk) = RsaPublicKey::from_public_key_pem(public_key_string) {
        Ok(pk)
    } else {
        Err(Box::from("public key parsing failed"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger;
    use log::*;

    #[test]
    fn test_roundtrip_signing_and_verifying() {
        let mut previous_signature: Option<String> = None;

        let cases = vec![
            (PRIVATE_KEY_PKCS1_PEM, PUBLIC_KEY_PKCS1_PEM),
            (PRIVATE_KEY_PKCS1_PEM, PUBLIC_KEY_PKCS8_PEM),
            (PRIVATE_KEY_PKCS8_PEM, PUBLIC_KEY_PKCS1_PEM),
            (PRIVATE_KEY_PKCS8_PEM, PUBLIC_KEY_PKCS8_PEM),
        ];
        for case in cases {
            let (private_key_str, public_key_str) = case;

            let private_key = parse_private_key(&private_key_str.to_string())
                .expect("private key parsing to complete without error");

            let signature = sign_string_with_private_key(private_key, &TO_SIGN.to_string())
                .expect("signing to complete without error");
            match &previous_signature {
                None => previous_signature = Some(signature.clone()),
                Some(previous_signature) => {
                    assert_eq!(&signature, previous_signature);
                    assert_eq!(&signature, EXPECTED_SIGNATURE);
                }
            }

            let public_key = parse_public_key(&public_key_str.to_string())
                .expect("public key parsing to complete without error");

            let result = verify_signature_with_signing_string_and_public_key(
                public_key,
                &signature,
                &TO_SIGN.to_string(),
            );
            assert!(result.is_ok());
        }
    }

    const TO_SIGN: &str = "(request-target): post /inbox\nhost: toot.example.com\ndate: Mon, 14 Nov 2022 03:08:11 GMT";

    const EXPECTED_SIGNATURE: &str = "Mot+5x0SVIKbmFk3BxM0gtbYqMtSBN8GPNry+ZDatAGt/2apaflVTCFe6E1WP0fTGgPLQNT72iEeJ9s0Qoc29vp47JVxyKZWA2NMUfTvDSJ3EmiZLcM+FnfrkSFp4Cen+oacBcspww2Gvj2SNbf76h1KZpl8ceBr77HRpSchrHZMzYmpfzmQWNZwhPAM4LQGhxegUcXYBlXc9Ya0UkdBfCOHJ4jcHiScUKRz3/xnLKzLZAXpvT2ttBdURC/PZmw0W+3PPyQA7V4+eRpqsezGsSyAHqQDQ7J2HCfu4QLawgyuhz5D4qTx960i99DgYSCs3d+ebbtih7mNUkZuclHtBQ==";

    /*
        # This is convoluted, but this seems to work for generating all these key variants
        # Only need a pkcs1 or pkcs8 pair, but have encountered both in the wild
        openssl genrsa -out private-pkcs1.pem 2048
        openssl pkcs8 -topk8 -inform PEM -in private-pkcs1.pem -outform pem -nocrypt -out private-pkcs8.pem
        openssl rsa -in private-pkcs1.pem -outform PEM -pubout -out public-pkcs8.pem
        openssl rsa -pubin -in public-pkcs8.pem -RSAPublicKey_out -out public-pkcs1.pem
    */

    const PUBLIC_KEY_PKCS1_PEM: &str = r#"-----BEGIN RSA PUBLIC KEY-----
MIIBCgKCAQEA4hNT+ovmGp8mn71J7mSZhlpr5Hv4b4wDEpM4OdoWYfHD7oJ6iFG1
G69XE37ICz/V+TcWWwLuKs2PAIdTdZI3epp1wpuUV0d56Zp85Un4AdSAVdcpR+mU
ebra1hxRwZAyeo/Kql0d0MlWrmNyBFVOXEE0uNJTZQCDGVHIOGcyCm5MdJQ4vpkY
as9IRNMGAz75Nyf3kVkHVu0Lp0mtSzPDXWVyOZU5uLarn5qUYRTxtqdMXkUpIfvd
D4dfQBPm/jUWl6ZGpxCIgJZjNV/zMLdAM5jzMHkPJjnCOrJlK01I5KaANyiawXRq
eA70o/tW1qM99znjBOA2I7R7vI1wS30E2QIDAQAB
-----END RSA PUBLIC KEY-----"#;

    const PRIVATE_KEY_PKCS1_PEM: &str = r#"-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEA4hNT+ovmGp8mn71J7mSZhlpr5Hv4b4wDEpM4OdoWYfHD7oJ6
iFG1G69XE37ICz/V+TcWWwLuKs2PAIdTdZI3epp1wpuUV0d56Zp85Un4AdSAVdcp
R+mUebra1hxRwZAyeo/Kql0d0MlWrmNyBFVOXEE0uNJTZQCDGVHIOGcyCm5MdJQ4
vpkYas9IRNMGAz75Nyf3kVkHVu0Lp0mtSzPDXWVyOZU5uLarn5qUYRTxtqdMXkUp
IfvdD4dfQBPm/jUWl6ZGpxCIgJZjNV/zMLdAM5jzMHkPJjnCOrJlK01I5KaANyia
wXRqeA70o/tW1qM99znjBOA2I7R7vI1wS30E2QIDAQABAoIBAEc5PaDWiFTkxP9Y
XD1dtjxsqkceg2NpIeKtPO6E9b4/s7Glq8LcswkY0X8T1yQsJVZRc5qSrsQPZLkO
6U1GFnJqzSTVbQOeQhtFj4mRJzA0aYYhtiCwxxUeuCjyXRN1QWH5gSjxEx6e/88L
B3W0Bm7sIBcGND3CzoqgiEq0wEAmbf8YPfDtrOb+m620+MwlJibHtg/PUEyGnxN9
qnQasbqZR4gdksL4VJ7s0HWcOwt5T1AnfarXqZ5hfGAEPuWbVh0W4lngMK4cVL+2
mWncem6kwAV09nQwzWIB0QbaFkO+zh2LhhE8ot0si2L4i6DmkYT9snpAb9B/Ea+6
LyoikAECgYEA+hhwF1sG4ArgRnT9rdlRMO6Ib48YTgU/HkzqNHqMMLpDl5ShToEI
qVd2CPQpjxzVBj1b+RAQSFQJGzDQWt82UTupAwB5GDhq/GRuT4kQuzSpV3cKAScC
wTmkzcJL7k+AjUqb7uhkXOgyDEr1CU8IzuaZJzJALiUZi+qAMI/vGgECgYEA52m3
CDeSfr4YzsyDymSzRK7EWHcIiwxiCdgjXN7sg0JL+LTvBdTI9O/7MIYeLOFPG9FZ
f8pvv1v4iM4mSXqVPCkHpim3L26UyBXA4p+H0PMxI/+ACtInctc6qTgXGZ1OS/GZ
rro3ykm0dJkA9sG25YEx22DqeS9Vlmb2zuxr+tkCgYEA4vVec7OFr7fvA8GTnTsD
9VXituqy8crNFuBSAkHcCzRdtqeJI0bx50uZsFJzjI5ru3ffiUzMmfa5NNW9n1nF
Xz63CCRbwGEipd02RjNq3ZjSvK8ogxc15sg7CQb0BVZcoNw+WjbHLZECKimz6Tiy
E6EtB2cQ20+LuW+b2XYlVAECgYBlYuqe/gH/53dk+zNudAoauFsryxMCIK2/VGPB
56VFqMEyOMtUCbL1pUKvMsN7tEb9kA4fL+kftMLB5Vfe74b8sZk5UqOAc3lZ5DX6
1BwkDrhj5igKFaLU7Lk1tG5ieYn5OeO0KsAQr5QRVYuXkK16Bc17KQ3xhCyCVaAi
bnCsmQKBgQCRYFSWWasXViyCLNKl/AH0O0du/dDC/h5uNNoNw6NSjedlOUhYbePW
WpPuJXyX/8lYMgl7mChNoRdVfOuI0c0SL5FjZWS41TfcGglbqVO6QZj96cKaU4va
AZDO63yR17oHr9HUNilLqJi4ECSGPCdqGw7ZG6TbWZ2wM6sEV44GCw==
-----END RSA PRIVATE KEY-----"#;

    const PUBLIC_KEY_PKCS8_PEM: &str = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA4hNT+ovmGp8mn71J7mSZ
hlpr5Hv4b4wDEpM4OdoWYfHD7oJ6iFG1G69XE37ICz/V+TcWWwLuKs2PAIdTdZI3
epp1wpuUV0d56Zp85Un4AdSAVdcpR+mUebra1hxRwZAyeo/Kql0d0MlWrmNyBFVO
XEE0uNJTZQCDGVHIOGcyCm5MdJQ4vpkYas9IRNMGAz75Nyf3kVkHVu0Lp0mtSzPD
XWVyOZU5uLarn5qUYRTxtqdMXkUpIfvdD4dfQBPm/jUWl6ZGpxCIgJZjNV/zMLdA
M5jzMHkPJjnCOrJlK01I5KaANyiawXRqeA70o/tW1qM99znjBOA2I7R7vI1wS30E
2QIDAQAB
-----END PUBLIC KEY-----"#;

    const PRIVATE_KEY_PKCS8_PEM: &str = r#"-----BEGIN PRIVATE KEY-----
MIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQDiE1P6i+Yanyaf
vUnuZJmGWmvke/hvjAMSkzg52hZh8cPugnqIUbUbr1cTfsgLP9X5NxZbAu4qzY8A
h1N1kjd6mnXCm5RXR3npmnzlSfgB1IBV1ylH6ZR5utrWHFHBkDJ6j8qqXR3QyVau
Y3IEVU5cQTS40lNlAIMZUcg4ZzIKbkx0lDi+mRhqz0hE0wYDPvk3J/eRWQdW7Qun
Sa1LM8NdZXI5lTm4tqufmpRhFPG2p0xeRSkh+90Ph19AE+b+NRaXpkanEIiAlmM1
X/Mwt0AzmPMweQ8mOcI6smUrTUjkpoA3KJrBdGp4DvSj+1bWoz33OeME4DYjtHu8
jXBLfQTZAgMBAAECggEARzk9oNaIVOTE/1hcPV22PGyqRx6DY2kh4q087oT1vj+z
saWrwtyzCRjRfxPXJCwlVlFzmpKuxA9kuQ7pTUYWcmrNJNVtA55CG0WPiZEnMDRp
hiG2ILDHFR64KPJdE3VBYfmBKPETHp7/zwsHdbQGbuwgFwY0PcLOiqCISrTAQCZt
/xg98O2s5v6brbT4zCUmJse2D89QTIafE32qdBqxuplHiB2SwvhUnuzQdZw7C3lP
UCd9qtepnmF8YAQ+5ZtWHRbiWeAwrhxUv7aZadx6bqTABXT2dDDNYgHRBtoWQ77O
HYuGETyi3SyLYviLoOaRhP2yekBv0H8Rr7ovKiKQAQKBgQD6GHAXWwbgCuBGdP2t
2VEw7ohvjxhOBT8eTOo0eowwukOXlKFOgQipV3YI9CmPHNUGPVv5EBBIVAkbMNBa
3zZRO6kDAHkYOGr8ZG5PiRC7NKlXdwoBJwLBOaTNwkvuT4CNSpvu6GRc6DIMSvUJ
TwjO5pknMkAuJRmL6oAwj+8aAQKBgQDnabcIN5J+vhjOzIPKZLNErsRYdwiLDGIJ
2CNc3uyDQkv4tO8F1Mj07/swhh4s4U8b0Vl/ym+/W/iIziZJepU8KQemKbcvbpTI
FcDin4fQ8zEj/4AK0idy1zqpOBcZnU5L8ZmuujfKSbR0mQD2wbblgTHbYOp5L1WW
ZvbO7Gv62QKBgQDi9V5zs4Wvt+8DwZOdOwP1VeK26rLxys0W4FICQdwLNF22p4kj
RvHnS5mwUnOMjmu7d9+JTMyZ9rk01b2fWcVfPrcIJFvAYSKl3TZGM2rdmNK8ryiD
FzXmyDsJBvQFVlyg3D5aNsctkQIqKbPpOLIToS0HZxDbT4u5b5vZdiVUAQKBgGVi
6p7+Af/nd2T7M250Chq4WyvLEwIgrb9UY8HnpUWowTI4y1QJsvWlQq8yw3u0Rv2Q
Dh8v6R+0wsHlV97vhvyxmTlSo4BzeVnkNfrUHCQOuGPmKAoVotTsuTW0bmJ5ifk5
47QqwBCvlBFVi5eQrXoFzXspDfGELIJVoCJucKyZAoGBAJFgVJZZqxdWLIIs0qX8
AfQ7R2790ML+Hm402g3Do1KN52U5SFht49Zak+4lfJf/yVgyCXuYKE2hF1V864jR
zRIvkWNlZLjVN9waCVupU7pBmP3pwppTi9oBkM7rfJHXugev0dQ2KUuomLgQJIY8
J2obDtkbpNtZnbAzqwRXjgYL
-----END PRIVATE KEY-----"#;
}
