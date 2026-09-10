use uuid::Uuid;

pub fn parse_uuid(s: &str) -> Option<Uuid> {
    Uuid::parse_str(s).ok()
}
