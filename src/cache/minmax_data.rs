#[derive(Clone, Copy)]
pub struct MinMaxData<Action> {
    pub is_maximizer: bool,
    pub depth: usize,
    pub value: f64,
    pub action: Option<Action>,
}

impl<Action> MinMaxData<Action> {
    pub fn new(is_maximizer: bool) -> Self {
        if is_maximizer {
            MinMaxData {
                is_maximizer: true,
                depth: 0,
                value: f64::NEG_INFINITY,
                action: None,
            }
        } else {
            MinMaxData {
                is_maximizer: false,
                depth: 0,
                value: f64::INFINITY,
                action: None,
            }
        }
    }
}

impl<Action> Default for MinMaxData<Action> {
    fn default() -> Self {
        MinMaxData {
            is_maximizer: true,
            depth: 0,
            value: f64::NAN,
            action: None,
        }
    }
}
