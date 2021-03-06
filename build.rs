// fn main() {

// }

extern crate gcc;
extern crate bindgen;

use std::path::PathBuf;

fn main() {
    // let bindings = bindgen::Builder::default()
    //     .no_unstable_rust()
    //     .header("src-cpp/pinset.hpp")
    //     .layout_tests(false)
    //     .use_core()
    //     .generate()
    //     .expect("Unable to generate bindings");

    // let out_path = PathBuf::from("src");
    // bindings
    //     .write_to_file(out_path.join("lib.pinset.rs"))
    //     .expect("Couldn't write bindings!");

    let firmware_dir = "/Users/pete/dev/particle/firmware";
    gcc::Config::new()
        .cpp(false) // Switch to C++ library compilation.
        .compiler("arm-none-eabi-gcc")
        .target("thumbv7m-none-eabi")
        .flag(&format!("-I{}/{}", firmware_dir, "user/inc"))
        .flag(&format!("-I{}/{}", firmware_dir, "inc"))
        .flag(&format!("-I{}/{}", firmware_dir, "wiring/inc"))
        .flag(&format!("-I{}/{}", firmware_dir, "system/inc"))
        .flag(&format!("-I{}/{}", firmware_dir, "services/inc"))
        .flag(&format!("-I{}/{}", firmware_dir, "communication/src"))
        .flag(&format!("-I{}/{}", firmware_dir, "hal/inc"))
        .flag(&format!("-I{}/{}", firmware_dir, "hal/shared"))
        .flag(&format!("-I{}/{}", firmware_dir, "hal/src/photon"))
        .flag(&format!("-I{}/{}", firmware_dir, "hal/src/stm32f2xx"))
        .flag(&format!("-I{}/{}", firmware_dir, "hal/src/stm32"))
        .flag(&format!("-I{}/{}", firmware_dir, "hal/src/photon/api"))
        .flag(&format!("-I{}/{}", firmware_dir, "platform/shared/inc"))
        .flag(&format!("-I{}/{}", firmware_dir, "platform/MCU/STM32F2xx/CMSIS/Include"))
        .flag(&format!("-I{}/{}", firmware_dir, "platform/MCU/STM32F2xx/CMSIS/Device/ST/Include"))
        .flag(&format!("-I{}/{}", firmware_dir, "platform/MCU/STM32F2xx/SPARK_Firmware_Driver/inc"))
        .flag(&format!("-I{}/{}", firmware_dir, "platform/MCU/shared/STM32/inc"))
        .flag(&format!("-I{}/{}", firmware_dir, "platform/MCU/STM32F2xx/STM32_StdPeriph_Driver/inc"))
        .flag(&format!("-I{}/{}", firmware_dir, "platform/MCU/STM32F2xx/STM32_USB_Device_Driver/inc"))
        .flag(&format!("-I{}/{}", firmware_dir, "platform/MCU/STM32F2xx/STM32_USB_Host_Driver/inc"))
        .flag(&format!("-I{}/{}", firmware_dir, "platform/MCU/STM32F2xx/STM32_USB_OTG_Driver/inc"))
        .flag(&format!("-I{}/{}", firmware_dir, "dynalib/inc"))

        .flag("-DSTM32_DEVICE")
        .flag("-DSTM32F2XX")
        .flag("-DPLATFORM_THREADING=1")
        .flag("-DPLATFORM_ID=6")
        .flag("-DPLATFORM_NAME=photon")
        .flag("-DUSBD_VID_SPARK=0x2B04")
        .flag("-DUSBD_PID_DFU=0xD006")
        .flag("-DUSBD_PID_CDC=0xC006")
        .flag("-DSPARK_PLATFORM")
        .flag("-g3")
        .flag("-gdwarf-2")
        .flag("-Os")
        .flag("-mcpu=cortex-m3")
        .flag("-mthumb")
        .flag("-DINCLUDE_PLATFORM=1")
        .flag("-DPRODUCT_ID=6")
        .flag("-DPRODUCT_FIRMWARE_VERSION=65535")
        .flag("-DUSE_STDPERIPH_DRIVER")
        .flag("-DDFU_BUILD_ENABLE")
        .flag("-DSYSTEM_VERSION_STRING=0.6.2")
        .flag("-DRELEASE_BUILD")

        .flag("-ffunction-sections")
        .flag("-fdata-sections")
        .flag("-Wall")
        .flag("-Wno-switch")
        .flag("-Wno-error=deprecated-declarations")
        .flag("-fmessage-length=0")
        .flag("-fno-strict-aliasing")
        .flag("-DSPARK=1")
        .flag("-DPARTICLE=1")
        .flag("-DSTART_DFU_FLASHER_SERIAL_SPEED=14400")
        .flag("-DSTART_YMODEM_FLASHER_SERIAL_SPEED=28800")
        .flag("-DSPARK_PLATFORM_NET=CC3000")
        .flag("-fno-builtin-malloc")
        .flag("-fno-builtin-free")
        .flag("-fno-builtin-realloc")
        .flag("-DLOG_INCLUDE_SOURCE_INFO=1")
        .flag("-DPARTICLE_USER_MODULE")
        .flag("-DUSE_THREADING=0")
        .flag("-DUSE_SPI=SPI")
        .flag("-DUSE_CS=A2")
        .flag("-DMODULE_VERSION=0")
        .flag("-DMODULE_FUNCTION=3")
        .flag("-DMODULE_DEPENDENCY=0,0,0")
        .flag("-D_WINSOCK_H")
        .flag("-D_GNU_SOURCE")
        .flag("-DLOG_MODULE_CATEGORY=\"\\\"app\\\"\"")
        .flag("-fno-exceptions")
        .flag("-fno-rtti")
        .flag("-fcheck-new")
        .flag("-std=gnu++11")

        .file("src-cpp/pinset.cpp")
        .compile("libpinset.a");
}
