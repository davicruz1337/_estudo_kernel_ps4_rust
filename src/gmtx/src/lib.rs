pub use self::guard::*;
use std::cell::{Cell, UnsafeCell};
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex, Weak};
use std::thread::Thread;

mod guard;

/// A mutex that grant exclusive access to a group of members.
///
/// The [`std::sync::Mutex`] and the related types are prone to deadlock when using on a multiple
/// struct fields like this:
///
/// ```
/// use std::sync::Mutex;
///
/// pub struct Foo {
///     field1: Mutex<()>,
///     field2: Mutex<()>,
/// }
/// ```
///
/// The order to acquire the lock must be the same everywhere otherwise the deadlock is possible.
/// Maintaining the lock order manually are cumbersome task so we introduce this type to handle this
/// instead.
///
/// How this type are working is simple. Any locks on any member will lock the same mutex in the
/// group, which mean there are only one mutex in the group. It have the same effect as the
/// following code:
///
/// ```
/// use std::sync::Mutex;
///
/// pub struct Foo {
///     data: Mutex<Data>,
/// }
///
/// struct Data {
///     field1: (),
///     field2: (),
/// }
/// ```
///
/// The bonus point of this type is it will allow recursive lock for read-only access so you will
/// never end up deadlock yourself. It will panic if you try to acquire write access while the
/// readers are still active the same as [`std::cell::RefCell`].
#[derive(Debug)]
pub struct Gutex<T> {
    group: Arc<GutexGroup>,
    active: UnsafeCell<usize>,
    value: UnsafeCell<T>,
}

impl<T> Gutex<T> {
    pub fn get_mut(&mut self) -> &mut T {
        self.value.get_mut()
    }

    /// # Panics
    /// - If there are an active writer.
    /// - If the calling thread already hold the lock on another group.
    pub fn read(&self) -> GutexReadGuard<'_, T> {
        // Check if there are an active writer.
        let lock = self.group.lock();
        let active = self.active.get();

        unsafe {
            if *active == usize::MAX {
                panic!("attempt to acquire the read lock while there are an active write lock");
            } else if *active == (usize::MAX - 1) {
                // This should never happen because stack overflow should be triggering first.
                panic!("maximum number of active readers has been reached");
            }

            *active += 1;
        }

        GutexReadGuard::new(lock, self)
    }

    /// # Panics
    /// - If there are any active reader or writer.
    /// - If the calling thread already hold the lock on another group.
    pub fn write(&self) -> GutexWriteGuard<'_, T> {
        // Check if there are active reader or writer.
        let lock = self.group.lock();
        let active = self.active.get();

        unsafe {
            if *active != 0 {
                panic!(
                    "attempt to acquire the write lock while there are an active reader or writer"
                );
            }

            *active = usize::MAX;
        }

        GutexWriteGuard::new(lock, self)
    }
}

unsafe impl<T: Send> Send for Gutex<T> {}
unsafe impl<T: Send> Sync for Gutex<T> {}

/// Represents a group of [`Gutex`].
#[derive(Debug)]
pub struct GutexGroup {
    name: Arc<String>,
    owning: ThreadId,
    active: UnsafeCell<usize>,
    waiting: Mutex<VecDeque<Weak<Thread>>>,
}

impl GutexGroup {
    pub fn new<N: Into<String>>(name: N) -> Arc<Self> {
        Arc::new(Self {
            name: Arc::new(name.into()),
            owning: ThreadId::new(0),
            active: UnsafeCell::new(0),
            waiting: Mutex::new(VecDeque::new()),
        })
    }

    pub fn spawn<T>(self: &Arc<Self>, value: T) -> Gutex<T> {
        Gutex {
            group: self.clone(),
            active: UnsafeCell::new(0),
            value: UnsafeCell::new(value),
        }
    }

    fn lock(&self) -> GroupGuard<'_> {
        // Check if the calling thread already own the lock.
        let current = Self::current_thread();

        if current == self.owning.load(Ordering::Relaxed) {
            // SAFETY: This is safe because the current thread own the lock.
            return unsafe { GroupGuard::new(self) };
        }

        // Check if the calling thread already own a lock on another group to prevent a possible
        // deadlock.
        if let Some(active) = ACTIVE_GROUP.take() {
            panic!(
                "attempt to acquire '{}' mutex group while holding the lock on '{}' group",
                self.name, active
            );
        }

        // Put the current thread into the wait queue. This need to be done before
        // compare_exchange() so we don't end up parking the current thread when someone just
        // release the lock after compare_exchange().
        let waiting = Arc::new(std::thread::current());

        self.waiting
            .lock()
            .unwrap()
            .push_back(Arc::downgrade(&waiting));

        // Acquire the lock.
        while self
            .owning
            .compare_exchange(0, current, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // Wait for the lock to unlock.
            std::thread::park();
        }

        drop(waiting);

        // Set active group.
        ACTIVE_GROUP.set(Some(self.name.clone()));

        // SAFETY: This is safe because the current thread acquire the lock successfully by the
        // above compare_exchange().
        unsafe { GroupGuard::new(self) }
    }

    #[cfg(target_os = "linux")]
    fn current_thread() -> i32 {
        unsafe { libc::gettid() }
    }

    #[cfg(target_os = "macos")]
    fn current_thread() -> u64 {
        let mut id = 0;
        assert_eq!(unsafe { libc::pthread_threadid_np(0, &mut id) }, 0);
        id
    }

    #[cfg(target_os = "windows")]
    fn current_thread() -> u32 {
        unsafe { windows_sys::Win32::System::Threading::GetCurrentThreadId() }
    }
}

unsafe impl Send for GutexGroup {}
unsafe impl Sync for GutexGroup {}

/// An RAII object used to release the lock on [`GutexGroup`]. This type cannot be send because it
/// will cause data race on the group when dropping if more than one [`GroupGuard`] are active.
struct GroupGuard<'a> {
    group: &'a GutexGroup,
    phantom: PhantomData<Rc<i32>>, // For !Send and !Sync.
}

impl<'a> GroupGuard<'a> {
    /// # Safety
    /// The group must be locked by the calling thread with no active references to any of its
    /// field.
    unsafe fn new(group: &'a GutexGroup) -> Self {
        *group.active.get() += 1;

        Self {
            group,
            phantom: PhantomData,
        }
    }
}

impl<'a> Drop for GroupGuard<'a> {
    fn drop(&mut self) {
        // Decrease the active lock.
        unsafe {
            let active = self.group.active.get();

            *active -= 1;

            if *active != 0 {
                return;
            }
        }

        // Release the lock.
        ACTIVE_GROUP.take().unwrap();
        self.group.owning.store(0, Ordering::Release);

        // Wakeup one waiting thread.
        let mut waiting = match self.group.waiting.try_lock() {
            Ok(v) => v,
            Err(_) => return,
        };

        while let Some(t) = waiting.pop_front() {
            if let Some(t) = t.upgrade() {
                t.unpark();
                break;
            }
        }
    }
}

#[cfg(target_os = "linux")]
type ThreadId = std::sync::atomic::AtomicI32;

#[cfg(target_os = "macos")]
type ThreadId = std::sync::atomic::AtomicU64;

#[cfg(target_os = "windows")]
type ThreadId = std::sync::atomic::AtomicU32;

thread_local! {
    static ACTIVE_GROUP: Cell<Option<Arc<String>>> = const { Cell::new(None) };
}
