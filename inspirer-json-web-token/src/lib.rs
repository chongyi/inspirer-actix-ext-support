#[macro_use]
extern crate serde;

pub use jsonwebtoken::*;
use jsonwebtoken::errors::Result;
use serde::Serialize;

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct PublicClaims {
    #[serde(skip_serializing_if = "Option::is_none")]
    aud: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iss: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub: Option<String>,
    exp: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    nbf: Option<usize>,
}

impl PublicClaims {
    pub fn exp(exp: usize) -> Self {
        PublicClaims {
            exp,
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims<T> {
    #[serde(flatten)]
    pub public_claims: PublicClaims,
    #[serde(flatten)]
    pub private_claims: T,
}

impl<T> Claims<T>
where
    T: Serialize,
{
    pub fn build_jwt_token(self) -> JwtTokenBuilder<T> {
        JwtTokenBuilder::new(self)
    }
}

pub struct JwtTokenBuilder<T> {
    header: Header,
    encode_key: EncodingKey,
    claims: Claims<T>,
}

impl<T> JwtTokenBuilder<T>
where
    T: Serialize,
{
    pub fn new(claims: Claims<T>) -> Self {
        JwtTokenBuilder {
            header: Header::default(),
            encode_key: EncodingKey::from_secret("secret".as_bytes()),
            claims,
        }
    }

    pub fn header(mut self, header: Header) -> Self {
        self.header = header;
        self
    }

    pub fn encode_key(mut self, key: EncodingKey) -> Self {
        self.encode_key = key;
        self
    }

    pub fn build(&self) -> Result<String> {
        encode(&self.header, &self.claims, &self.encode_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_token_build() {
        #[derive(Serialize)]
        pub struct Test {
            username: String,
        }

        let token = Claims {
            public_claims: PublicClaims::default(),
            private_claims: Test {
                username: "admin".into()
            }
        }.build_jwt_token()
        .build()
        .unwrap();

        assert_eq!("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjAsInVzZXJuYW1lIjoiYWRtaW4ifQ.mmbgq2yGAclAujyEO9Zp1csGf9-CIpA_--20U1lZtXE", token);
    }
}
