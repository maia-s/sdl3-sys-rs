//! Functions for creating Vulkan surfaces on SDL windows.

use super::stdinc::*;

use super::error::*;

use super::video::*;

// [sdl3-sys-gen] skipped function-like define `VK_DEFINE_HANDLE`

#[cfg(any(any(/* always disabled: _M_IA64 */), target_arch = "x86_64", all(windows, target_pointer_width = "64"), all(not(windows), target_pointer_width = "64"), target_arch = "aarch64", any(/* always disabled: __ia64 */), target_arch = "powerpc64", target_arch = "x86_64"))]
emit! {
    // [sdl3-sys-gen] skipped function-like define `VK_DEFINE_NON_DISPATCHABLE_HANDLE`

}

#[cfg(not(any(any(/* always disabled: _M_IA64 */), target_arch = "x86_64", all(windows, target_pointer_width = "64"), all(not(windows), target_pointer_width = "64"), target_arch = "aarch64", any(/* always disabled: __ia64 */), target_arch = "powerpc64", target_arch = "x86_64")))]
emit! {
    // [sdl3-sys-gen] skipped function-like define `VK_DEFINE_NON_DISPATCHABLE_HANDLE`

}

pub type VkInstance = *mut __VkInstance;

#[repr(C)]
#[non_exhaustive]
pub struct __VkInstance {
    _opaque: [::core::primitive::u8; 0],
}

pub type VkPhysicalDevice = *mut __VkPhysicalDevice;

#[repr(C)]
#[non_exhaustive]
pub struct __VkPhysicalDevice {
    _opaque: [::core::primitive::u8; 0],
}

#[cfg(target_pointer_width = "64")]
pub type VkSurfaceKHR = *mut __VkSurfaceKHR;

#[cfg(target_pointer_width = "64")]
#[repr(C)]
#[non_exhaustive]
pub struct __VkSurfaceKHR {
    _opaque: [::core::primitive::u8; 0],
}

#[cfg(not(target_pointer_width = "64"))]
pub type VkSurfaceKHR = ::core::primitive::u64;

extern "C" {
    /// Dynamically load the Vulkan loader library.
    ///
    /// This should be called after initializing the video driver, but before
    /// creating any Vulkan windows. If no Vulkan loader library is loaded, the
    /// default library will be loaded upon creation of the first Vulkan window.
    ///
    /// It is fairly common for Vulkan applications to link with libvulkan instead
    /// of explicitly loading it at run time. This will work with SDL provided the
    /// application links to a dynamic library and both it and SDL use the same
    /// search path.
    ///
    /// If you specify a non-NULL `path`, an application should retrieve all of the
    /// Vulkan functions it uses from the dynamic library using
    /// SDL_Vulkan_GetVkGetInstanceProcAddr unless you can guarantee `path` points
    /// to the same vulkan loader library the application linked to.
    ///
    /// On Apple devices, if `path` is NULL, SDL will attempt to find the
    /// `vkGetInstanceProcAddr` address within all the Mach-O images of the current
    /// process. This is because it is fairly common for Vulkan applications to
    /// link with libvulkan (and historically MoltenVK was provided as a static
    /// library). If it is not found, on macOS, SDL will attempt to load
    /// `vulkan.framework/vulkan`, `libvulkan.1.dylib`,
    /// `MoltenVK.framework/MoltenVK`, and `libMoltenVK.dylib`, in that order. On
    /// iOS, SDL will attempt to load `libMoltenVK.dylib`. Applications using a
    /// dynamic framework or .dylib must ensure it is included in its application
    /// bundle.
    ///
    /// On non-Apple devices, application linking with a static libvulkan is not
    /// supported. Either do not link to the Vulkan loader or link to a dynamic
    /// library version.
    ///
    /// \param path the platform dependent Vulkan loader library name or NULL.
    /// \returns true on success or false on failure; call SDL_GetError() for more
    ///          information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_Vulkan_GetVkGetInstanceProcAddr
    /// \sa SDL_Vulkan_UnloadLibrary
    pub fn SDL_Vulkan_LoadLibrary(path: *const ::core::ffi::c_char) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the address of the `vkGetInstanceProcAddr` function.
    ///
    /// This should be called after either calling SDL_Vulkan_LoadLibrary() or
    /// creating an SDL_Window with the `SDL_WINDOW_VULKAN` flag.
    ///
    /// The actual type of the returned function pointer is
    /// PFN_vkGetInstanceProcAddr, but that isn't available because the Vulkan
    /// headers are not included here. You should cast the return value of this
    /// function to that type, e.g.
    ///
    /// `vkGetInstanceProcAddr =
    /// (PFN_vkGetInstanceProcAddr)SDL_Vulkan_GetVkGetInstanceProcAddr();`
    ///
    /// \returns the function pointer for `vkGetInstanceProcAddr` or NULL on
    ///          failure; call SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_Vulkan_GetVkGetInstanceProcAddr() -> SDL_FunctionPointer;
}

extern "C" {
    /// Unload the Vulkan library previously loaded by SDL_Vulkan_LoadLibrary().
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_Vulkan_LoadLibrary
    pub fn SDL_Vulkan_UnloadLibrary();
}

extern "C" {
    /// Get the Vulkan instance extensions needed for vkCreateInstance.
    ///
    /// This should be called after either calling SDL_Vulkan_LoadLibrary() or
    /// creating an SDL_Window with the `SDL_WINDOW_VULKAN` flag.
    ///
    /// On return, the variable pointed to by `count` will be set to the number of
    /// elements returned, suitable for using with
    /// VkInstanceCreateInfo::enabledExtensionCount, and the returned array can be
    /// used with VkInstanceCreateInfo::ppEnabledExtensionNames, for calling
    /// Vulkan's vkCreateInstance API.
    ///
    /// You should not free the returned array; it is owned by SDL.
    ///
    /// \param count a pointer filled in with the number of extensions returned.
    /// \returns an array of extension name strings on success, NULL on failure;
    ///          call SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_Vulkan_CreateSurface
    pub fn SDL_Vulkan_GetInstanceExtensions(
        count: *mut Uint32,
    ) -> *const *const ::core::ffi::c_char;
}

extern "C" {
    /// Create a Vulkan rendering surface for a window.
    ///
    /// The `window` must have been created with the `SDL_WINDOW_VULKAN` flag and
    /// `instance` must have been created with extensions returned by
    /// SDL_Vulkan_GetInstanceExtensions() enabled.
    ///
    /// If `allocator` is NULL, Vulkan will use the system default allocator. This
    /// argument is passed directly to Vulkan and isn't used by SDL itself.
    ///
    /// \param window the window to which to attach the Vulkan surface.
    /// \param instance the Vulkan instance handle.
    /// \param allocator a VkAllocationCallbacks struct, which lets the app set the
    ///                  allocator that creates the surface. Can be NULL.
    /// \param surface a pointer to a VkSurfaceKHR handle to output the newly
    ///                created surface.
    /// \returns true on success or false on failure; call SDL_GetError() for more
    ///          information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_Vulkan_GetInstanceExtensions
    /// \sa SDL_Vulkan_DestroySurface
    pub fn SDL_Vulkan_CreateSurface(
        window: *mut SDL_Window,
        instance: VkInstance,
        allocator: *const VkAllocationCallbacks,
        surface: *mut VkSurfaceKHR,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Destroy the Vulkan rendering surface of a window.
    ///
    /// This should be called before SDL_DestroyWindow, if SDL_Vulkan_CreateSurface
    /// was called after SDL_CreateWindow.
    ///
    /// The `instance` must have been created with extensions returned by
    /// SDL_Vulkan_GetInstanceExtensions() enabled and `surface` must have been
    /// created successfully by an SDL_Vulkan_CreateSurface() call.
    ///
    /// If `allocator` is NULL, Vulkan will use the system default allocator. This
    /// argument is passed directly to Vulkan and isn't used by SDL itself.
    ///
    /// \param instance the Vulkan instance handle.
    /// \param surface vkSurfaceKHR handle to destroy.
    /// \param allocator a VkAllocationCallbacks struct, which lets the app set the
    ///                  allocator that destroys the surface. Can be NULL.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_Vulkan_GetInstanceExtensions
    /// \sa SDL_Vulkan_CreateSurface
    pub fn SDL_Vulkan_DestroySurface(
        instance: VkInstance,
        surface: VkSurfaceKHR,
        allocator: *const VkAllocationCallbacks,
    );
}

extern "C" {
    /// Query support for presentation via a given physical device and queue
    /// family.
    ///
    /// The `instance` must have been created with extensions returned by
    /// SDL_Vulkan_GetInstanceExtensions() enabled.
    ///
    /// \param instance the Vulkan instance handle.
    /// \param physicalDevice a valid Vulkan physical device handle.
    /// \param queueFamilyIndex a valid queue family index for the given physical
    ///                         device.
    /// \returns true if supported, false if unsupported or an error occurred.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_Vulkan_GetInstanceExtensions
    pub fn SDL_Vulkan_GetPresentationSupport(
        instance: VkInstance,
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: Uint32,
    ) -> ::core::primitive::bool;
}

#[repr(C)]
#[non_exhaustive]
pub struct VkAllocationCallbacks {
    _opaque: [::core::primitive::u8; 0],
}
