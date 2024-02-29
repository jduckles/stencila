use common::tracing;
use node_store::ReadNode;
use schema::Node;

use crate::{
    Command, Document, DocumentCommandReceiver, DocumentKernels, DocumentStore,
    DocumentUpdateSender,
};

impl Document {
    /// Asynchronous task to coalesce and perform document commands
    pub(super) async fn command_task(
        mut command_receiver: DocumentCommandReceiver,
        store: DocumentStore,
        kernels: DocumentKernels,
        update_sender: DocumentUpdateSender,
    ) {
        tracing::debug!("Document command task started");

        // Receive commands
        while let Some(command) = command_receiver.recv().await {
            tracing::trace!("Document command `{}` received", command.to_string());

            match command {
                Command::ExecuteDocument => {
                    execute_document(&store, &kernels, &update_sender).await
                }
                _ => {
                    tracing::warn!("TODO: handle {command} command");
                }
            }
        }

        tracing::debug!("Document command task stopped");
    }
}

async fn execute_document(
    store: &DocumentStore,
    kernels: &DocumentKernels,
    update_sender: &DocumentUpdateSender,
) {
    // Load the root node from the store
    let mut root = {
        // This is within a block to ensure that the lock on `store` gets
        // dropped before execution
        let store = store.read().await;
        Node::load(&*store).unwrap()
    };

    let mut kernels = kernels.write().await;

    // TODO: this executes the entire document and then sends a single update.
    // Instead, have a `node_execute` function that takes a channel which can
    // receive updates for individual nodes after they are updated

    // Execute the root node
    node_execute::execute(&mut root, &mut kernels, None)
        .await
        .unwrap();

    // Send the updated root node to the store
    if let Err(error) = update_sender.send(root).await {
        tracing::error!("While sending root update: {error}");
    }
}
