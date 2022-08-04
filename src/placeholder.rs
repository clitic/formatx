use crate::format_spec::FormatSpec;

/// Template placeholder specifications.
#[derive(Debug)]
pub struct Placeholder {
    pub attributes: Option<String>,
    pub format_spec: FormatSpec,
    pub replacer: String,
}

impl Placeholder {
    /// Parse template string and deserialize it to `Self`.
    pub fn new(template: &str, placeholder: &str) -> Option<Self> {
        if let Some(start) = template.find(&format!("{{{}", placeholder)) {
            let matched = &template[(start + 1)..(start + template[start..].find("}").unwrap())];

            let attributes = if matched.rfind(":").is_some() {
                let mut attributes = matched.split(":").collect::<Vec<&str>>();
                let _ = attributes.pop();
                attributes
                    .join(":")
                    .trim_start_matches(placeholder)
                    .trim()
                    .to_owned()
            } else {
                matched.trim_start_matches(placeholder).trim().to_owned()
            };

            let attributes = if attributes == "" {
                None
            } else {
                Some(attributes)
            };

            let format_specs = if matched.rfind(":").is_some() {
                FormatSpec::new(matched.split(":").last().unwrap().to_owned())
            } else {
                FormatSpec::new("".to_owned())
            };

            return Some(Self {
                attributes,
                format_spec: format_specs,
                replacer: format!("{{{}}}", matched),
            });
        }

        None
    }

    /// Get attribute value if present else returns `None`.
    ///
    /// Attributes can be defined for as:
    ///
    /// ```text
    /// 1. name=emily -> Some("emily")
    /// 2. name="emily cooper" -> Some("emily cooper")
    /// 3. name='emily clarke' -> Some("emily clarke")
    /// ```
    pub fn attr(&self, attribute: &str) -> Option<String> {
        if let Some(attributes) = &self.attributes {
            let attribute_identifier = format!("{}=", attribute);

            if let Some(attribute_index) = attributes.find(&attribute_identifier) {
                let value = attributes[attribute_index..].trim_start_matches(&attribute_identifier);

                if value.starts_with("\"") {
                    return Some(value[1..(value[1..].find("\"").unwrap() + 1)].to_owned());
                } else if value.starts_with("'") {
                    return Some(value[1..(value[1..].find("'").unwrap())].to_owned());
                } else {
                    return value.split(" ").nth(0).map(|x| x.to_owned());
                }
            }
        }

        None
    }
}
