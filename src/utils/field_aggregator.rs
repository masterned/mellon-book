#[derive(Clone, Debug, Default)]
pub struct FieldAggregator(pub Option<Vec<&'static str>>);

impl FieldAggregator {
    pub fn new() -> Self {
        FieldAggregator::default()
    }

    pub fn field_check<T>(&mut self, field: &Option<T>, field_name: &'static str) {
        if field.is_none() {
            self.0.get_or_insert_default().push(field_name);
        }
    }
}
