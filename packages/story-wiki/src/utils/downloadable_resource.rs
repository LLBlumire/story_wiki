use std::{fmt::Debug, ops::Deref};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum DownloadableResource<T> {
    NotYetRequested,
    Downloading,
    Ready(T),
}
impl<T> Default for DownloadableResource<T> {
    fn default() -> Self {
        DownloadableResource::NotYetRequested
    }
}
impl<T> DownloadableResource<T> {
    pub fn requested(&self) -> bool {
        !matches!(self, DownloadableResource::NotYetRequested)
    }

    pub fn is_ready(&self) -> bool {
        matches!(self, DownloadableResource::Ready(_))
    }

    pub fn opt(self) -> Option<T> {
        if let DownloadableResource::Ready(t) = self {
            Some(t)
        } else {
            None
        }
    }

    pub fn as_ref(&self) -> DownloadableResource<&T> {
        match self {
            DownloadableResource::NotYetRequested => DownloadableResource::NotYetRequested,
            DownloadableResource::Downloading => DownloadableResource::Downloading,
            DownloadableResource::Ready(t) => DownloadableResource::Ready(t),
        }
    }

    pub fn as_deref(&self) -> DownloadableResource<&T::Target>
    where
        T: Deref,
    {
        match self {
            DownloadableResource::NotYetRequested => DownloadableResource::NotYetRequested,
            DownloadableResource::Downloading => DownloadableResource::Downloading,
            DownloadableResource::Ready(t) => DownloadableResource::Ready(&**t),
        }
    }
}

impl<T: Debug> DownloadableResource<T> {
    pub fn unwrap(self) -> T {
        if let DownloadableResource::Ready(this) = self {
            this
        } else {
            panic!("Unwrap of {:?}", self)
        }
    }
}
