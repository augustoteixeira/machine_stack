use cartesi_machine::config::machine::{
    MachineConfig, MemoryRangeConfig, RAMConfig,
};

use cartesi_machine::config::machine::DTBConfig;
use cartesi_machine::config::runtime::{HTIFRuntimeConfig, RuntimeConfig};
use cartesi_machine::types::cmio::{
    CmioRequest, CmioResponseReason, ManualReason,
};
use cartesi_machine::Machine;

const TEST_GIO: &str =
    r#"echo '{ "domain": 16,
               "id": "'$(echo -n Hi from inside! | hex --encode)'"}' \
       | rollup gio | grep -Eo '0x[0-9a-f]+' | tr -d '\n' \
       | hex --decode; echo
    "#;

const TEST_HELLO: &str = r#"rullup"#;

fn main() {
    let machine_config = MachineConfig::new_with_ram(RAMConfig {
        length: 134217728,
        image_filename: "linux.bin".into(),
    })
    .dtb(DTBConfig {
        entrypoint: "./test-cross; rollup accept".to_string(),
        // entrypoint: TEST_GIO.to_string(),
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
    if let CmioRequest::Manual(ManualReason::GIO { domain: 16, data }) =
        machine.receive_cmio_request().unwrap()
    {
        println!("Received gio request for data: {:?}", data);
    }

    let a = "Hi from inside".as_bytes();
    machine
        .send_cmio_response(CmioResponseReason::Inspect, &a)
        .unwrap();
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
