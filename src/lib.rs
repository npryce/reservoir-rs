
extern crate rand;

use rand::Rng;

/// Return a random sample of a known maximum size from an iterator of unknown length.
///
/// Implements Jeffrey Vitter's Algorithm R (see https://en.wikipedia.org/wiki/Reservoir_sampling)

pub fn sample<I, RNG>(rng : &mut RNG, sample_size : usize, iter : I) -> Vec<I::Item>
    where I : Iterator,
          RNG : Rng
{
    let mut samples = Vec::<I::Item>::with_capacity(sample_size);
    
    let mut count : usize = 0;

    for element in iter {
        count += 1;
        
        if count <= sample_size {
            samples.push(element);
        } else {
            let index = rng.gen_range(0, count);
            if index < samples.len() {
                samples[index] = element;
            }
        }
    }

    samples
}

#[cfg(test)]
mod test {
    use rand::thread_rng;
    use super::sample;
    
    #[test]
    fn sampling() {
        let iter = 0..10;
        
        let samples = sample(&mut thread_rng(), 4, iter);
        
        assert_eq!(4, samples.len());
        assert!(samples.iter().all(|e| *e >= 0 && *e < 10));
    }
    
    #[test]
    fn sample_size_larger_than_iter() {
        let iter = 0..10;
        
        let samples : Vec<i32> = sample(&mut thread_rng(), 20, iter);
        let expected_samples : Vec<i32> = (0..10).collect();
        
        assert_eq!(expected_samples, samples);
    }
}
