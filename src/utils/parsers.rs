use regex::Regex;

fn validate_regex(value: &str, reg_exp: &Regex) -> Result<String, String> {
    match reg_exp.is_match(value) {
        true => Ok(value.to_string()),
        false => Err(String::from("Failed to validate regex")),
    }
}

/// Attempts to parse a Github profile slug, based on a regular expression
/// typically used while restricting profile names. Assures that only
/// valid profile slugs are being provided as an argument to Clap.
///
/// # Errors
/// Returns an error if provided slug argument does not match regular
/// expression.
///
pub fn parse_slug(value: &str) -> Result<String, String> {
    let github_slug_regex = Regex::new(r"^[a-zA-Z0-9-]{0,38}$").unwrap();

    let result = validate_regex(value, &github_slug_regex).map_err(|_| 
        "slug must only contain alphanumeric characters and/or hyphens"
    )?;

    Ok(result)
}

/// Attempts to parse a provided Year argument, based on simple regular
/// expression. Assures that only valid calendar years are being provided
/// as an argument to clap.
///
/// # Errors
/// Returns an error if provided year argument does not match regular
/// expression.
///
pub fn parse_year(value: &str) -> Result<String, String> {
    let year_regex = Regex::new(r"^[\d]{4}$").unwrap();

    let result = validate_regex(value, &year_regex).map_err(|_|
        "year must be a valid calendar year, e.g. 2022"
    )?;
    
    Ok(result)
}
