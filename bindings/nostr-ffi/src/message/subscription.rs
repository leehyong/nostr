// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

use std::ops::Deref;
use std::sync::Arc;

use nostr::message::subscription;
use nostr::JsonUtil;
use uniffi::{Enum, Object};

use crate::error::Result;
use crate::helper::unwrap_or_clone_arc;
use crate::{Event, EventId, Kind, PublicKey, Timestamp};

#[derive(Enum)]
pub enum Alphabet {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl From<Alphabet> for subscription::Alphabet {
    fn from(value: Alphabet) -> Self {
        match value {
            Alphabet::A => Self::A,
            Alphabet::B => Self::B,
            Alphabet::C => Self::C,
            Alphabet::D => Self::D,
            Alphabet::E => Self::E,
            Alphabet::F => Self::F,
            Alphabet::G => Self::G,
            Alphabet::H => Self::H,
            Alphabet::I => Self::I,
            Alphabet::J => Self::J,
            Alphabet::K => Self::K,
            Alphabet::L => Self::L,
            Alphabet::M => Self::M,
            Alphabet::N => Self::N,
            Alphabet::O => Self::O,
            Alphabet::P => Self::P,
            Alphabet::Q => Self::Q,
            Alphabet::R => Self::R,
            Alphabet::S => Self::S,
            Alphabet::T => Self::T,
            Alphabet::U => Self::U,
            Alphabet::V => Self::V,
            Alphabet::W => Self::W,
            Alphabet::X => Self::X,
            Alphabet::Y => Self::Y,
            Alphabet::Z => Self::Z,
        }
    }
}

#[derive(Object)]
pub struct SingleLetterTag {
    inner: subscription::SingleLetterTag,
}

impl Deref for SingleLetterTag {
    type Target = subscription::SingleLetterTag;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<subscription::SingleLetterTag> for SingleLetterTag {
    fn from(inner: subscription::SingleLetterTag) -> Self {
        Self { inner }
    }
}

#[uniffi::export]
impl SingleLetterTag {
    #[uniffi::constructor]
    pub fn lowercase(character: Alphabet) -> Self {
        Self {
            inner: subscription::SingleLetterTag::lowercase(character.into()),
        }
    }

    #[uniffi::constructor]
    pub fn uppercase(character: Alphabet) -> Self {
        Self {
            inner: subscription::SingleLetterTag::uppercase(character.into()),
        }
    }

    pub fn is_lowercase(&self) -> bool {
        self.inner.is_lowercase()
    }

    pub fn is_uppercase(&self) -> bool {
        self.inner.is_uppercase()
    }
}

#[derive(Clone, Object)]
pub struct Filter {
    inner: nostr::Filter,
}

impl Deref for Filter {
    type Target = nostr::Filter;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<nostr::Filter> for Filter {
    fn from(f: nostr::Filter) -> Self {
        Self { inner: f }
    }
}

#[uniffi::export]
impl Filter {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {
            inner: nostr::Filter::new(),
        }
    }

    pub fn id(self: Arc<Self>, id: &EventId) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.id(**id);
        builder
    }

    pub fn ids(self: Arc<Self>, ids: &[Arc<EventId>]) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.ids(ids.iter().map(|id| ***id));
        builder
    }

    pub fn remove_ids(self: Arc<Self>, ids: &[Arc<EventId>]) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.remove_ids(ids.iter().map(|id| ***id));
        builder
    }

    /// Add event author Public Key
    pub fn author(self: Arc<Self>, author: &PublicKey) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.author(**author);
        builder
    }

    pub fn authors(self: Arc<Self>, authors: &[Arc<PublicKey>]) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.authors(authors.iter().map(|pk| ***pk));
        builder
    }

    pub fn remove_authors(self: Arc<Self>, authors: &[Arc<PublicKey>]) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.remove_authors(authors.iter().map(|pk| ***pk));
        builder
    }

    pub fn kind(self: Arc<Self>, kind: &Kind) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.kind(**kind);
        builder
    }

    pub fn kinds(self: Arc<Self>, kinds: Vec<Arc<Kind>>) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.kinds(kinds.into_iter().map(|k| **k));
        builder
    }

    pub fn remove_kinds(self: Arc<Self>, kinds: Vec<Arc<Kind>>) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.remove_kinds(kinds.into_iter().map(|k| **k));
        builder
    }

    /// Add event ID (`e` tag)
    pub fn event(self: Arc<Self>, event_id: &EventId) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.event(**event_id);
        builder
    }

    /// Add event IDs (`e` tag)
    pub fn events(self: Arc<Self>, ids: &[Arc<EventId>]) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.events(ids.iter().map(|id| ***id));
        builder
    }

    pub fn remove_events(self: Arc<Self>, ids: &[Arc<EventId>]) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.remove_events(ids.iter().map(|id| ***id));
        builder
    }

    /// Add Public Key (`p` tag)
    pub fn pubkey(self: Arc<Self>, pubkey: &PublicKey) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.pubkey(**pubkey);
        builder
    }

    /// Add Public Keys (`p` tag)
    pub fn pubkeys(self: Arc<Self>, pubkeys: &[Arc<PublicKey>]) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.pubkeys(pubkeys.iter().map(|pk| ***pk));
        builder
    }

    pub fn remove_pubkeys(self: Arc<Self>, pubkeys: &[Arc<PublicKey>]) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.remove_pubkeys(pubkeys.iter().map(|pk| ***pk));
        builder
    }

    pub fn hashtag(self: Arc<Self>, hashtag: &str) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.hashtag(hashtag);
        builder
    }

    pub fn hashtags(self: Arc<Self>, hashtags: Vec<String>) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.hashtags(hashtags);
        builder
    }

    pub fn remove_hashtags(self: Arc<Self>, hashtags: Vec<String>) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.remove_hashtags(hashtags);
        builder
    }

    pub fn reference(self: Arc<Self>, reference: &str) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.reference(reference);
        builder
    }

    pub fn references(self: Arc<Self>, references: Vec<String>) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.references(references);
        builder
    }

    pub fn remove_references(self: Arc<Self>, references: Vec<String>) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.remove_references(references);
        builder
    }

    pub fn identifier(self: Arc<Self>, identifier: &str) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.identifier(identifier);
        builder
    }

    pub fn identifiers(self: Arc<Self>, identifiers: Vec<String>) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.identifiers(identifiers);
        builder
    }

    pub fn remove_identifiers(self: Arc<Self>, identifiers: Vec<String>) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.remove_identifiers(identifiers);
        builder
    }

    pub fn search(self: Arc<Self>, text: &str) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.search(text);
        builder
    }

    pub fn remove_search(self: Arc<Self>) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.remove_search();
        builder
    }

    pub fn since(self: Arc<Self>, timestamp: &Timestamp) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.since(**timestamp);
        builder
    }

    pub fn remove_since(self: Arc<Self>) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.remove_since();
        builder
    }

    pub fn until(self: Arc<Self>, timestamp: &Timestamp) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.until(**timestamp);
        builder
    }

    pub fn remove_until(self: Arc<Self>) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.remove_until();
        builder
    }

    pub fn limit(self: Arc<Self>, limit: u64) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.limit(limit as usize);
        builder
    }

    pub fn remove_limit(self: Arc<Self>) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.remove_limit();
        builder
    }

    pub fn custom_tag(self: Arc<Self>, tag: &SingleLetterTag, content: Vec<String>) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.custom_tag(**tag, content);
        builder
    }

    pub fn remove_custom_tag(
        self: Arc<Self>,
        tag: Arc<SingleLetterTag>,
        content: Vec<String>,
    ) -> Self {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.remove_custom_tag(**tag, content);
        builder
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Determine if `Filter` match given `Event`.
    ///
    /// The `search` filed is not supported yet!
    pub fn match_event(&self, event: &Event) -> bool {
        self.inner.match_event(event.deref())
    }

    #[uniffi::constructor]
    pub fn from_json(json: String) -> Result<Self> {
        Ok(Self {
            inner: nostr::Filter::from_json(json)?,
        })
    }

    pub fn as_json(&self) -> String {
        self.inner.as_json()
    }
}
