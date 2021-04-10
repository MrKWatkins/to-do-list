#[derive(Clone)]
pub struct FormField<T>
{
    pub label: String,
    pub value: T,
    pub is_enabled: bool,
    pub is_valid: bool
}

impl<T> FormField<T> {
    pub fn new(label: &str, value: T) -> FormField<T>
    {
        return FormField {
            label: label.to_string(),
            value,
            is_enabled: true,
            is_valid: true
        };
    }

    pub fn disabled(label: &str, value: T) -> FormField<T>
    {
        return FormField {
            label: label.to_string(),
            value,
            is_enabled: false,
            is_valid: true
        };
    }
}