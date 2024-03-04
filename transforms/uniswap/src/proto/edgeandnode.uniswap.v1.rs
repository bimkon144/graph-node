#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<Event>,
}
/// Every address is stored as hex string.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// Owner points to the address that originated this event
    /// The PoolCreated will set this to factory, which is what we can use
    /// to track different factories with compatible events.
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(enumeration = "EventType", tag = "2")]
    pub r#type: i32,
    #[prost(message, optional, tag = "3")]
    pub event: ::core::option::Option<::prost_types::Any>,
    #[prost(string, tag = "4")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub tx_gas_used: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub tx_gas_price: ::prost::alloc::string::String,
    /// This duplicates data (as opposed to adding this data to the head) but AssemblyScript does
    /// not support closures and so using the data is not super easy if it's in the header so I'll
    /// leave it here.
    #[prost(int32, tag = "8")]
    pub block_number: i32,
    #[prost(string, tag = "9")]
    pub block_timestamp: ::prost::alloc::string::String,
    #[prost(oneof = "event::Event2", tags = "10, 11, 12, 13, 14, 15, 16, 17, 18, 19")]
    pub event2: ::core::option::Option<event::Event2>,
}
/// Nested message and enum types in `Event`.
pub mod event {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event2 {
        #[prost(message, tag = "10")]
        Poolcreated(super::PoolCreated),
        #[prost(message, tag = "11")]
        Increaseliquidity(super::IncreaseLiquidity),
        #[prost(message, tag = "12")]
        Decreaseliquidity(super::DecreaseLiquidity),
        #[prost(message, tag = "13")]
        Collect(super::Collect),
        #[prost(message, tag = "14")]
        Transfer(super::Transfer),
        #[prost(message, tag = "15")]
        Initialize(super::Initialize),
        #[prost(message, tag = "16")]
        Swap(super::Swap),
        #[prost(message, tag = "17")]
        Mint(super::Mint),
        #[prost(message, tag = "18")]
        Burn(super::Burn),
        #[prost(message, tag = "19")]
        Flash(super::Flash),
    }
}
/// Factory
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolCreated {
    #[prost(string, tag = "1")]
    pub token0: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub token1: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub fee: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub tick_spacing: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub pool: ::prost::alloc::string::String,
}
/// Position Manager
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncreaseLiquidity {
    #[prost(string, tag = "1")]
    pub token_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub liquidity: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub amount1: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecreaseLiquidity {
    #[prost(string, tag = "1")]
    pub token_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub liquidity: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub amount1: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Collect {
    #[prost(string, tag = "1")]
    pub token_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub amount1: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfer {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub token_id: ::prost::alloc::string::String,
}
/// Pool
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Initialize {
    #[prost(string, tag = "1")]
    pub sqrt_price_x96: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub tick: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Swap {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub amount1: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub sqrt_price_x96: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub liquidity: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub tick: ::prost::alloc::string::String,
    #[prost(int32, tag = "8")]
    pub log_index: i32,
    #[prost(string, tag = "9")]
    pub transaction_from: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mint {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub tick_lower: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub tick_upper: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub amount1: ::prost::alloc::string::String,
    #[prost(int32, tag = "8")]
    pub log_index: i32,
    #[prost(string, tag = "9")]
    pub transaction_from: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Burn {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub tick_lower: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub tick_upper: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub amount1: ::prost::alloc::string::String,
    #[prost(int32, tag = "7")]
    pub log_index: i32,
    #[prost(string, tag = "8")]
    pub transaction_from: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Flash {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub amount1: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub paid0: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub paid1: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EventType {
    /// Factory
    PoolCreated = 0,
    /// Position Manager
    IncreaseLiquidity = 1,
    DecreaseLiquidity = 2,
    Collect = 3,
    Transfer = 4,
    /// Pool
    Initialize = 5,
    Swap = 6,
    Mint = 7,
    Burn = 8,
    Flash = 9,
}
impl EventType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EventType::PoolCreated => "POOL_CREATED",
            EventType::IncreaseLiquidity => "INCREASE_LIQUIDITY",
            EventType::DecreaseLiquidity => "DECREASE_LIQUIDITY",
            EventType::Collect => "COLLECT",
            EventType::Transfer => "TRANSFER",
            EventType::Initialize => "INITIALIZE",
            EventType::Swap => "SWAP",
            EventType::Mint => "MINT",
            EventType::Burn => "BURN",
            EventType::Flash => "FLASH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "POOL_CREATED" => Some(Self::PoolCreated),
            "INCREASE_LIQUIDITY" => Some(Self::IncreaseLiquidity),
            "DECREASE_LIQUIDITY" => Some(Self::DecreaseLiquidity),
            "COLLECT" => Some(Self::Collect),
            "TRANSFER" => Some(Self::Transfer),
            "INITIALIZE" => Some(Self::Initialize),
            "SWAP" => Some(Self::Swap),
            "MINT" => Some(Self::Mint),
            "BURN" => Some(Self::Burn),
            "FLASH" => Some(Self::Flash),
            _ => None,
        }
    }
}
