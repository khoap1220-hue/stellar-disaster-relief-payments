// Kevin737866 Disaster Relief: Supply Chain Partial Delivery Tracking (Issue #328)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShipmentDeliveryStatus {
    Pending,
    PartiallyDelivered,
    Delivered,
}

pub struct PartialDeliveryAccounting {
    pub declared_quantity: u64,
    pub delivered_quantity: u64,
}

impl PartialDeliveryAccounting {
    pub fn new(declared: u64) -> Self {
        Self { declared_quantity: declared, delivered_quantity: 0 }
    }

    pub fn record_delivery(&mut self, received: u64) -> Result<ShipmentDeliveryStatus, &'static str> {
        if received == 0 {
            return Err("Received quantity must be positive");
        }
        if self.delivered_quantity + received > self.declared_quantity {
            return Err("Received quantity exceeds remaining shipment balance");
        }
        self.delivered_quantity += received;
        if self.delivered_quantity == self.declared_quantity {
            Ok(ShipmentDeliveryStatus::Delivered)
        } else {
            Ok(ShipmentDeliveryStatus::PartiallyDelivered)
        }
    }
}
