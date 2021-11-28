use std::cell::RefCell;
use std::rc::Rc;

mod bridge;

pub struct RustApp {
    click_counter: Rc<RefCell<u32>>,
    rerender_trigger: Rc<RefCell<bridge::RerenderTrigger>>,
}

impl RustApp {
    fn new(rerender_trigger: bridge::RerenderTrigger) -> Self {
        Self {
            click_counter: Rc::new(RefCell::new(0)),
            rerender_trigger: Rc::new(RefCell::new(rerender_trigger)),
        }
    }

    fn render(&self) -> bridge::SwiftUIButton {
        let text = format!(
            "The button has been clicked {} times!",
            self.click_counter.borrow()
        );

        let text = bridge::SwiftUIText::new(&text);
        // text.bold();

        let click_counter = Rc::clone(&self.click_counter);
        let rerender_trigger = Rc::clone(&self.rerender_trigger);

        let onclick = move || {
            println!("Button clicked!");
            *click_counter.borrow_mut() += 1;
            rerender_trigger.borrow().render();
        };
        let onclick = bridge::ButtonAction(Box::new(onclick));

        bridge::SwiftUIButton::new(text, onclick)
    }
}
