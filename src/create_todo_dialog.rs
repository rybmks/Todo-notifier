use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

type MyDialogue = Dialogue<DialogueState, InMemStorage<DialogueState>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
enum DialogueState {
    #[default]
    Start,
    ReceiveTodoName,
    ReceiveTodoDescription {
        todo_name: String,
    },
}

pub async fn get_todo_info(bot: Bot) {
    let storage = InMemStorage::<DialogueState>::new();

    Dispatcher::builder(
        bot,
        Update::filter_message()
            .branch(dptree::case![DialogueState::Start].endpoint(start))
            .branch(dptree::case![DialogueState::ReceiveTodoName].endpoint(receive_todo_name))
            .branch(
                dptree::case![DialogueState::ReceiveTodoDescription { todo_name }]
                    .endpoint(receive_todo_desc),
            ),
    )
    .dependencies(dptree::deps![storage]) // Здесь передаем хранилище
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Write an event name:")
        .await?;
    dialogue.update(DialogueState::ReceiveTodoName).await?;
    Ok(())
}

async fn receive_todo_name(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            bot.send_message(msg.chat.id, "Great").await?;
            dialogue
                .update(DialogueState::ReceiveTodoDescription {
                    todo_name: text.into(),
                })
                .await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Send me a todo name.")
                .await?;
        }
    }

    Ok(())
}
async fn receive_todo_desc(
    bot: Bot,
    dialogue: MyDialogue,
    todo_name: String,
    msg: Message,
) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            bot.send_message(msg.chat.id, format!("YRAAA: {} || {}", todo_name, text))
                .await?;
            dialogue.exit().await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Send me a todo name.")
                .await?;
        }
    }
    Ok(())
}
