# vma-sys

A ffi wrapper around the Vulkan Memory Allocator API.

## Supported Platforms

As of now, only Windows is being tested and supported.

## Building

This crate currently requires the Vulkan SDK with the VMA headers to be installed on your system.
The VULKAN_SDK environment variable is used to locate the SDK.
You can download it from the [LunarG website](https://vulkan.lunarg.com/sdk/home).

This crate also requires the vulkan-sys crate to be in an adjacent directory.