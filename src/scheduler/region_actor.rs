use crate::world::RegionId;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
pub struct RegionActor {
    id: RegionId,
    applied: usize,
    inbox: mpsc::Receiver<RegionCommand>,
}

#[derive(Debug, Clone)]
pub struct RegionHandle {
    id: RegionId,
    outbox: mpsc::Sender<RegionCommand>,
}

#[derive(Debug)]
enum RegionCommand {
    Apply {
        label: String,
        reply: oneshot::Sender<usize>,
    },
    Snapshot {
        reply: oneshot::Sender<usize>,
    },
}

impl RegionActor {
    pub fn spawn(id: RegionId) -> RegionHandle {
        let (outbox, inbox) = mpsc::channel(64);
        let actor = Self {
            id,
            applied: 0,
            inbox,
        };
        tokio::spawn(actor.run());
        RegionHandle { id, outbox }
    }

    async fn run(mut self) {
        while let Some(command) = self.inbox.recv().await {
            match command {
                RegionCommand::Apply { label, reply } => {
                    tracing::trace!(region = self.id.0, %label, "region task applied");
                    self.applied += 1;
                    let _ = reply.send(self.applied);
                }
                RegionCommand::Snapshot { reply } => {
                    let _ = reply.send(self.applied);
                }
            }
        }
    }
}

impl RegionHandle {
    pub const fn id(&self) -> RegionId {
        self.id
    }

    pub async fn apply(&self, label: impl Into<String>) -> Result<usize, RegionActorError> {
        let (reply, receive) = oneshot::channel();
        self.outbox
            .send(RegionCommand::Apply {
                label: label.into(),
                reply,
            })
            .await
            .map_err(|_| RegionActorError::Closed)?;
        receive.await.map_err(|_| RegionActorError::Closed)
    }

    pub async fn applied_count(&self) -> Result<usize, RegionActorError> {
        let (reply, receive) = oneshot::channel();
        self.outbox
            .send(RegionCommand::Snapshot { reply })
            .await
            .map_err(|_| RegionActorError::Closed)?;
        receive.await.map_err(|_| RegionActorError::Closed)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RegionActorError {
    #[error("region actor is closed")]
    Closed,
}

#[cfg(test)]
mod tests {
    use super::RegionActor;
    use crate::world::RegionId;

    #[tokio::test]
    async fn applies_tasks_in_mailbox_order() {
        let handle = RegionActor::spawn(RegionId(7));
        assert_eq!(handle.id(), RegionId(7));
        assert_eq!(handle.apply("a").await.unwrap(), 1);
        assert_eq!(handle.apply("b").await.unwrap(), 2);
        assert_eq!(handle.applied_count().await.unwrap(), 2);
    }
}
