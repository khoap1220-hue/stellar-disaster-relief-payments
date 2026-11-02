// Kevin737866 Disaster Relief: Recipient Confirmation Binding (Issue #327)
pub struct ShipmentAuthorization {
    pub shipment_id: u64,
    pub intended_recipient_id: u64,
}

impl ShipmentAuthorization {
    pub fn assert_authorized(&self, caller_id: u64) -> Result<(), &'static str> {
        if caller_id != self.intended_recipient_id {
            return Err("Caller is not the designated recipient of this relief shipment");
        }
        Ok(())
    }
}
