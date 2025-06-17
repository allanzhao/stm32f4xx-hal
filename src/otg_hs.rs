//! USB OTG high-speed peripheral
//!
//! Requires the `usb_hs` feature.
//! Only one of the `usb_fs`/`usb_hs` features can be selected at the same time.

use crate::pac;

use crate::gpio::alt::otg_hs as alt;
use crate::rcc::{Clocks, Enable, Reset};
use fugit::HertzU32 as Hertz;

pub use synopsys_usb_otg::UsbBus;
use synopsys_usb_otg::UsbPeripheral;

pub struct USB {
    pub usb_global: pac::OTG_HS_GLOBAL,
    pub usb_device: pac::OTG_HS_DEVICE,
    pub usb_pwrclk: pac::OTG_HS_PWRCLK,
    pub pin_dm: alt::Dm,
    pub pin_dp: alt::Dp,
    pub hclk: Hertz,
}

impl USB {
    pub fn new(
        periphs: (pac::OTG_HS_GLOBAL, pac::OTG_HS_DEVICE, pac::OTG_HS_PWRCLK),
        pins: (impl Into<alt::Dm>, impl Into<alt::Dp>),
        clocks: &Clocks,
    ) -> Self {
        Self {
            usb_global: periphs.0,
            usb_device: periphs.1,
            usb_pwrclk: periphs.2,
            pin_dm: pins.0.into(),
            pin_dp: pins.1.into(),
            hclk: clocks.hclk(),
        }
    }
}

unsafe impl Sync for USB {}

unsafe impl UsbPeripheral for USB {
    const REGISTERS: *const () = pac::OTG_HS_GLOBAL::ptr() as *const ();

    const HIGH_SPEED: bool = true;
    const FIFO_DEPTH_WORDS: usize = 1024;

    #[cfg(any(feature = "gpio-f417", feature = "gpio-f427"))]
    const ENDPOINT_COUNT: usize = 6;
    #[cfg(any(feature = "gpio-f446", feature = "gpio-f469"))]
    const ENDPOINT_COUNT: usize = 9;

    fn enable() {
        cortex_m::interrupt::free(|_| {
            unsafe {
                // Enable USB peripheral
                pac::OTG_HS_GLOBAL::enable_unchecked();
                // Reset USB peripheral
                pac::OTG_HS_GLOBAL::reset_unchecked();
            }
        });
    }

    fn ahb_frequency_hz(&self) -> u32 {
        self.hclk.raw()
    }
}

pub type UsbBusType = UsbBus<USB>;

pub struct UsbUlpi {
    pub usb_global: pac::OTG_HS_GLOBAL,
    pub usb_device: pac::OTG_HS_DEVICE,
    pub usb_pwrclk: pac::OTG_HS_PWRCLK,
    pub ulpi_clk: alt::UlpiCk,
    pub ulpi_dir: alt::UlpiDir,
    pub ulpi_nxt: alt::UlpiNxt,
    pub ulpi_stp: alt::UlpiStp,
    pub ulpi_d0: alt::UlpiD0,
    pub ulpi_d1: alt::UlpiD1,
    pub ulpi_d2: alt::UlpiD2,
    pub ulpi_d3: alt::UlpiD3,
    pub ulpi_d4: alt::UlpiD4,
    pub ulpi_d5: alt::UlpiD5,
    pub ulpi_d6: alt::UlpiD6,
    pub ulpi_d7: alt::UlpiD7,
    pub hclk: Hertz,
}

impl UsbUlpi {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        usb_global: pac::OTG_HS_GLOBAL,
        usb_device: pac::OTG_HS_DEVICE,
        usb_pwrclk: pac::OTG_HS_PWRCLK,
        ulpi_clk: impl Into<alt::UlpiCk>,
        ulpi_dir: impl Into<alt::UlpiDir>,
        ulpi_nxt: impl Into<alt::UlpiNxt>,
        ulpi_stp: impl Into<alt::UlpiStp>,
        ulpi_d0: impl Into<alt::UlpiD0>,
        ulpi_d1: impl Into<alt::UlpiD1>,
        ulpi_d2: impl Into<alt::UlpiD2>,
        ulpi_d3: impl Into<alt::UlpiD3>,
        ulpi_d4: impl Into<alt::UlpiD4>,
        ulpi_d5: impl Into<alt::UlpiD5>,
        ulpi_d6: impl Into<alt::UlpiD6>,
        ulpi_d7: impl Into<alt::UlpiD7>,
        clocks: &Clocks,
    ) -> Self {
        UsbUlpi {
            usb_global,
            usb_device,
            usb_pwrclk,
            ulpi_clk: ulpi_clk.into(),
            ulpi_dir: ulpi_dir.into(),
            ulpi_nxt: ulpi_nxt.into(),
            ulpi_stp: ulpi_stp.into(),
            ulpi_d0: ulpi_d0.into(),
            ulpi_d1: ulpi_d1.into(),
            ulpi_d2: ulpi_d2.into(),
            ulpi_d3: ulpi_d3.into(),
            ulpi_d4: ulpi_d4.into(),
            ulpi_d5: ulpi_d5.into(),
            ulpi_d6: ulpi_d6.into(),
            ulpi_d7: ulpi_d7.into(),
            hclk: clocks.hclk(),
        }
    }
}

unsafe impl Sync for UsbUlpi {}

unsafe impl UsbPeripheral for UsbUlpi {
    const REGISTERS: *const () = pac::OTG_HS_GLOBAL::ptr() as *const ();

    const HIGH_SPEED: bool = true;
    const FIFO_DEPTH_WORDS: usize = 1024; // RM0090 page 1429
    const ENDPOINT_COUNT: usize = 6; // RM0090 page 1386

    fn enable() {
        let rcc = unsafe { &*pac::RCC::ptr() };

        cortex_m::interrupt::free(|_| {
            // Enable ULPI clock
            rcc.ahb1enr().modify(|_, w| w.otghsulpien().enabled());
        });

        unsafe {
            // Enable USB peripheral
            pac::OTG_HS_GLOBAL::enable_unchecked();
            // Reset USB peripheral
            pac::OTG_HS_GLOBAL::reset_unchecked();
        }
    }

    fn ahb_frequency_hz(&self) -> u32 {
        self.hclk.raw()
    }

    fn phy_type(&self) -> synopsys_usb_otg::PhyType {
        synopsys_usb_otg::PhyType::ExternalHighSpeed
    }
}

pub type UsbUlpiBusType = UsbBus<UsbUlpi>;
