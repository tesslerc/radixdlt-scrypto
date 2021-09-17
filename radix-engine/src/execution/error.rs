use sbor::*;
use scrypto::rust::fmt;
use scrypto::types::*;
use wasmi::*;

use crate::model::*;

/// Represents an error occurred during transaction execution.
#[derive(Debug)]
pub enum RuntimeError {
    InvalidModule(Error),

    UnableToInstantiate(Error),

    StartFunctionNotAllowed,

    FloatingPointNotAllowed,

    NoValidMemoryExport,

    InvokeError(Error),

    MemoryAccessError(Error),

    NoReturnValue,

    InvalidReturnType,

    InvalidOpCode(u32),

    InvalidRequest(DecodeError),

    InvalidData(DecodeError),

    UnknownHostFunction(usize),

    UnableToAllocateMemory,

    ResourceLeak,

    PackageAlreadyExists(Address),

    ComponentAlreadyExists(Address),

    ResourceAlreadyExists(Address),

    PackageNotFound(Address),

    ComponentNotFound(Address),

    StorageNotFound(SID),

    ResourceNotFound(Address),

    FixedResourceMintNotAllowed,

    UnauthorizedToMint,

    BucketNotFound,

    ReferenceNotFound,

    AccountingError(BucketError),

    UnauthorizedToWithdraw,

    PersistentBucketMoveNotAllowed,

    ReferenceNotAllowed,

    VmNotStarted,

    InvalidLogLevel,

    UnexpectedResourceReturn,

    UnauthorizedAccess,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl HostError for RuntimeError {}
