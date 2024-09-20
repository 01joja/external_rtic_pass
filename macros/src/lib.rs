extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

// generates an valid rtic application
#[proc_macro_attribute]
pub fn generate_rtic_app(_: TokenStream, _: TokenStream) -> TokenStream{
    let ts = quote! {
        #[rtic::app(device = lm3s6965, compiler_passes = [resources,hardware])]
        mod app {
            use cortex_m_semihosting::{debug, hprintln};

            #[shared]
            struct Shared {}

            #[local]
            struct Local {
            }
            
            #[init]
            fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {

                hprintln!("init").unwrap();

                (Shared {}, Local {}, init::Monotonics())
            }

            #[idle]
            fn idle(_: idle::Context) -> ! {

                hprintln!("idle").unwrap();


                debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator

                loop {
                    cortex_m::asm::nop();
                }
            }

        }
    };
    ts.into()
}

