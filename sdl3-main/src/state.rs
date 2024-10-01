use core::{
    ffi::c_void,
    ptr::{self, NonNull},
};

#[cfg(feature = "alloc")]
use alloc::{boxed::Box, sync::Arc};

#[cfg(feature = "std")]
use std::sync::{Mutex, RwLock};

pub trait AppState: Send + Sync + Sized {
    /// # Safety
    /// - `raw` must've been previously got from `into_raw` and not passed to `from_raw` before
    /// - all borrows from `raw` must end before this function is called
    #[must_use]
    unsafe fn from_raw(raw: *mut c_void) -> Self;

    fn into_raw(self) -> *mut c_void;
}

impl AppState for () {
    #[inline(always)]
    unsafe fn from_raw(_: *mut c_void) {}

    #[inline(always)]
    fn into_raw(self) -> *mut c_void {
        ptr::null_mut()
    }
}

impl<T> AppState for SyncPtr<T> {
    #[inline(always)]
    unsafe fn from_raw(raw: *mut c_void) -> Self {
        Self(raw as *mut T)
    }

    #[inline(always)]
    fn into_raw(self) -> *mut c_void {
        self.0 as *mut c_void
    }
}

#[cfg(feature = "alloc")]
impl<T: Send + Sync> AppState for Arc<T> {
    unsafe fn from_raw(raw: *mut c_void) -> Self {
        assert!(!raw.is_null());
        unsafe { Self::from_raw(raw as *mut T) }
    }

    #[inline(always)]
    fn into_raw(self) -> *mut c_void {
        Self::into_raw(self) as *mut c_void
    }
}

#[cfg(feature = "alloc")]
impl<T: Send + Sync> AppState for Box<T> {
    #[inline(always)]
    unsafe fn from_raw(raw: *mut c_void) -> Self {
        assert!(!raw.is_null());
        unsafe { Self::from_raw(raw as *mut T) }
    }

    #[inline(always)]
    fn into_raw(self) -> *mut c_void {
        Self::into_raw(self) as *mut c_void
    }
}

#[repr(transparent)]
pub struct SyncPtr<T>(*mut T);

impl<T> SyncPtr<T> {
    /// # Safety
    /// It must be valid to send the pointer to other threads and to access the data it
    /// points to concurrently, or the pointer must not be accessed in other threads
    #[inline(always)]
    pub const unsafe fn new(ptr: *mut T) -> Self {
        Self(ptr)
    }

    /// # Safety
    /// It must be valid to send the pointer to other threads and to access the data it
    /// points to concurrently, or the pointer must not be accessed in other threads
    #[inline(always)]
    pub const unsafe fn as_ptr(&self) -> *mut T {
        self.0
    }
}

unsafe impl<T> Send for SyncPtr<T> {}
unsafe impl<T> Sync for SyncPtr<T> {}

pub trait BorrowVal<Borrowed>: AppState {
    /// # Safety
    /// - `raw` must've been obtained from [`AppState::into_raw`] and be valid
    /// - whether or not this is safe to call concurrently depends on both `Self` and `Borrowed`
    /// - the implementation must be sound if `raw` is null (e.g. panic)
    unsafe fn borrow_val<R>(raw: *mut c_void, f: impl FnOnce(Borrowed) -> R) -> R;
}

pub trait BorrowRef<Borrowed>: AppState {
    /// # Safety
    /// - `raw` must've been obtained from [`AppState::into_raw`] and be valid
    /// - whether or not this is safe to call concurrently depends on both `Self` and `Borrowed`
    /// - the implementation must be sound if `raw` is null (e.g. panic)
    unsafe fn borrow_ref<R>(raw: *mut c_void, f: impl FnOnce(&Borrowed) -> R) -> R;
}

pub trait BorrowMut<Borrowed>: AppState {
    /// # Safety
    /// - `raw` must've been obtained from [`AppState::into_raw`] and be valid
    /// - whether or not this is safe to call concurrently depends on both `Self` and `Borrowed`
    /// - the implementation must be sound if `raw` is null (e.g. panic)
    unsafe fn borrow_mut<R>(raw: *mut c_void, f: impl FnOnce(&mut Borrowed) -> R) -> R;
}

pub trait ConsumeVal<Consumed>: AppState {
    /// # Safety
    /// - all borrows from `raw` must end before this function is called
    /// - this calls [`AppState::from_raw`] with `raw`
    unsafe fn consume_val<R>(raw: *mut c_void, f: impl FnOnce(Consumed) -> R) -> R;

    /// # Safety
    /// see [`ConsumeVal::consume_val`]
    #[inline(always)]
    unsafe fn default_consume_val<R>(raw: *mut c_void, f: impl FnOnce(Consumed) -> R) -> R
    where
        Self: BorrowVal<Consumed>,
    {
        let r = unsafe { Self::borrow_val(raw, f) };
        let _ = unsafe { Self::from_raw(raw) };
        r
    }
}

pub trait ConsumeRef<Consumed>: AppState {
    /// # Safety
    /// - all borrows from `raw` must end before this function is called
    /// - this calls [`AppState::from_raw`] with `raw`
    unsafe fn consume_ref<R>(raw: *mut c_void, f: impl FnOnce(&Consumed) -> R) -> R;
}

impl<S: BorrowRef<T>, T> ConsumeRef<T> for S {
    #[inline(always)]
    unsafe fn consume_ref<R>(raw: *mut c_void, f: impl FnOnce(&T) -> R) -> R {
        let r = unsafe { S::borrow_ref(raw, f) };
        let _ = unsafe { S::from_raw(raw) };
        r
    }
}

pub trait ConsumeMut<Consumed>: AppState {
    /// # Safety
    /// - all borrows from `raw` must end before this function is called
    /// - this calls [`AppState::from_raw`] with `raw`
    unsafe fn consume_mut<R>(raw: *mut c_void, f: impl FnOnce(&mut Consumed) -> R) -> R;
}

impl<S: BorrowMut<T>, T> ConsumeMut<T> for S {
    #[inline(always)]
    unsafe fn consume_mut<R>(raw: *mut c_void, f: impl FnOnce(&mut T) -> R) -> R {
        let r = unsafe { S::borrow_mut(raw, f) };
        let _ = unsafe { S::from_raw(raw) };
        r
    }
}

impl<S: AppState> BorrowVal<()> for S {
    #[inline(always)]
    unsafe fn borrow_val<R>(_: *mut c_void, f: impl FnOnce(()) -> R) -> R {
        f(())
    }
}

impl<S: AppState> ConsumeVal<()> for S {
    #[inline(always)]
    unsafe fn consume_val<R>(_: *mut c_void, f: impl FnOnce(()) -> R) -> R {
        f(())
    }
}

impl<S: AppState> BorrowVal<*mut c_void> for S {
    #[inline(always)]
    unsafe fn borrow_val<R>(raw: *mut c_void, f: impl FnOnce(*mut c_void) -> R) -> R {
        f(raw)
    }
}

impl<S: AppState> ConsumeVal<*mut c_void> for S {
    #[inline(always)]
    unsafe fn consume_val<R>(raw: *mut c_void, f: impl FnOnce(*mut c_void) -> R) -> R {
        f(raw)
    }
}

impl<T> BorrowVal<NonNull<T>> for SyncPtr<T>
where
    Self: AppState,
{
    #[inline(always)]
    unsafe fn borrow_val<R>(raw: *mut c_void, f: impl FnOnce(NonNull<T>) -> R) -> R {
        f(NonNull::new(raw as *mut T).unwrap())
    }
}

impl<T> ConsumeVal<NonNull<T>> for SyncPtr<T>
where
    Self: AppState,
{
    #[inline(always)]
    unsafe fn consume_val<R>(raw: *mut c_void, f: impl FnOnce(NonNull<T>) -> R) -> R {
        unsafe { Self::default_consume_val(raw, f) }
    }
}

impl<T> BorrowVal<Option<NonNull<T>>> for SyncPtr<T>
where
    Self: AppState,
{
    #[inline(always)]
    unsafe fn borrow_val<R>(raw: *mut c_void, f: impl FnOnce(Option<NonNull<T>>) -> R) -> R {
        f(NonNull::new(raw as *mut T))
    }
}

impl<T> ConsumeVal<Option<NonNull<T>>> for SyncPtr<T>
where
    Self: AppState,
{
    #[inline(always)]
    unsafe fn consume_val<R>(raw: *mut c_void, f: impl FnOnce(Option<NonNull<T>>) -> R) -> R {
        unsafe { Self::default_consume_val(raw, f) }
    }
}

impl<T> BorrowVal<Self> for SyncPtr<T>
where
    Self: AppState,
{
    #[inline(always)]
    unsafe fn borrow_val<R>(raw: *mut c_void, f: impl FnOnce(Self) -> R) -> R {
        f(SyncPtr(raw as *mut T))
    }
}

impl<T> ConsumeVal<Self> for SyncPtr<T>
where
    Self: AppState,
{
    #[inline(always)]
    unsafe fn consume_val<R>(raw: *mut c_void, f: impl FnOnce(Self) -> R) -> R {
        unsafe { Self::default_consume_val(raw, f) }
    }
}

#[cfg(feature = "alloc")]
impl<T> BorrowVal<NonNull<T>> for Arc<T>
where
    Self: AppState,
{
    #[inline(always)]
    unsafe fn borrow_val<R>(raw: *mut c_void, f: impl FnOnce(NonNull<T>) -> R) -> R {
        f(NonNull::new(raw as *mut T).unwrap())
    }
}

#[cfg(feature = "alloc")]
impl<T> ConsumeVal<NonNull<T>> for Arc<T>
where
    Self: AppState,
{
    #[inline(always)]
    unsafe fn consume_val<R>(raw: *mut c_void, f: impl FnOnce(NonNull<T>) -> R) -> R {
        unsafe { Self::default_consume_val(raw, f) }
    }
}

#[cfg(feature = "alloc")]
impl<T> BorrowRef<T> for Arc<T>
where
    Self: AppState,
{
    #[inline(always)]
    unsafe fn borrow_ref<R>(raw: *mut c_void, f: impl FnOnce(&T) -> R) -> R {
        assert!(!raw.is_null());
        f(unsafe { &*(raw as *mut T) })
    }
}

#[cfg(feature = "alloc")]
impl<T> BorrowVal<Self> for Arc<T>
where
    Self: AppState,
{
    #[inline(always)]
    unsafe fn borrow_val<R>(raw: *mut c_void, f: impl FnOnce(Self) -> R) -> R {
        assert!(!raw.is_null());
        let raw = raw as *mut T;
        unsafe {
            Self::increment_strong_count(raw);
            f(Self::from_raw(raw))
        }
    }
}

#[cfg(feature = "alloc")]
impl<T> ConsumeVal<Self> for Arc<T>
where
    Self: AppState,
{
    #[inline(always)]
    unsafe fn consume_val<R>(raw: *mut c_void, f: impl FnOnce(Self) -> R) -> R {
        unsafe { f(Self::from_raw(raw as *mut T)) }
    }
}

#[cfg(feature = "std")]
impl<T> BorrowRef<T> for Arc<Mutex<T>>
where
    Self: AppState,
{
    #[inline(always)]
    unsafe fn borrow_ref<R>(raw: *mut c_void, f: impl FnOnce(&T) -> R) -> R {
        assert!(!raw.is_null());
        f(&*unsafe { &*(raw as *mut Mutex<T>) }.lock().unwrap())
    }
}

#[cfg(feature = "std")]
impl<T> BorrowMut<T> for Arc<Mutex<T>>
where
    Self: AppState,
{
    #[inline(always)]
    unsafe fn borrow_mut<R>(raw: *mut c_void, f: impl FnOnce(&mut T) -> R) -> R {
        assert!(!raw.is_null());
        f(&mut *unsafe { &*(raw as *mut Mutex<T>) }.lock().unwrap())
    }
}

#[cfg(feature = "std")]
impl<T> BorrowRef<T> for Arc<RwLock<T>>
where
    Self: AppState,
{
    #[inline(always)]
    unsafe fn borrow_ref<R>(raw: *mut c_void, f: impl FnOnce(&T) -> R) -> R {
        assert!(!raw.is_null());
        f(&*unsafe { &*(raw as *mut RwLock<T>) }.read().unwrap())
    }
}

#[cfg(feature = "std")]
impl<T> BorrowMut<T> for Arc<RwLock<T>>
where
    Self: AppState,
{
    #[inline(always)]
    unsafe fn borrow_mut<R>(raw: *mut c_void, f: impl FnOnce(&mut T) -> R) -> R {
        assert!(!raw.is_null());
        f(&mut *unsafe { &*(raw as *mut RwLock<T>) }.write().unwrap())
    }
}

#[cfg(feature = "alloc")]
impl<T> BorrowVal<NonNull<T>> for Box<T>
where
    Self: AppState,
{
    #[inline(always)]
    unsafe fn borrow_val<R>(raw: *mut c_void, f: impl FnOnce(NonNull<T>) -> R) -> R {
        f(NonNull::new(raw as *mut T).unwrap())
    }
}

#[cfg(feature = "alloc")]
impl<T> ConsumeVal<NonNull<T>> for Box<T>
where
    Self: AppState,
{
    #[inline(always)]
    unsafe fn consume_val<R>(raw: *mut c_void, f: impl FnOnce(NonNull<T>) -> R) -> R {
        unsafe { Self::default_consume_val(raw, f) }
    }
}

#[cfg(feature = "alloc")]
impl<T> ConsumeVal<Self> for Box<T>
where
    Self: AppState,
{
    #[inline(always)]
    unsafe fn consume_val<R>(raw: *mut c_void, f: impl FnOnce(Self) -> R) -> R {
        unsafe { f(Self::from_raw(raw as *mut T)) }
    }
}

#[cfg(feature = "alloc")]
impl<T> BorrowRef<T> for Box<T>
where
    Self: AppState,
{
    #[inline(always)]
    unsafe fn borrow_ref<R>(raw: *mut c_void, f: impl FnOnce(&T) -> R) -> R {
        assert!(!raw.is_null());
        f(unsafe { &*(raw as *mut T) })
    }
}

#[cfg(feature = "std")]
impl<T> BorrowRef<T> for Box<Mutex<T>>
where
    Self: AppState,
{
    #[inline(always)]
    unsafe fn borrow_ref<R>(raw: *mut c_void, f: impl FnOnce(&T) -> R) -> R {
        assert!(!raw.is_null());
        f(&*unsafe { &*(raw as *mut Mutex<T>) }.lock().unwrap())
    }
}

#[cfg(feature = "std")]
impl<T> BorrowMut<T> for Box<Mutex<T>>
where
    Self: AppState,
{
    #[inline(always)]
    unsafe fn borrow_mut<R>(raw: *mut c_void, f: impl FnOnce(&mut T) -> R) -> R {
        assert!(!raw.is_null());
        f(&mut *unsafe { &*(raw as *mut Mutex<T>) }.lock().unwrap())
    }
}

#[cfg(feature = "std")]
impl<T> BorrowRef<T> for Box<RwLock<T>>
where
    Self: AppState,
{
    #[inline(always)]
    unsafe fn borrow_ref<R>(raw: *mut c_void, f: impl FnOnce(&T) -> R) -> R {
        assert!(!raw.is_null());
        f(&*unsafe { &*(raw as *mut RwLock<T>) }.read().unwrap())
    }
}

#[cfg(feature = "std")]
impl<T> BorrowMut<T> for Box<RwLock<T>>
where
    Self: AppState,
{
    #[inline(always)]
    unsafe fn borrow_mut<R>(raw: *mut c_void, f: impl FnOnce(&mut T) -> R) -> R {
        assert!(!raw.is_null());
        f(&mut *unsafe { &*(raw as *mut RwLock<T>) }.write().unwrap())
    }
}
