pub struct EBUR128Compressor {
    rate: f32,
    offset_t: f32,
    new_frequency: f32,
    current_frequency: f32,
}


impl EBUR128Compressor {
    pub fn new(rate: f32, frequency: f32) -> EBUR128Compressor {
        EBUR128Compressor {
            rate,
            new_frequency: frequency,
            current_frequency: frequency,
            offset_t: 0.,
        }
    }

    pub fn rate(&self) -> f32 {
        self.rate
    }
    pub fn reset(&mut self) {
        self.offset_t = 0.0;
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.new_frequency = frequency;
    }

    pub fn generate(&mut self, output: &mut [f32]) {

        if self.new_frequency == self.current_frequency {
            // no frequency change scheduled, render frames normally and
            // record new offset_t for next batch
            self.offset_t = EBUR128Compressor::generate_freq(self.rate, output, self.current_frequency, self.offset_t);
        } else {

            // how many samples until we reach the end of the period,
            // where we hit the zero line?
            let period_t: f32 = 1.0 / self.current_frequency;
            let n = (f32::max(0., period_t - self.offset_t) * self.rate) as usize;

            // check if we'll actually reach the end of the period with the current_frequency
            if n > output.len() {
                // we won't reach the end this time, do not change frequency yet, but continue
                // rendering samples normally instead.
                // frequency change will be attempted again in next rendering batch
                self.offset_t = EBUR128Compressor::generate_freq(self.rate, output, self.current_frequency, self.offset_t);
            } else {
                // we'll read the end of current_frequency's domain, issue frequency change
                // by splitting the buffer in two segments:
                // * left: rendered with current_frequency
                // * right: rendered with new_frequency
                let (left, right) = output.split_at_mut(n);

                // calculate left buffer first, but ignore reported offset time - it'll be close
                // to zero
                EBUR128Compressor::generate_freq(self.rate, left, self.current_frequency, self.offset_t);

                // calculate right buffer side with new frequency after that, starting at offset 0
                self.offset_t = EBUR128Compressor::generate_freq(self.rate, right, self.new_frequency, 0f32);

                self.current_frequency = self.new_frequency;
            }
            
        }

    }

    fn generate_freq(rate: f32, output: &mut [f32], frequency: f32, offset_t: f32) -> f32{
        let period_t: f32 = 1.0 / frequency;

        // make offset_t mutable
        let mut offset_t = offset_t;

        let mut i: i32 = 0;
        let mut t = offset_t;

        output.iter_mut().for_each(|sample| {
            let x = frequency * 2.0 * std::f32::consts::PI * t;
            *sample = x.sin();
            
            i += 1;
            if t >= period_t {
                offset_t -= period_t;
            }
            t = offset_t + (i as f32 / rate);
        });

        t
    }
}

#[cfg(test)]
mod tests {
    use crate::EBUR128Compressor;
    #[test]
    fn test_signal() {
        let mut gen = EBUR128Compressor::new(8.0, 1.);

        let mut buf: Vec<f32> = Vec::new();
        buf.resize(gen.rate as usize + 1, 0.0);
        // you might have noticed that the buffer we allocated
        // has rate+1 elements - this is because we
        // run the full sine from 0..2*PI - however, 2*PI is
        // already part of the next cycle. This is so 
        // have the first element of the next cycle 
        // in the last buffer element

        gen.generate(buf.as_mut_slice());
        println!("buf: {:?}", buf);

        // we check the minima, maxima and zero crossings of the
        // generated sine
        assert!(0.0 - buf[0] <= std::f32::EPSILON);
        assert!(1.0 - buf[(buf.len()-1)/4] <= std::f32::EPSILON);
        assert!(0.0 - buf[(buf.len()-1)/2] <= std::f32::EPSILON);
        assert!(-1.0 - buf[(buf.len()-1)/4*3] <= std::f32::EPSILON);
        assert!(0.0 - buf[(buf.len()-1)] <= std::f32::EPSILON);
    }

    #[test]
    fn test_continuation_simple() {
        // to ensure that we 

        let mut gen = EBUR128Compressor::new(8., 1.);

        let mut buf = Vec::new();
        buf.resize(4, 0.);

        gen.generate(buf.as_mut());
        println!("buf: {:?}", buf);
        assert!(eq_epsilon(0.0, buf[0]));
        assert!(eq_epsilon(1.0, buf[2]));

        gen.generate(buf.as_mut());
        println!("buf: {:?}", buf);
        assert!(eq_epsilon(0.0, buf[0]));
        assert!(eq_epsilon(-1.0, buf[2]));
    }

    #[test]
    fn test_continuation_odd() {
        let mut gen = EBUR128Compressor::new(16., 1.);
        gen.offset_t = 0.25;

        let mut buf = Vec::new();
        buf.resize(16, 0.);
        
        // run full 16 samples, this should generate the whole cycle.
        // we expect offset_t to be back at zero
        gen.generate(buf.as_mut());
        println!("buf: {:?}", buf);
        println!("gen.offset_t: {:?}", gen.offset_t);
        assert!(eq_epsilon(0.25, gen.offset_t));

        // gen.reset() sets offset_t back to 0
        gen.reset();

        // run 4 samples, generating only a quarter.
        // we expect offset_t to be at a quarter of the
        // time, 0.25s
        // then we run another quarter, offset_t should
        // then reach 0.5
        buf.resize(4, 0.);
        // first quarter
        gen.generate(buf.as_mut());
        println!("buf: {:?}", buf);
        println!("gen.offset_t: {:?}", gen.offset_t);
        assert!(eq_epsilon(0.25, gen.offset_t));
        assert!(eq_epsilon(0., buf[0]));

        // second quarter
        gen.generate(buf.as_mut());
        println!("buf: {:?}", buf);
        println!("gen.offset_t: {:?}", gen.offset_t);
        assert!(eq_epsilon(0.5, gen.offset_t));
        assert!(eq_epsilon(1., buf[0]));
    }

    #[test]
    fn change_frequency() {
        let mut gen = EBUR128Compressor::new(8., 2.);

        let mut buf = Vec::new();
        buf.resize(2, 0.);

        gen.generate(buf.as_mut());
        println!("buf: {:?}", buf);
        println!("gen.offset_t: {:?}", gen.offset_t);
        assert!(eq_epsilon(0.25, gen.offset_t));
        assert!(eq_epsilon(0., buf[0]));
        assert!(eq_epsilon(1., buf[1]));

        gen.set_frequency(1.);
        buf.resize(6, 0.);

        gen.generate(buf.as_mut());
        println!("buf: {:?}", buf);
        println!("gen.offset_t: {:?}", gen.offset_t);
        // samples still @2Hz
        assert!(eq_epsilon( 0., buf[0]));
        assert!(eq_epsilon(-1., buf[1]));
        // samples after effective frequency change @1Hz
        assert!(eq_epsilon( 0., buf[2]));
        assert!(eq_epsilon( 1., buf[4]));
        // stopped sampling at offset 0.5s (half the period of 1Hz)
        assert!(eq_epsilon(0.5, gen.offset_t));

    }

    fn eq_epsilon(a: f32, b: f32) -> bool {
        (a - b).abs() <= std::f32::EPSILON
    }
}
