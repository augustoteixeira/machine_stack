use cartesi_machine::config::machine::{
    MachineConfig, MemoryRangeConfig, RAMConfig,
};

use cartesi_machine::config::machine::DTBConfig;
use cartesi_machine::config::runtime::{HTIFRuntimeConfig, RuntimeConfig};
use cartesi_machine::types::cmio::{CmioRequest, ManualReason};
use cartesi_machine::Machine;

fn main() {
    let machine_config =         MachineConfig::new_with_ram(RAMConfig {
            length: 134217728,
            image_filename: "linux.bin".into(),
        })
        .dtb(DTBConfig {
            entrypoint:
                r#"echo '{"domain":16, "id":"'$(echo -n Hello from inside! | hex --encode)'"}' \
                     | rollup gio #| grep -Eo '0x[0-9a-f]+' | tr -d '\n' | hex --decode; echo"#
                    .to_string(),
            ..Default::default()
        })
        .add_flash_drive(MemoryRangeConfig {
            image_filename: "rootfs.ext2".into(),
            ..Default::default()
        });
    let runtime_config = RuntimeConfig {
        htif: Some(HTIFRuntimeConfig {
            no_console_putchar: Some(false),
        }),
        ..Default::default()
    };


    let mut machine =
        Machine::create(&machine_config, &runtime_config).unwrap();

    machine.run(u64::MAX).unwrap();
    println!("{}",matches!(machine.receive_cmio_request().unwrap(), CmioRequest::Manual(ManualReason::GIO {domain:16, .. })));
    //let a = [0x67, 0x69, 0x6F, 0x2D, 0x72, 0x65, 0x70, 0x6C, 0x79, 0x2D, 0x30];
    let a = "Hi from inside".as_bytes();
    //let a = "0x00000000".as_bytes();
    machine.send_cmio_response(cartesi_machine::types::cmio::CmioResponseReason::Inspect, &a).unwrap();
    machine.run(u64::MAX).unwrap();
}

#[cfg(test)]
mod tests {
    use cartesi_machine::config::machine::{
        MachineConfig, MemoryRangeConfig, RAMConfig,
    };

    use cartesi_machine::config::machine::DTBConfig;
    use cartesi_machine::config::runtime::{HTIFRuntimeConfig, RuntimeConfig};
    use cartesi_machine::Machine;

    #[test]
    fn test_print() {
        let machine_config = MachineConfig::new_with_ram(RAMConfig {
            length: 134217728,
            image_filename: "linux.bin".into(),
        })
        .dtb(DTBConfig {
            entrypoint: "echo Hello from inside!".to_string(),
            ..Default::default()
        })
        .add_flash_drive(MemoryRangeConfig {
            image_filename: "rootfs.ext2".into(),
            ..Default::default()
        });
        let runtime_config = RuntimeConfig {
            htif: Some(HTIFRuntimeConfig {
                no_console_putchar: Some(false),
            }),
            ..Default::default()
        };

        let mut machine =
            Machine::create(&machine_config, &runtime_config).unwrap();

        machine.run(u64::MAX).unwrap();
    }
}
