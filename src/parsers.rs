use regex::Regex;

fn validate_regex(value: &str, reg_exp: &Regex) -> Result<String, String> {
    match reg_exp.is_match(value) {
        true => Ok(value.to_string()),
        false => Err(String::from("Failed to validate regex")),
    }
}

pub fn parse_slug(value: &str) -> Result<String, String> {
    let github_slug_regex = Regex::new(r"^[a-zA-Z0-9-]{0,38}$").unwrap();

    let result = validate_regex(value, &github_slug_regex).map_err(|_| 
        "slug must only contain alphanumeric characters and/or hyphens"
    )?;

    Ok(result)
}

pub fn parse_year(value: &str) -> Result<String, String> {
    let year_regex = Regex::new(r"^[\d]{4}$").unwrap();

    let result = validate_regex(value, &year_regex).map_err(|_|
        "year must be a valid calendar year, e.g. 2022"
    )?;
    
    Ok(result)
}
