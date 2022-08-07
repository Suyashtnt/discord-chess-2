use commands::CommandError;

use error_stack::Report;
use tracing::error;

pub async fn on_error(error: poise::FrameworkError<'_, (), Report<CommandError>>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            error!(
                "{error:?}\nHere are the args: \n{}",
                error.request_ref::<commands::Arg>().iter().fold("".to_string(), |acc, arg| acc
                    + &(match arg {
                        commands::Arg::String(n, v) => format!("String {n} - {v}"),
                        commands::Arg::User(n, v) => format!("User ID {n} - {v}"),
                        commands::Arg::Int(n, v) => format!("Int {n} - {v}"),
                        commands::Arg::Number(n, v) => format!("Double {n} - {v}"),
                        commands::Arg::Boolean(n, v) => format!("Bool {n} - {v}"),
                        commands::Arg::Channel(n, v) => format!("Channel ID {n} - {v}"),
                        commands::Arg::Role(n, v) => format!("Role ID {n} - {v}"),
                        commands::Arg::Mentionable(n, v) => format!("Mention {n} - {}", v),
                        commands::Arg::Attachment(n, v) =>
                            format!("Atttachment {n} - ID {}, URL {}", v.id, v.url),
                    }))
            );

            if let Err(e) = ctx
                .send(|m| {
                    m.embed(|e| {
                        e.title("!ERROR! !ERROR! !ABORT! !ERROR! !ERROR!")
                            .description(format!(
                                "An error occured when running command {}",
                                error.current_context().name
                            ))
                    })
                })
                .await
            {
                error!("Error sending error message: {}", e)
            };
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                error!("Error while handling error: {}", e)
            }
        }
    }
}
