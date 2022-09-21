use scraper::ElementRef;

const LEVEL_ATTR: &str = "data-level";

#[derive(Debug)]
pub struct Contribution {
    pub heat_level: usize
}

impl Contribution {
    pub fn from_el(el: &ElementRef) -> Self {
       let heat_level = el.value().attr(LEVEL_ATTR).unwrap().parse().unwrap();
       Contribution { heat_level }
    }
}
