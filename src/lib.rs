
extern crate lv2rs;

mod gen;

use lv2rs::core::{self,*};
use std::ffi::CStr;
use gen::SignalGenerator;

pub struct SignalGeneratorPlugin {
    out: ports::AudioOutputPort,
    rel_freq: ports::ParameterInputPort,
    generator: gen::SignalGenerator,
}

impl Plugin for SignalGeneratorPlugin {

    fn instantiate(
        _descriptor: &Descriptor,
        rate: f64,
        _bundle_path: &CStr,
        _features: Option<&FeaturesList>,
    ) -> Option<Self> {
        
        Some(Self {
            rel_freq: ports::ParameterInputPort::new(),
            out: ports::AudioOutputPort::new(),
            generator: SignalGenerator::new(rate as f32, 300.),
        })
    }

    fn connect_port(&mut self, port: u32, data: *mut ()) {
        match port {
            0 => self.rel_freq.connect(data as *const f32),
            1 => self.out.connect(data as *mut f32),
            _ => (),
        }
    }    

    fn activate(&mut self) {
        self.generator.reset();
    }

    fn run(&mut self, n_samples: u32) {
        let rel_freq = *(unsafe { self.rel_freq.get() }.unwrap());

        // Convert rel_frequency to frequency
        
        const MIN_FREQ: f32 = 20.;
        let freq = (self.generator.rate()/2. - MIN_FREQ) * rel_freq + MIN_FREQ;
        self.generator.set_frequency(freq);

        let output =  unsafe { self.out.as_slice(n_samples) }
            .unwrap();
        self.generator.generate(output)
    }
}



lv2_main!(core, SignalGeneratorPlugin, b"urn:upachler:signal-generator\0");
