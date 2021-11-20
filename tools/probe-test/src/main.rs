#[macro_use]
extern crate log;

use env_logger::Env;
use probe_rs::{Probe, Core, Session, CoreRegisterAddress, MemoryInterface};
use probe_rs::flashing::{
    Format,
    download_file_with_options,
    DownloadOptions,
};

use std::path::Path;
use std::thread::sleep_ms;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("probe=debug,probe_rs=debug,probe_rs::probe::jlink::swd=info")).init();

    info!("Connecting to Apollo3 MCU through first attached probe..");

    let mut session = Session::auto_attach("AMA3B1KK-KBR")?;

    {
        info!("Select a core");
        let mut core = session.core(0)?;

        info!("Trying to halt and continue core..");
        core.halt(Duration::from_millis(100))?;

        print_core_status(&mut core);
        assert!(core.core_halted().unwrap());

        info!("Resuming core..");
        core.run()?;

        print_core_status(&mut core);
        assert!(!core.core_halted().unwrap());

        info!("Try reset_and_halt");
        core.reset_and_halt(Duration::from_millis(500))?;
        print_core_status(&mut core);
        assert!(core.core_halted().unwrap());

        info!("Write and readback test RAM");
        // Try and write some sample data
        let data = (0..1024).collect::<Vec<u32>>();
        assert_eq!(data.len(), 1024);

        core.write_32(0x1000_5000, &data)?;

        // Reading back and verifying.
        let mut read_back = vec![0u32; 1024];
        core.read_32(0x1000_5000, &mut read_back)?;

        assert_eq!(data, read_back);

        // info!("Write and readback test FLASH");
        // // Can we write to flash?... no obviously not.
        // core.write_32(0x10_00c0, &[1u32, 2, 4])?;
    }

    // Should now be ready for flashing.
    // let mut mm = session.memory_map();
    let elf = "/home/gauteh/dev/ambiq-rs/boards/redboard-nano/target/thumbv7em-none-eabihf/debug/examples/rtt";
    flash_test(&mut session, elf)?;

    {
        let mut core = session.core(0)?;
        info!("Trying to resume (reset)..");
        core.reset()?;

        print_core_status(&mut core);
        assert!(!core.core_halted().unwrap());
    }

    Ok(())
}

pub fn flash(session: &mut Session, elf: &str) -> anyhow::Result<()> {
    warn!("Trying to flash: {}", elf);

    let mut opts = DownloadOptions::default();
    opts.verify = true;

    download_file_with_options(
        session,
        Path::new(elf),
        Format::Elf,
        opts)?;

    warn!("Done flashing: {}", elf);
    Ok(())
}

pub fn flash_test(session: &mut Session, elf: &str) -> anyhow::Result<()> {
    warn!("Trying flash_test: {}", elf);

    let mut opts = DownloadOptions::default();

    let mut loader = session.target().flash_loader();
    loader.add_data(0x10_000, &[1, 2, 3, 4])?;
    loader.commit(session, opts)?;

    let mut core = session.core(0)?;
    let mut data = vec![0u8; 16];
    core.read_8(0x10_000, &mut data)?;
    info!("read back: {:?}", data);

    warn!("Done flashing: {}", elf);
    Ok(())
}

pub fn print_core_status(core: &mut Core) -> anyhow::Result<()> {
    let running = !core.core_halted().unwrap();
    let pc = core.read_core_reg(CoreRegisterAddress(15))?;

    info!("Core running: {}", running);
    info!("PC: {:#08x}", pc);

    Ok(())
}
