use dominator::{html, Dom, clone};
use crate::data::{raw, state::*};
use std::rc::Rc;
use utils::events;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{ReadOnlyMutable, SignalExt},
    signal_vec::SignalVecExt,
};


pub struct PairDom {}
impl PairDom {
    pub fn render(state:Rc<State>, game_mode: GameMode, step: Step, index: ReadOnlyMutable<Option<usize>>, pair:(Card, Card)) -> Dom {

        let left = CardDom::render(state.clone(), game_mode, step, index.clone(), Side::Left, pair.0.clone(), pair.1.clone());
        let right = CardDom::render(state.clone(), game_mode, step, index.clone(), Side::Right, pair.1, pair.0);

        if step == Step::One {
            html!("main-card-pair", {
                .property("hoverable", true)
                .property_signal("index", index.signal().map(|x| {
                    JsValue::from_f64(x.unwrap_or_default() as f64)
                }))
                .child(left)
                .child(right)
                .child(html!("button-icon", {
                    .property("slot", "close")
                    .property("icon", "circle-x-blue")
                    .event(clone!(state => move |evt:events::Click| {
                        state.delete_pair(index.get().unwrap_or_default());
                    }))
                }))
            })
        } else {
            html!("main-card-pair", {
                .property("hoverable", false)
                .property_signal("index", index.signal().map(|x| {
                    JsValue::from_f64(x.unwrap_or_default() as f64)
                }))
                .child(left)
                .child(right)
            })
        }
    }
}

struct CardDom {}

impl CardDom {
    pub fn render(state:Rc<State>, game_mode: GameMode, step: Step, index: ReadOnlyMutable<Option<usize>>, side:Side, card: Card, other: Card) -> Dom {
        let original_data = card.data.get_cloned();
        html!("main-card", {
            .property("slot", side.slot_name())
            .property("flippable", step == Step::Two)
            .property("editing", step == Step::One)
            .property_signal("theme", state.theme.signal_cloned())
            .child({
                html!("input-text-content", {
                    .property_signal(
                        "value", 
                        card.data
                            .signal_cloned()
                            .map(|value| value.unwrap_or_default())
                    )
                    .property("clickMode", "single")
                    .event(clone!(state, index, other => move |evt:events::CustomInput| {
                        let index = index.get().unwrap_or_default();
                        let value = evt.value();

                        if game_mode == GameMode::Duplicate {
                            other.data.set(Some(value));
                        }
                    }))
                    .event(clone!(state, index => move |evt:events::CustomChange| {
                        let index = index.get().unwrap_or_default();
                        let value = evt.value();
                        state.replace_card_value(&card, index, side, value);
                    }))
                    .event(clone!(state, index, other, original_data => move |evt:events::Reset| {
                        let index = index.get().unwrap_or_default();
                        if game_mode == GameMode::Duplicate {
                            other.data.set(original_data.clone());
                        }
                    }))
                })
            })
        })
    }
}

/*
    const editing = ioMode === "edit"; 
    if(contentMode === "text") {
        const value = "hello";
        return `<input-text-content value="${value}" ${editing}></input-text-content>`;
    } else if(contentMode === "image") {
        return MockJiImage({size: "thumb"})
    } else if(contentMode === "image-empty") {
        return `<img-ui path="core/_common/image-empty.svg"></img-ui>`
    }
    */
