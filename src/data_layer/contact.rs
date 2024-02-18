#[derive(Debug)]
pub struct Contact {
    pub id: u64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company_name: Option<String>,
    pub phone_number: String,
}

impl std::fmt::Display for Contact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let first_name = self.first_name.as_deref().unwrap_or("N/A");
        let last_name = self.last_name.as_deref().unwrap_or("N/A");
        let company = self.company_name.as_deref().unwrap_or("N/A");

        write!(
            f,
            "#{} - {first_name} {last_name} - {company} - {}",
            self.id, self.phone_number
        )
    }
}

pub struct ContactForUpdate {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company_name: Option<String>,
    pub phone_number: String,
}
