//! Serial NOR configuration blocks and fields

use crate::flexspi;

/// `ipCmdSerialClkFreq` field for serial NOR-specific FCB
///
/// Chip specific value, not used by ROM.
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum SerialClockFrequency {
    /// No change, keep current serial clock unchanged
    NoChange = 0,
    MHz30,
    MHz50,
    MHz60,
    #[cfg(not(feature = "imxrt500"))]
    MHz75,
    MHz80,
    MHz100,
    #[cfg(any(feature = "imxrt1060", feature = "imxrt1064", feature = "imxrt500"))]
    MHz120,
    MHz133,
    #[cfg(any(feature = "imxrt1060", feature = "imxrt1064", feature = "imxrt500"))]
    MHz166,
}

/// A serial NOR configuration block
///
/// This is the memory that you'll need to properly place in memory in order to
/// boot your i.MX RT system. Consider keeping the symbol name, and specifying
/// a link section, so that you can more easily place the memory in your linker
/// script.
///
/// Unless otherwise specified, all unset fields are set to a bitpattern of zero.
///
/// ```no_run
/// use imxrt_boot_gen::serial_flash::nor;
/// # use imxrt_boot_gen::flexspi::{self, LookupTable};
///
/// # const FLEXSPI_CONFIGURATION_BLOCK: flexspi::ConfigurationBlock = flexspi::ConfigurationBlock::new(LookupTable::new());
/// #[no_mangle]
/// #[link_section = ".serial_nor_cb"]
/// static SERIAL_NOR_CONFIGURATION_BLOCK: nor::ConfigurationBlock =
///     nor::ConfigurationBlock::new(FLEXSPI_CONFIGURATION_BLOCK)
///         .page_size(256)
///         .sector_size(4096)
///         .ip_cmd_serial_clk_freq(nor::SerialClockFrequency::MHz30);
/// ```
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct ConfigurationBlock {
    mem_cfg: flexspi::ConfigurationBlock,
    page_size: u32,
    sector_size: u32,
    ip_cmd_serial_clk_freq: u32,
    _reserved: [u8; 52],
}

impl ConfigurationBlock {
    /// Create a new serial NOR configuration block based on the FlexSPI configuration
    /// block
    pub const fn new(mut mem_cfg: flexspi::ConfigurationBlock) -> Self {
        mem_cfg.device_type = 1;
        ConfigurationBlock {
            mem_cfg,
            page_size: 0,
            sector_size: 0,
            ip_cmd_serial_clk_freq: 0,
            _reserved: [0; 52],
        }
    }
    /// Set the serial NOR page size
    pub const fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = page_size;
        self
    }
    /// Set the serial NOR sector size
    pub const fn sector_size(mut self, sector_size: u32) -> Self {
        self.sector_size = sector_size;
        self
    }
    /// Set the serial clock frequency
    pub const fn ip_cmd_serial_clk_freq(
        mut self,
        serial_clock_frequency: SerialClockFrequency,
    ) -> Self {
        self.ip_cmd_serial_clk_freq = serial_clock_frequency as u32;
        self
    }
}

const _STATIC_ASSERT_SIZE: [u32; 1] =
    [0; (core::mem::size_of::<ConfigurationBlock>() == 512) as usize];

#[cfg(test)]
mod test {
    use super::{flexspi, ConfigurationBlock, SerialClockFrequency};
    use crate::flexspi::LookupTable;

    #[test]
    fn smoke() {
        const _CFG: ConfigurationBlock =
            ConfigurationBlock::new(flexspi::ConfigurationBlock::new(LookupTable::new()))
                .page_size(256)
                .sector_size(4095)
                .ip_cmd_serial_clk_freq(SerialClockFrequency::MHz30);
    }

    #[test]
    #[cfg(feature = "imxrt500")]
    fn serial_clk_freq() {
        assert_eq!(SerialClockFrequency::MHz80 as u8, 4);
        assert_eq!(SerialClockFrequency::MHz166 as u8, 8);
    }

    #[test]
    #[cfg(feature = "imxrt1010")]
    fn serial_clk_freq() {
        assert_eq!(SerialClockFrequency::MHz133 as u8, 7);
    }
}
