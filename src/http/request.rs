use crate::error::ServerError;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub version: String,
}

impl HttpRequest {
    pub fn parse_line(request_line: &str) -> Result<Self, ServerError> {
        let parts: Vec<&str> = request_line.split_whitespace().collect();

        if parts.len() != 3 {
            return Err(ServerError::InvalidRequest(
                "La línea de petición no tiene 3 componentes".into(),
            ));
        }

        Ok(Self {
            method: parts[0].to_string(),
            path: parts[1].to_string(),
            version: parts[2].to_string(),
        })
    }
}