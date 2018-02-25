pub struct Post {
    state: Option<Box<State>>,
    content: String,
}

trait State {
    // snip
    fn add_text(&self, post: &mut Post, text: &str) { }
}


struct Draft { }

impl State for Draft {
    // snip
    fn add_text(&self, post: &mut Post, text: &str) {
        post.content.push_str(text);
    }
}

impl Post {
    // snip

    pub fn add_text(&mut self, text: &str){
        let state = &mut self.state;
        let state = state.unwrap().as_mut();
        state.add_text(self, text);  // so that this mutable borrow is no longer possible
    }
}
