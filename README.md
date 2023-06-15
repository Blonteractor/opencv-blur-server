# Aftershoot Internship Task

This repository contains the solution for the internship task assigned by Aftershoot. The task involves creating a web server that applies Gaussian blur to an image using OpenCV. Additionally, the task requires static and dynamic linking of the OpenCV library and packaging them in a zip file.

## Task Solution

To accomplish the task, the following steps were taken:

### Step 1: Environment Variables Setup

To enable static linking of the OpenCV library, the following environment variables were set:

-   `OPENCV_LINK_LIBS`: Specify the libraries to link with OpenCV.
-   `OPENCV_LINK_PATHS`: Set the path to the OpenCV library files.
-   `OPENCV_INCLUDE_PATHS`: Provide the path to the OpenCV header files.

By setting these environment variables, the Rust build system (Cargo) was informed about the required linking and include paths for the OpenCV library.

### Step 2: Rust Build Configuration

In the build script, the `lib/` folder was specified as the location to look for libraries at runtime. This ensures that the built application can find the necessary OpenCV libraries during execution.

```rust
// build.rs

fn main() {

    // Specify the runtime library search path for OpenCV libraries
    println!("cargo:rustc-link-arg=-Wl,-rpath,./lib");
}
```

### Step 3: Web Server Implementation

Upon receiving an image file through the `/applyblur` endpoint, the server utilized the OpenCV library to apply Gaussian blur to the image. The specific OpenCV functions and parameters for Gaussian blur were used to achieve the desired effect.
