#[derive(Clone, Debug, PartialEq)]
pub enum QuantumixException {
    ColumnNotFound(Option<String>),
    CreateFieldFailed(Option<String>),
    PasswordHashFailed(Option<String>),
    ColumnExists(Option<String>),
}

impl QuantumixException {
    fn write_error(
        f: &mut core::fmt::Formatter,
        name: &str,
        info: &Option<String>,
    ) -> core::fmt::Result {
        if info.is_none() {
            let info = "Unknown";
            f.write_str(&format!("quantumix::exceptions::{}: {}", name, info))
        } else {
            let info = info.clone().unwrap();
            f.write_str(&format!("quantumix::exceptions::{}: {}", name, info))
        }
    }
}

impl core::fmt::Display for QuantumixException {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ColumnNotFound(info) => {
                QuantumixException::write_error(f, "ColumnNotFound", info)
            }
            Self::CreateFieldFailed(info) => {
                QuantumixException::write_error(f, "CreateFieldFailed", info)
            }
            Self::PasswordHashFailed(info) => {
                QuantumixException::write_error(f, "PasswordHashFailed", info)
            }
            Self::ColumnExists(info) => QuantumixException::write_error(f, "ColumnExists", info),
        }
    }
}
