//! The grammar (checking) actor

use crate::{
    actor::typ_server::EntryStateExt,
    tools::checkers::nlprule::{diag_from_suggestion, Suggestion},
};
use log::info;
use tinymist_query::VersionedDocument;
use tokio::sync::{
    broadcast::{self, error::RecvError},
    mpsc, watch,
};
use typst_ts_compiler::service::Compiler;

use crate::tools::checkers;

use super::{render::RenderActorRequest, typ_client::CompileDriver};

pub struct VersionedSuggestions {
    pub version: u64,
    pub suggestions: Option<Vec<checkers::nlprule::Suggestion>>,
}

pub struct GrammarActor {
    render_rx: broadcast::Receiver<RenderActorRequest>,
    document: watch::Receiver<Option<VersionedDocument>>,
    suggestions: mpsc::Sender<VersionedSuggestions>,
}

impl GrammarActor {
    pub fn new(
        render_rx: broadcast::Receiver<RenderActorRequest>,
        document: watch::Receiver<Option<VersionedDocument>>,
        suggestions: mpsc::Sender<VersionedSuggestions>,
    ) -> Self {
        Self {
            render_rx,
            document,
            suggestions,
        }
    }

    pub async fn run(mut self) {
        loop {
            tokio::select! {
                req = self.render_rx.recv() => {
                    let req = match req {
                        Ok(req) => req,
                        Err(RecvError::Closed) => {
                            info!("GrammarActor: channel closed");
                            break;
                        }
                        Err(RecvError::Lagged(_)) => {
                            info!("GrammarActor: channel lagged");
                            continue;
                        }

                    };

                    info!("GrammarActor: received request: {req:?}", req = req);
                    match req {
                        RenderActorRequest::ChangeConfig(..) => {
                        }
                        RenderActorRequest::ChangeExportPath(..) => {
                        }
                        _ => {
                            // todo: check grammar
                            let doc = self.document.borrow().clone();
                            let Some(doc) = doc else {
                                continue;
                            };
                           let res=  checkers::nlprule::nlp_check_docs(doc.document);
                            self.suggestions.send(VersionedSuggestions {
                                version: doc.version as u64,
                                suggestions: res,
                            }).await.unwrap();
                        }
                    }
                }
            }
        }
        info!("GrammarActor: stopped");
    }
}

impl CompileDriver {
    pub fn notify_suggestions(&mut self, suggestions: Option<Vec<Suggestion>>) {
        log::trace!("notify suggestions: {:#?}", suggestions);
        let suggestions = suggestions.unwrap_or_default();
        let suggestions = suggestions
            .into_iter()
            .map(diag_from_suggestion)
            .collect::<Vec<_>>();

        let suggestions =
            self.run_analysis(|ctx| tinymist_query::convert_diagnostics(ctx, suggestions.iter()));

        match suggestions {
            Ok(suggestions) => {
                // todo: better way to remove suggestions
                // todo: check all errors in this file
                let detached = self.inner.world().entry.is_inactive();
                let valid = !detached;
                self.handler
                    .push_diagnostics(valid.then_some(suggestions), Some("grammar"));
            }
            Err(err) => {
                log::error!("TypstActor: failed to convert diagnostics: {:#}", err);
                self.handler.push_diagnostics(None, Some("grammar"));
            }
        }
    }
}
