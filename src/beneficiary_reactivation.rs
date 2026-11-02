// Kevin737866 Disaster Relief: Beneficiary Reactivation Workflow (Issue #326)
pub struct BeneficiaryRecord {
    pub beneficiary_id: u64,
    pub is_active: bool,
    pub reactivation_count: u32,
}

impl BeneficiaryRecord {
    pub fn reactivate(&mut self, is_admin: bool) -> Result<(), &'static str> {
        if !is_admin {
            return Err("Only authorized admin can reactivate a beneficiary");
        }
        if self.is_active {
            return Err("Beneficiary is already active");
        }
        self.is_active = true;
        self.reactivation_count += 1;
        Ok(())
    }
}
