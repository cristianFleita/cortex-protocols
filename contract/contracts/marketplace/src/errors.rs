use soroban_sdk::contracterror;

/// Marketplace contract error codes.
#[contracterror]
#[derive(Clone, Debug, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum MarketplaceError {
    /// Caller is not the asset owner
    NotOwner = 1,
    /// Asset does not exist
    AssetNotFound = 2,
    /// Asset is not currently active
    AssetInactive = 3,
    /// Buyer cannot purchase their own asset
    SelfPurchase = 4,
    /// Price must be greater than zero
    InvalidPrice = 5,
    /// Maximum asset limit reached
    AssetLimitReached = 6,
    /// License already exists for this buyer
    LicenseAlreadyExists = 7,
}
