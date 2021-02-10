use openssl::error::ErrorStack;
#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    /// The request specified an account that does not exist
    AccountDoesNotExist,
    /// The request specified a certificate to be revoked
    /// that has already been revoked
    AlreadyRevoked,
    /// The CSR is unacceptable (e.g., due to a short key)
    BadCSR,
    /// The client sent an unacceptable anti-replay nonce     
    BadNonce,
    ///  The JWS was signed by a public key the server does
    /// not support
    BadPublicKey,
    /// The revocation reason provided is not allowed by
    /// the server  
    BadRevocationReason,
    /// The JWS was signed with an algorithm the server does
    /// not support
    BadSignatureAlgorithm,
    /// Certification Authority Authorization (CAA) records
    /// forbid the CA from issuing a certificate  
    Caa,
    /// Specific error conditions are indicated in the
    /// "subproblems" array
    Compound,
    /// The server could not connect to validation target  
    Connection,
    /// There was a problem with a DNS query during
    /// identifier validation
    Dns,
    /// The request must include a value for the
    /// "externalAccountBinding" field
    ExternalAccountRequired,
    /// Response received didn't match the
    /// challenge's requirements   
    IncorrectResponse,
    /// A contact URL for an account was invalid       
    InvalidContact,
    /// The request message was malformed
    Malformed,
    /// The request attempted to finalize an order that is
    /// not ready to be finalized
    OrderNotReady,
    /// The request exceeds a rate limit
    RateLimited,
    /// The server will not issue certificates for the identifier
    RejectedIdentifier,
    /// The server experienced an internal error   
    ServerInternal,
    /// The server received a TLS error during validation   
    Tls,
    /// The client lacks sufficient authorization
    Unauthorized,
    /// A contact URL for an account used an unsupported protocol scheme
    UnsupportedContact,
    /// An identifier is of an unsupported type
    UnsupportedIdentifier,
    /// Visit the "instance" URL and take actions specified there
    UserActionRequired,
    /// Error converted from an Utf8 error
    FromUtf8Error(std::str::Utf8Error),
    /// Error converted from a reqwest error
    FromReqwestError(reqwest::Error),
    /// Error converted from an Url parse error
    FromUrlParseError(url::ParseError),
    /// Error converted from an ErrorStack Error (occurs during RSA generation)
    FromRsaError(ErrorStack),
    /// Error converted from a json serde error
    FromSerdeError(serde_json::Error),
}

impl From<std::str::Utf8Error> for Error {
    fn from(error: std::str::Utf8Error) -> Self {
        Error::FromUtf8Error(error)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::FromReqwestError(error)
    }
}

impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Self {
        Error::FromUrlParseError(error)
    }
}

impl From<ErrorStack> for Error {
    fn from(error: ErrorStack) -> Self {
        Error::FromRsaError(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::FromSerdeError(error)
    }
}