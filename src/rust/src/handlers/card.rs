use std::sync::Arc;

use crate::State;

pub async fn get_all_cards(state: Arc<State>) -> String {
    format!("{state:?}")
}
