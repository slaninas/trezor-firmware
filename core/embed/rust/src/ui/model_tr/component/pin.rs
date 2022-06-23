use core::ops::Deref;

use crate::{
    trezorhal::random,
    ui::{
        component::{text::common::TextBox, Component, Event, EventCtx},
        geometry::{Point, Rect},
    },
};

use super::{common, common::MultilineStringChoiceItem, ChoicePage, ChoicePageMsg};
use heapless::{LinearMap, String, Vec};

pub enum PinEntryMsg {
    Confirmed,
    Cancelled,
}

const PIN_ROW: i32 = 40;

const MAX_LENGTH: usize = 50;
const MAX_VISIBLE_DOTS: usize = 18;
const MAX_VISIBLE_DIGITS: usize = 18;

const CHOICE_LENGTH: usize = 11;
const DIGITS: [&str; CHOICE_LENGTH] = [
    "PLACEHOLDER FOR THE PROMPT",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    "0",
];

/// Component for entering a PIN.
pub struct PinEntry {
    choice_page: ChoicePage<MultilineStringChoiceItem, CHOICE_LENGTH>,
    show_real_pin: bool,
    textbox: TextBox<MAX_LENGTH>,
}

impl PinEntry {
    pub fn new<T>(prompt: T) -> Self
    where
        T: Deref<Target = str>,
    {
        // TODO: it is possible to create a vector combining both
        // StringChoiceItem and MultilineStringChoiceItem?

        // TODO: there could be some better way of showing the prompt

        // Putting the prompt at the first place of the choice list,
        // replacing the placeholder
        let mut choices: Vec<MultilineStringChoiceItem, CHOICE_LENGTH> = Vec::new();
        for (index, digit) in DIGITS.iter().enumerate() {
            let item = if index == 0 {
                MultilineStringChoiceItem::from_slice(&prompt).use_delimiter(' ')
            } else {
                MultilineStringChoiceItem::from_slice(digit)
            };
            choices.push(item).unwrap();
        }

        // Creating a custom middle-button-text for the prompt
        let mut button_map = LinearMap::new();
        button_map.insert(0, "ACCEPT").unwrap();

        let mut choice_page = ChoicePage::new(choices).with_button_map(button_map);

        // Setting the button to delete digits and cancel when there is no digit
        choice_page.set_leftmost_button("BIN");

        Self {
            choice_page,
            show_real_pin: false,
            textbox: TextBox::empty(),
        }
    }

    fn update_situation(&mut self) {
        if self.show_real_pin {
            self.reveal_current_pin();
            self.show_real_pin = false;
        } else {
            self.show_pin_length();
        }
    }

    fn show_pin_length(&self) {
        // Only showing the maximum visible length
        let digits = self.textbox.len();
        let dots_visible = digits.min(MAX_VISIBLE_DOTS);

        // String::repeat() is not available for heapless::String
        let mut dots: String<50> = String::new();
        for _ in 0..dots_visible {
            dots.push_str("*").unwrap();
        }

        // Giving some notion of change even for longer-than-visible PINs
        // - slightly shifting the dots to the left and right after each new digit
        if digits > MAX_VISIBLE_DOTS && digits % 2 == 0 {
            common::display_bold_center(Point::new(61, PIN_ROW), &dots);
        } else {
            common::display_bold_center(Point::new(64, PIN_ROW), &dots);
        }
    }

    fn reveal_current_pin(&self) {
        let digits = self.textbox.len();

        if digits <= MAX_VISIBLE_DOTS {
            common::display_bold_center(Point::new(64, PIN_ROW), self.pin());
        } else {
            // Show the last part of PIN with preceding ellipsis to show something is hidden
            let ellipsis = "...";
            let offset: usize = digits.saturating_sub(MAX_VISIBLE_DIGITS) + ellipsis.len();
            let mut to_show: String<MAX_VISIBLE_DIGITS> = String::from(ellipsis);
            to_show.push_str(&self.pin()[offset..]).unwrap();
            common::display_bold_center(Point::new(32, PIN_ROW), &to_show);
        }
    }

    fn append_new_digit(&mut self, ctx: &mut EventCtx, page_counter: u8) {
        let digit = DIGITS[page_counter as usize];
        self.textbox.append_slice(ctx, digit);
    }

    fn delete_last_digit(&mut self, ctx: &mut EventCtx) {
        self.textbox.delete_last(ctx);
    }

    pub fn pin(&self) -> &str {
        self.textbox.content()
    }

    fn is_full(&self) -> bool {
        self.textbox.is_full()
    }

    fn is_empty(&self) -> bool {
        self.textbox.is_empty()
    }
}

impl Component for PinEntry {
    type Msg = PinEntryMsg;

    fn place(&mut self, bounds: Rect) -> Rect {
        self.choice_page.place(bounds)
    }

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        let msg = self.choice_page.event(ctx, event);

        match msg {
            // Sending the result, appending the new digit, deleting the last digit or cancelling
            Some(ChoicePageMsg::Choice(page_counter)) => match page_counter {
                0 => Some(PinEntryMsg::Confirmed),
                _ => {
                    if !self.is_full() {
                        self.append_new_digit(ctx, page_counter);
                        let new_page_counter =
                            random::uniform_between(1, (CHOICE_LENGTH - 1) as u32);
                        self.choice_page.set_page_counter(new_page_counter as u8);
                    }
                    None
                }
            },
            Some(ChoicePageMsg::LeftMost) => {
                if self.is_empty() {
                    Some(PinEntryMsg::Cancelled)
                } else {
                    self.delete_last_digit(ctx);
                    None
                }
            }
            _ => None,
        }
    }

    fn paint(&mut self) {
        self.choice_page.paint();
        self.update_situation();
    }
}

#[cfg(feature = "ui_debug")]
impl crate::trace::Trace for PinEntry {
    fn trace(&self, t: &mut dyn crate::trace::Tracer) {
        t.open("PinEntry");
        t.close();
    }
}
