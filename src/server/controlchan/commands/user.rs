use crate::auth::UserDetail;
use crate::server::controlchan::error::ControlChanError;
use crate::server::controlchan::handler::CommandContext;
use crate::server::controlchan::handler::CommandHandler;
use crate::server::controlchan::{Reply, ReplyCode};
use crate::server::session::SessionState;
use crate::storage;
use async_trait::async_trait;
use bytes::Bytes;

pub struct User {
    username: Bytes,
}

impl User {
    pub fn new(username: Bytes) -> Self {
        User { username }
    }
}

#[async_trait]
impl<S, U> CommandHandler<S, U> for User
where
    U: UserDetail,
    S: 'static + storage::StorageBackend<U> + Sync + Send,
    S::File: tokio::io::AsyncRead + Send,
    S::Metadata: storage::Metadata,
{
    async fn handle(&self, args: CommandContext<S, U>) -> Result<Reply, ControlChanError> {
        let mut session = args.session.lock().await;
        match session.state {
            SessionState::New | SessionState::WaitPass => {
                let user = std::str::from_utf8(&self.username)?;
                session.username = Some(user.to_string());
                session.state = SessionState::WaitPass;
                Ok(Reply::new(ReplyCode::NeedPassword, "Password Required"))
            }
            _ => Ok(Reply::new(ReplyCode::BadCommandSequence, "Please create a new connection to switch user")),
        }
    }
}
