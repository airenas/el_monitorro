use super::Command;
use super::Message;
use super::Response;
use super::ShowFeedKeyboard;
use crate::db::telegram;
use diesel::PgConnection;
use frankenstein::SendMessageParams;
use typed_builder::TypedBuilder;

static COMMAND: &str = "/remove_template";

#[derive(TypedBuilder)]
pub struct RemoveTemplate {
    message: Message,
    args: String,
    callback: bool,
}

impl RemoveTemplate {
    pub fn run(&self) {
        self.execute(&self.message, &format!("{} {}", Self::command(), self.args));
    }

    fn remove_template(&self, db_connection: &mut PgConnection) -> String {
        let subscription =
            match self.find_subscription(db_connection, self.message.chat.id, &self.args) {
                Err(message) => return message,
                Ok(subscription) => subscription,
            };

        match telegram::set_template(db_connection, &subscription, None) {
            Ok(_) => "The template was removed".to_string(),
            Err(_) => "Failed to update the template".to_string(),
        }
    }

    pub fn command() -> &'static str {
        COMMAND
    }
}

impl Command for RemoveTemplate {
    fn response(&self) -> Response {
        let response = match self.fetch_db_connection() {
            Ok(mut connection) => self.remove_template(&mut connection),
            Err(error_message) => error_message,
        };

        if self.callback {
            self.simple_keyboard(
                response,
                format!("{} {}", ShowFeedKeyboard::command(), self.args),
                self.message.chat.id,
            )
        } else {
            Response::Simple(response)
        }
    }

    fn send_message(&self, send_message_params: SendMessageParams) {
        self.send_message_and_remove(send_message_params, &self.message);
    }
}
