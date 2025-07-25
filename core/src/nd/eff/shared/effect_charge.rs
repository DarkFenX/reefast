use crate::ad;

// Specifies where effect should look for charge
#[derive(Copy, Clone)]
pub(crate) struct NEffectCharge {
    pub(crate) location: NEffectChargeLoc,
    // When true, controlled charge state is switched to active when effect is running
    pub(crate) activates_charge: bool,
}

// Specifies where effect should look for charge
#[derive(Copy, Clone)]
pub(crate) enum NEffectChargeLoc {
    // Effect uses charge loaded into item
    Loaded(NEffectChargeDepl),
    // Effect uses charge referenced by an attribute on effect item, which is automatically loaded
    // into containing item
    Autocharge(ad::AAttrId),
}

// Charge depletion mode
#[derive(Copy, Clone)]
pub(crate) enum NEffectChargeDepl {
    // Charge is not depleted if loaded
    None,
    // Each module cycle removes chargeRate attr value count from count of loaded charges
    ChargeRate { can_run_uncharged: bool },
    // Only 1 charge is loaded, and used until it is destroyed
    Crystal,
}
