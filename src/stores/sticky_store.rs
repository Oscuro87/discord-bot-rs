use serenity::model::id::MessageId;

// TODO: Timeout & Time Handling

pub struct StickyStore {
    pub current_sticky: String,
    pub posted_message_id: Option<MessageId>,
}

impl StickyStore {
    pub fn create_empty_store() -> Self {
        StickyStore {
            current_sticky: "".into(),
            posted_message_id: None,
        }
    }

    pub fn create_store_populate(initial_value: String) -> Self {
        StickyStore { 
            current_sticky: initial_value,
            posted_message_id: None,
        }
    }

    pub fn sticky_exists(&self) -> bool {
        self.current_sticky.trim().len() > 0
    }

    pub fn create_sticky(&mut self, sticky_message: &String) -> () {
        // self.posted_message_id = Some(msg_id);
        self.current_sticky = sticky_message.clone();
    }

    pub fn update_posted_message_id(&mut self, msg_id: MessageId) -> () {
        self.posted_message_id = Some(msg_id.clone());
    }

    pub fn clear_posted_message_id(&mut self) -> () {
        self.posted_message_id = None;
    }
}