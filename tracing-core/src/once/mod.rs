use core::fmt;
use once_cell::sync::OnceCell;

/// A synchronization primitive which can be used to run a one-time global
/// initialization. Unlike its std equivalent, this is implemented on top of
/// `OnceCell`, thereby storing the returned value, activing something like
/// a future.
pub struct Once<T> {
    cell: OnceCell<T>,
}

impl<T: fmt::Debug> fmt::Debug for Once<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.cell.get() {
            Some(s) => write!(f, "Once {{ cell: ")
                .and_then(|()| s.fmt(f))
                .and_then(|()| write!(f, "}}")),
            None => write!(f, "Once {{ <uninitialized> }}"),
        }
    }
}

impl<T> Once<T> {
    /// Initialization constant of `Once`.
    pub const INIT: Self = Once {
        cell: OnceCell::new(),
    };

    /// Creates a new `Once` instance.
    pub const fn new() -> Once<T> {
        Self::INIT
    }

    /// Performs an initialization routine once and only once. The given closure
    /// will be executed if this is the first time `call_once` has been called,
    /// and otherwise the routine will *not* be invoked.
    ///
    /// This method will block the calling thread if another initialization
    /// routine is currently running.
    ///
    /// When this function returns, it is guaranteed that some initialization
    /// has run and completed (it may not be the closure specified). The
    /// returned pointer will point to the result from the closure that was
    /// run.
    pub fn call_once<'a, F>(&'a self, builder: F) -> &'a T
    where
        F: FnOnce() -> T,
    {
        self.cell.get_or_init(builder)
    }
}
