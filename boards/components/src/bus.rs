use capsules::bus::SpiBus;
use capsules::virtual_spi::VirtualSpiMasterDevice;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use kernel::component::Component;
use kernel::hil::spi;
use kernel::static_init_half;

// Setup static space for the objects.
#[macro_export]
macro_rules! spi_bus_component_helper {
    ($S:ty, $select:expr, $spi_mux: expr) => {{
        use capsules::bus::SpiBus;
        use core::mem::{size_of, MaybeUninit};
        let bus_spi: &'static capsules::virtual_spi::VirtualSpiMasterDevice<'static, $S> =
            components::spi::SpiComponent::new($spi_mux, $select)
                .finalize(components::spi_component_helper!($S));
        static mut ADDRESS_BUFFER: [u8; size_of::<usize>()] = [0; size_of::<usize>()];
        static mut bus: MaybeUninit<SpiBus<'static>> = MaybeUninit::uninit();
        (&bus_spi, &mut bus, &mut ADDRESS_BUFFER)
    };};
}

pub struct SpiBusComponent<S: 'static + spi::SpiMaster> {
    _select: PhantomData<S>,
}

impl<S: 'static + spi::SpiMaster> SpiBusComponent<S> {
    pub fn new() -> SpiBusComponent<S> {
        SpiBusComponent {
            _select: PhantomData,
        }
    }
}

impl<S: 'static + spi::SpiMaster> Component for SpiBusComponent<S> {
    type StaticInput = (
        &'static VirtualSpiMasterDevice<'static, S>,
        &'static mut MaybeUninit<SpiBus<'static>>,
        &'static mut [u8],
    );
    type Output = &'static SpiBus<'static>;

    unsafe fn finalize(self, static_buffer: Self::StaticInput) -> Self::Output {
        let bus = static_init_half!(
            static_buffer.1,
            SpiBus<'static>,
            SpiBus::new(static_buffer.0, static_buffer.2)
        );
        static_buffer.0.set_client(bus);

        bus
    }
}
