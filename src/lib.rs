
extern crate lv2rs;

mod cmp;

use lv2rs::core::{self,*};
use std::ffi::CStr;
use cmp::EBUR128Compressor;

pub struct EBUR128CompressorPlugin {
    inp: ports::AudioInputPort,
    out: ports::AudioOutputPort,
    threshold: ports::ParameterInputPort,
    generator: cmp::EBUR128Compressor,
}

impl Plugin for EBUR128CompressorPlugin {

    fn instantiate(
        _descriptor: &Descriptor,
        rate: f64,
        _bundle_path: &CStr,
        _features: Option<&FeaturesList>,
    ) -> Option<Self> {
        
        Some(EBUR128CompressorPlugin {
            threshold: ports::ParameterInputPort::new(),
            inp: ports::AudioInputPort::new(),
            out: ports::AudioOutputPort::new(),
            generator: EBUR128Compressor::new(rate as f32, 300.),
        })
    }

    fn connect_port(&mut self, port: u32, data: *mut ()) {
        match port {
            0 => self.threshold.connect(data as *const f32),
            1 => self.inp.connect(data as *mut f32),
            2 => self.out.connect(data as *mut f32),
            _ => (),
        }
    }    

    fn activate(&mut self) {
        self.generator.reset();
    }

    fn run(&mut self, n_samples: u32) {
        let rel_freq = *(unsafe { self.threshold.get() }.unwrap());

        // Convert rel_frequency to frequency
        
        const MIN_FREQ: f32 = 20.;
        let freq = (self.generator.rate()/2. - MIN_FREQ) * rel_freq + MIN_FREQ;
        self.generator.set_frequency(freq);

        let output =  unsafe { self.out.as_slice(n_samples) }
            .unwrap();
        self.generator.generate(output)
    }
}



lv2_main!(core, EBUR128CompressorPlugin, b"urn:upachler:ebur128-compressor\0");
