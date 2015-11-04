//! Reservoir sampling: collect a random sample of a known maximum
//! size from an iterator of unknown length.
//!
//! Implements Jeffrey Vitter's Algorithm R (see
//! https://en.wikipedia.org/wiki/Reservoir_sampling)

extern crate rand;

use rand::Rng;

/// Return a random sample of a known maximum size from an iterator of
/// unknown length.
///
/// # Examples
/// 
/// ```
/// # extern crate rand;
/// # extern crate reservoir;
/// # use rand::thread_rng;
/// # use reservoir::sample;
/// # fn main() {
/// let iter = 0..10;
/// 
/// let samples = sample(&mut thread_rng(), 4, iter);
/// 
/// assert_eq!(4, samples.len());
/// assert!(samples.iter().all(|e| *e >= 0 && *e < 10));
/// # }
/// ```
/// 
/// If the sampled iterator contains fewer items than the sample_size,
/// all items are returned.
///
/// ```
/// # extern crate rand;
/// # extern crate reservoir;
/// # use rand::thread_rng;
/// # use reservoir::sample;
/// # fn main() {
/// let iter = 0..10;
/// 
/// let samples : Vec<i32> = sample(&mut thread_rng(), 20, iter);
/// let expected_samples : Vec<i32> = (0..10).collect();
/// 
/// assert_eq!(expected_samples, samples);
/// # }
/// ```

pub fn sample<I, RNG>(rng : &mut RNG, sample_size : usize, iter : I) -> Vec<I::Item>
    where I : Iterator,
          RNG : Rng
{
    let mut samples = Vec::<I::Item>::with_capacity(sample_size);
    sample_into(&mut samples, rng, sample_size, iter);
    samples
}

/// Collect a random sample of a known maximum size from an iterator
/// of unknown length into an existing Vec.
/// 
/// # Examples
///
/// For example:
///
/// ```
/// # extern crate rand;
/// # extern crate reservoir;
/// # use rand::thread_rng;
/// # use reservoir::sample_into;
/// # fn main() {
/// let iter = 0..10;
/// let mut samples : Vec<i32> = Vec::new();
/// 
/// sample_into(&mut samples, &mut thread_rng(), 4, iter);
/// 
/// assert_eq!(4, samples.len());
/// assert!(samples.iter().all(|e| *e >= 0 && *e < 10));
/// # }
/// ```
///
/// Preserves any elements already in the vector:
///
/// ```
/// # extern crate rand;
/// # extern crate reservoir;
/// # use rand::thread_rng;
/// # use reservoir::sample_into;
/// # fn main() {
/// let iter = 0..10;
/// let mut samples : Vec<i32> = vec![99,100];
/// 
/// sample_into(&mut samples, &mut thread_rng(), 4, iter);
/// 
/// assert_eq!(6, samples.len());
/// assert_eq!(99, samples[0]);
/// assert_eq!(100, samples[1]);
/// assert!(samples[2..].iter().all(|e| *e >= 0 && *e < 10));
/// # }
/// ```

pub fn sample_into<I, RNG>(samples : &mut Vec<I::Item>, rng : &mut RNG, sample_size : usize, iter : I)
    where I : Iterator,
          RNG : Rng
{
    let original_length = samples.len();
    let mut count : usize = 0;
    
    for element in iter {
        count += 1;
        
        if count <= sample_size {
            samples.push(element);
        } else {
            let index = rng.gen_range(0, count);
            if index < sample_size {
                samples[original_length+index] = element;
            }
        }
    }
}

