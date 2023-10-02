use serde::Serialize;

#[derive(Debug, Serialize)]
#[repr(i32)]
pub(crate) enum FailureCode {
    IncorrectOrUnknownPaymentDetails = 1,
    IncorrectPaymentAmount = 2,
    FinalIncorrectCltvExpiry = 3,
    FinalIncorrectHtlcAmount = 4,
    FinalExpiryTooSoon = 5,
    InvalidRealm = 6,
    ExpiryTooSoon = 7,
    InvalidOnionVersion = 8,
    InvalidOnionHmac = 9,
    InvalidOnionKey = 10,
    AmountBelowMinimum = 11,
    FeeInsufficient = 12,
    IncorrectCltvExpiry = 13,
    ChannelDisabled = 14,
    TemporaryChannelFailure = 15,
    RequiredNodeFeatureMissing = 16,
    RequiredChannelFeatureMissing = 17,
    UnknownNextPeer = 18,
    TemporaryNodeFailure = 19,
    PermanentNodeFailure = 20,
    PermanentChannelFailure = 21,
    ExpiryTooFar = 22,
    MppTimeout = 23,
    InvalidOnionPayload = 24,
    InternalFailure = 997,
    UnknownFailure = 998,
    UnreadableFailure = 999,
    UnknownFailureCode(i32), // for handling unknown failure codes
}

impl From<i32> for FailureCode {
    fn from(code: i32) -> Self {
        match code {
            1 => Self::IncorrectOrUnknownPaymentDetails,
            2 => Self::IncorrectPaymentAmount,
            3 => Self::FinalIncorrectCltvExpiry,
            4 => Self::FinalIncorrectHtlcAmount,
            5 => Self::FinalExpiryTooSoon,
            6 => Self::InvalidRealm,
            7 => Self::ExpiryTooSoon,
            8 => Self::InvalidOnionVersion,
            9 => Self::InvalidOnionHmac,
            10 => Self::InvalidOnionKey,
            11 => Self::AmountBelowMinimum,
            12 => Self::FeeInsufficient,
            13 => Self::IncorrectCltvExpiry,
            14 => Self::ChannelDisabled,
            15 => Self::TemporaryChannelFailure,
            16 => Self::RequiredNodeFeatureMissing,
            17 => Self::RequiredChannelFeatureMissing,
            18 => Self::UnknownNextPeer,
            19 => Self::TemporaryNodeFailure,
            20 => Self::PermanentNodeFailure,
            21 => Self::PermanentChannelFailure,
            22 => Self::ExpiryTooFar,
            23 => Self::MppTimeout,
            24 => Self::InvalidOnionPayload,
            997 => Self::InternalFailure,
            998 => Self::UnknownFailure,
            999 => Self::UnreadableFailure,
            _ => Self::UnknownFailureCode(code),
        }
    }
}

// Define an enum for the FailureReason
#[derive(Debug, PartialEq)]

pub enum FailureReason {
    None,
    Timeout,
    NoRoute,
    Error,
    IncorrectPaymentDetails,
    InsufficientBalance,
    Unknown(i32),
}

impl From<i32> for FailureReason {
    fn from(value: i32) -> Self {
        match value {
            0 => FailureReason::None,
            1 => FailureReason::Timeout,
            2 => FailureReason::NoRoute,
            3 => FailureReason::Error,
            4 => FailureReason::IncorrectPaymentDetails,
            5 => FailureReason::InsufficientBalance,
            _ => FailureReason::Unknown(value),
        }
    }
}
