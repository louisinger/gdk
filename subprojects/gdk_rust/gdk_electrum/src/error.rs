use serde::ser::{Serialize, SerializeStruct};
use std::convert::From;

#[derive(Debug)]
pub enum Error {
    Generic(String),
    UnknownCall,
    InvalidMnemonic,
    DB(sled::Error),
    AddrParse(String),
    Bitcoin(bitcoin::util::Error),
    BitcoinBIP32Error(bitcoin::util::bip32::Error),
    BitcoinConsensus(bitcoin::consensus::encode::Error),
    JSON(serde_json::error::Error),
    StdIOError(std::io::Error),
    Hex(hex::FromHexError),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Error", 1)?;
        match &self {
            Error::Generic(ref strerr) => {
                s.serialize_field("error", strerr)?;
            }
            // TODO: implement serialization of these errors
            Error::UnknownCall => {}
            Error::AddrParse(ref addr) => {
                s.serialize_field("error", &format!("could not parse SocketAddr `{}`", addr))?
            }
            Error::InvalidMnemonic => s.serialize_field("error", "invalid mnemonic")?,
            Error::DB(ref _dberr) => {}
            Error::Bitcoin(ref _btcerr) => {}
            Error::BitcoinBIP32Error(ref _bip32err) => {}
            Error::BitcoinConsensus(ref _consensus_err) => {}
            Error::JSON(ref _json_err) => {}
            Error::StdIOError(ref _io_err) => {}
            Error::Hex(ref _hex_err) => {}
        }

        s.end()
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(_err: std::net::AddrParseError) -> Self {
        Error::AddrParse("SocketAddr parse failure with no additional info".into())
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Self {
        Error::JSON(err)
    }
}

impl std::convert::From<bitcoin::util::bip32::Error> for Error {
    fn from(err: bitcoin::util::bip32::Error) -> Self {
        Error::BitcoinBIP32Error(err)
    }
}

impl std::convert::From<String> for Error {
    fn from(err: String) -> Self {
        Error::Generic(err)
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::StdIOError(err)
    }
}

impl std::convert::From<sled::Error> for Error {
    fn from(err: sled::Error) -> Self {
        Error::DB(err)
    }
}

impl std::convert::From<bitcoin::consensus::encode::Error> for Error {
    fn from(err: bitcoin::consensus::encode::Error) -> Self {
        Error::BitcoinConsensus(err)
    }
}

impl std::convert::From<hex::FromHexError> for Error {
    fn from(err: hex::FromHexError) -> Self {
        Error::Hex(err)
    }
}