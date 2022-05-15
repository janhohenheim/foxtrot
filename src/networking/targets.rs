mod shared;

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(not(target_arch = "wasm32"))]
mod native;

#[cfg(not(target_arch = "wasm32"))]
pub type PlatformPlugin = native::NativePlugin;

#[cfg(target_arch = "wasm32")]
pub type PlatformPlugin = wasm::WasmPlugin;

#[cfg(not(target_arch = "wasm32"))]
pub type PlatformConfig = native::NativeConfig;

#[cfg(target_arch = "wasm32")]
pub type PlatformConfig = wasm::WasmConfig;
