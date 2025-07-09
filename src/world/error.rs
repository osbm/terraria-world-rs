// Custom error for invalid footer
#[derive(Debug)]
pub struct InvalidFooterError(pub String);

impl std::fmt::Display for InvalidFooterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid footer: {}", self.0)
    }
}

impl std::error::Error for InvalidFooterError {} 