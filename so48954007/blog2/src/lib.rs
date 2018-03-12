pub struct Post {
    state: Option<Box<State>>,
    content: String,
}

trait State {
    // snip
    fn add_text(&self, content: &mut String, text: &str) { }
}

struct Draft { }

impl State for Draft {
    // snip
    fn add_text(&self, content: &mut String, text: &str) {
        content.push_str(text);
    }
}

impl Post {
    // snip

    pub fn add_text(&mut self, text: &str){
        let state = self.state.as_ref().unwrap();  // This immutably borrows self
        state.add_text(&mut self.content, text);  // so that this mutable borrow is no longer possible
    }
}
