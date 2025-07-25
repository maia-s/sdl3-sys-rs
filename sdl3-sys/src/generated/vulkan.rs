//! Functions for creating Vulkan surfaces on SDL windows.
//!
//! For the most part, Vulkan operates independent of SDL, but it benefits from
//! a little support during setup.
//!
//! Use [`SDL_Vulkan_GetInstanceExtensions()`] to get platform-specific bits for
//! creating a VkInstance, then [`SDL_Vulkan_GetVkGetInstanceProcAddr()`] to get
//! the appropriate function for querying Vulkan entry points. Then
//! [`SDL_Vulkan_CreateSurface()`] will get you the final pieces you need to
//! prepare for rendering into an [`SDL_Window`] with Vulkan.
//!
//! Unlike OpenGL, most of the details of "context" creation and window buffer
//! swapping are handled by the Vulkan API directly, so SDL doesn't provide
//! Vulkan equivalents of [`SDL_GL_SwapWindow()`], etc; they aren't necessary.

use super::stdinc::*;

use super::error::*;

use super::video::*;

apply_cfg!(#[cfg(any(all(not(target_pointer_width = "32"), target_arch = "x86_64"), all(target_arch = "riscv64", any(target_arch = "riscv32", target_arch = "riscv64")), any(/* always disabled: _M_IA64 */), target_arch = "x86_64", all(windows, target_pointer_width = "64"), all(not(windows), target_pointer_width = "64"), target_arch = "aarch64", any(/* always disabled: __ia64 */), target_arch = "powerpc64"))] => {
});

apply_cfg!(#[cfg(not(any(all(not(target_pointer_width = "32"), target_arch = "x86_64"), all(target_arch = "riscv64", any(target_arch = "riscv32", target_arch = "riscv64")), any(/* always disabled: _M_IA64 */), target_arch = "x86_64", all(windows, target_pointer_width = "64"), all(not(windows), target_pointer_width = "64"), target_arch = "aarch64", any(/* always disabled: __ia64 */), target_arch = "powerpc64")))] => {
});

apply_cfg!(#[cfg(feature = "use-ash-v0-38")] => {
    #[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
    /// (`sdl3-sys`) Enable a `use-ash-*` feature to alias this to `vk::Instance` from the `ash` crate. Otherwise it's a pointer to an opaque struct.
    pub type VkInstance = ::ash_v0_38::vk::Instance;
});

apply_cfg!(#[cfg(not(feature = "use-ash-v0-38"))] => {
    #[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
    /// (`sdl3-sys`) Enable a `use-ash-*` feature to alias this to `vk::Instance` from the `ash` crate. Otherwise it's a pointer to an opaque struct.
    pub type VkInstance = *mut __VkInstance;

    #[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
    #[doc(hidden)]
    #[repr(C)]
    pub struct __VkInstance { _opaque: [::core::primitive::u8; 0] }
});

apply_cfg!(#[cfg(feature = "use-ash-v0-38")] => {
    #[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
    /// (`sdl3-sys`) Enable a `use-ash-*` feature to alias this to `vk::PhysicalDevice` from the `ash` crate. Otherwise it's a pointer to an opaque struct.
    pub type VkPhysicalDevice = ::ash_v0_38::vk::PhysicalDevice;
});

apply_cfg!(#[cfg(not(feature = "use-ash-v0-38"))] => {
    #[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
    /// (`sdl3-sys`) Enable a `use-ash-*` feature to alias this to `vk::PhysicalDevice` from the `ash` crate. Otherwise it's a pointer to an opaque struct.
    pub type VkPhysicalDevice = *mut __VkPhysicalDevice;

    #[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
    #[doc(hidden)]
    #[repr(C)]
    pub struct __VkPhysicalDevice { _opaque: [::core::primitive::u8; 0] }
});

apply_cfg!(#[cfg(feature = "use-ash-v0-38")] => {
    #[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
    /// (`sdl3-sys`) Enable a `use-ash-*` feature to alias this to `vk::SurfaceKHR` from the `ash` crate. Otherwise it's a target dependent opaque type.
    pub type VkSurfaceKHR = ::ash_v0_38::vk::SurfaceKHR;
});

apply_cfg!(#[cfg(not(feature = "use-ash-v0-38"))] => {
    #[cfg(target_pointer_width = "64")]
    #[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
    pub type VkSurfaceKHR = *mut __VkSurfaceKHR;

    #[cfg(target_pointer_width = "64")]
    #[doc(hidden)]
    #[repr(C)]
    pub struct __VkSurfaceKHR { _opaque: [::core::primitive::u8; 0] }

    #[cfg(not(target_pointer_width = "64"))]
    #[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
    /// (`sdl3-sys`) Enable a `use-ash-*` feature to alias this to `vk::SurfaceKHR` from the `ash` crate. Otherwise it's a target dependent opaque type.
    pub type VkSurfaceKHR = ::core::primitive::u64;
});

extern "C" {
    /// Dynamically load the Vulkan loader library.
    ///
    /// This should be called after initializing the video driver, but before
    /// creating any Vulkan windows. If no Vulkan loader library is loaded, the
    /// default library will be loaded upon creation of the first Vulkan window.
    ///
    /// SDL keeps a counter of how many times this function has been successfully
    /// called, so it is safe to call this function multiple times, so long as it
    /// is eventually paired with an equivalent number of calls to
    /// [`SDL_Vulkan_UnloadLibrary`]. The `path` argument is ignored unless there is no
    /// library currently loaded, and and the library isn't actually unloaded until
    /// there have been an equivalent number of calls to [`SDL_Vulkan_UnloadLibrary`].
    ///
    /// It is fairly common for Vulkan applications to link with libvulkan instead
    /// of explicitly loading it at run time. This will work with SDL provided the
    /// application links to a dynamic library and both it and SDL use the same
    /// search path.
    ///
    /// If you specify a non-NULL `path`, an application should retrieve all of the
    /// Vulkan functions it uses from the dynamic library using
    /// [`SDL_Vulkan_GetVkGetInstanceProcAddr`] unless you can guarantee `path` points
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
    /// ## Parameters
    /// - `path`: the platform dependent Vulkan loader library name or NULL.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Thread safety
    /// This function is not thread safe.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_Vulkan_GetVkGetInstanceProcAddr`]
    /// - [`SDL_Vulkan_UnloadLibrary`]
    pub fn SDL_Vulkan_LoadLibrary(path: *const ::core::ffi::c_char) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the address of the `vkGetInstanceProcAddr` function.
    ///
    /// This should be called after either calling [`SDL_Vulkan_LoadLibrary()`] or
    /// creating an [`SDL_Window`] with the [`SDL_WINDOW_VULKAN`] flag.
    ///
    /// The actual type of the returned function pointer is
    /// PFN_vkGetInstanceProcAddr, but that isn't available because the Vulkan
    /// headers are not included here. You should cast the return value of this
    /// function to that type, e.g.
    ///
    /// `vkGetInstanceProcAddr =
    /// (PFN_vkGetInstanceProcAddr)SDL_Vulkan_GetVkGetInstanceProcAddr();`
    ///
    /// ## Return value
    /// Returns the function pointer for `vkGetInstanceProcAddr` or NULL on
    ///   failure; call [`SDL_GetError()`] for more information.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## Notes for `sdl3-sys`
    /// As of `sdl3-sys` 0.6, the return type is correct for known targets, so you don't have
    /// to cast it before use. It's compatible with the `ash` crate if you enable an integration
    /// feature for that, but note that the type alias this function returns is an Option.
    ///
    pub fn SDL_Vulkan_GetVkGetInstanceProcAddr() -> VkGetInstanceProcAddr;
}

/// (`sdl3-sys`) The definition of the `VkInstance` argument type can change based on enabled features. See [`VkInstance`].
pub type VkGetInstanceProcAddr =
    Option<unsafe extern "system" fn(VkInstance, *const ::core::ffi::c_char) -> VkVoidFunction>;

/// (`sdl3-sys`) Generic Vulkan void function. Cast to the appropriate type before use.
pub type VkVoidFunction = Option<unsafe extern "system" fn()>;

extern "C" {
    /// Unload the Vulkan library previously loaded by [`SDL_Vulkan_LoadLibrary()`].
    ///
    /// SDL keeps a counter of how many times this function has been called, so it
    /// is safe to call this function multiple times, so long as it is paired with
    /// an equivalent number of calls to [`SDL_Vulkan_LoadLibrary`]. The library isn't
    /// actually unloaded until there have been an equivalent number of calls to
    /// [`SDL_Vulkan_UnloadLibrary`].
    ///
    /// Once the library has actually been unloaded, if any Vulkan instances
    /// remain, they will likely crash the program. Clean up any existing Vulkan
    /// resources, and destroy appropriate windows, renderers and GPU devices
    /// before calling this function.
    ///
    /// ## Thread safety
    /// This function is not thread safe.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_Vulkan_LoadLibrary`]
    pub fn SDL_Vulkan_UnloadLibrary();
}

extern "C" {
    /// Get the Vulkan instance extensions needed for vkCreateInstance.
    ///
    /// This should be called after either calling [`SDL_Vulkan_LoadLibrary()`] or
    /// creating an [`SDL_Window`] with the [`SDL_WINDOW_VULKAN`] flag.
    ///
    /// On return, the variable pointed to by `count` will be set to the number of
    /// elements returned, suitable for using with
    /// VkInstanceCreateInfo::enabledExtensionCount, and the returned array can be
    /// used with VkInstanceCreateInfo::ppEnabledExtensionNames, for calling
    /// Vulkan's vkCreateInstance API.
    ///
    /// You should not free the returned array; it is owned by SDL.
    ///
    /// ## Parameters
    /// - `count`: a pointer filled in with the number of extensions returned.
    ///
    /// ## Return value
    /// Returns an array of extension name strings on success, NULL on failure;
    ///   call [`SDL_GetError()`] for more information.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_Vulkan_CreateSurface`]
    pub fn SDL_Vulkan_GetInstanceExtensions(
        count: *mut Uint32,
    ) -> *const *const ::core::ffi::c_char;
}

extern "C" {
    /// Create a Vulkan rendering surface for a window.
    ///
    /// The `window` must have been created with the [`SDL_WINDOW_VULKAN`] flag and
    /// `instance` must have been created with extensions returned by
    /// [`SDL_Vulkan_GetInstanceExtensions()`] enabled.
    ///
    /// If `allocator` is NULL, Vulkan will use the system default allocator. This
    /// argument is passed directly to Vulkan and isn't used by SDL itself.
    ///
    /// ## Parameters
    /// - `window`: the window to which to attach the Vulkan surface.
    /// - `instance`: the Vulkan instance handle.
    /// - `allocator`: a VkAllocationCallbacks struct, which lets the app set the
    ///   allocator that creates the surface. Can be NULL.
    /// - `surface`: a pointer to a VkSurfaceKHR handle to output the newly
    ///   created surface.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_Vulkan_GetInstanceExtensions`]
    /// - [`SDL_Vulkan_DestroySurface`]
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
    /// This should be called before [`SDL_DestroyWindow`], if [`SDL_Vulkan_CreateSurface`]
    /// was called after [`SDL_CreateWindow`].
    ///
    /// The `instance` must have been created with extensions returned by
    /// [`SDL_Vulkan_GetInstanceExtensions()`] enabled and `surface` must have been
    /// created successfully by an [`SDL_Vulkan_CreateSurface()`] call.
    ///
    /// If `allocator` is NULL, Vulkan will use the system default allocator. This
    /// argument is passed directly to Vulkan and isn't used by SDL itself.
    ///
    /// ## Parameters
    /// - `instance`: the Vulkan instance handle.
    /// - `surface`: vkSurfaceKHR handle to destroy.
    /// - `allocator`: a VkAllocationCallbacks struct, which lets the app set the
    ///   allocator that destroys the surface. Can be NULL.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_Vulkan_GetInstanceExtensions`]
    /// - [`SDL_Vulkan_CreateSurface`]
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
    /// [`SDL_Vulkan_GetInstanceExtensions()`] enabled.
    ///
    /// ## Parameters
    /// - `instance`: the Vulkan instance handle.
    /// - `physicalDevice`: a valid Vulkan physical device handle.
    /// - `queueFamilyIndex`: a valid queue family index for the given physical
    ///   device.
    ///
    /// ## Return value
    /// Returns true if supported, false if unsupported or an error occurred.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_Vulkan_GetInstanceExtensions`]
    pub fn SDL_Vulkan_GetPresentationSupport(
        instance: VkInstance,
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: Uint32,
    ) -> ::core::primitive::bool;
}

apply_cfg!(#[cfg(feature = "use-ash-v0-38")] => {
    #[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
    /// (`sdl3-sys`) Enable a `use-ash-*` feature to alias this to `vk::AllocationCallbacks::<'static>` from the `ash` crate. Otherwise it's an opaque type. <div class="warning">The `'static` lifetime is too long. `ash` requires a lifetime for this, but as it's a C ffi type there's no way for `sdl3-sys` to set the correct lifetime.</div>
    pub type VkAllocationCallbacks = ::ash_v0_38::vk::AllocationCallbacks::<'static>;
});

apply_cfg!(#[cfg(not(feature = "use-ash-v0-38"))] => {
    #[cfg_attr(all(feature = "nightly", doc), doc(cfg(all())))]
    /// (`sdl3-sys`) Enable a `use-ash-*` feature to alias this to `vk::AllocationCallbacks::<'static>` from the `ash` crate. Otherwise it's an opaque type. <div class="warning">The `'static` lifetime is too long. `ash` requires a lifetime for this, but as it's a C ffi type there's no way for `sdl3-sys` to set the correct lifetime.</div>
    #[repr(C)]
    pub struct VkAllocationCallbacks { _opaque: [::core::primitive::u8; 0] }
});

#[cfg(doc)]
use crate::everything::*;
