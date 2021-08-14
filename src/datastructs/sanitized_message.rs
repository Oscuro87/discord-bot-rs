pub struct SanitizedMessage {
    pub full_content: String,
    pub command: String,
    pub arguments: Vec<String>,
    pub args_single_line: String,
    pub num_args: usize,
}

impl SanitizedMessage {
    pub fn are_any_arguments_present(&self) -> bool {
        self.arguments.len() > 0
    }
}

impl From<serenity::model::channel::Message> for SanitizedMessage {
    fn from(source: serenity::model::channel::Message) -> Self {
        let mut content: String = source.content.clone();
        content.remove(0);
        let split_all = content
            .split(" ")
            .map(|entry| entry.into())
            .collect::<Vec<String>>();
        let command = split_all[0].clone();
        let split_args = split_all[1..].to_vec();
        let args_single_line: String = split_args.join(" ");

        SanitizedMessage {
            full_content: source.content.clone(),
            command,
            arguments: split_args.clone(),
            args_single_line,
            num_args: split_args.len(),
        }
    }
}

impl From<&serenity::model::channel::Message> for SanitizedMessage {
    fn from(source: &serenity::model::channel::Message) -> Self {
        let mut content: String = source.content.clone();
        content.remove(0);
        let split_all = content
            .split(" ")
            .map(|entry| entry.into())
            .collect::<Vec<String>>();
        let command = split_all[0].clone();
        let split_args = split_all[1..].to_vec();
        let args_single_line: String = split_args.join(" ");

        SanitizedMessage {
            full_content: source.content.clone(),
            command,
            arguments: split_args.clone(),
            args_single_line,
            num_args: split_args.len(),
        }
    }
}
