use rand::{thread_rng, Rng};
use rand_distr::{num_traits::Pow, Beta, Distribution, Gamma};

use crate::models::bacterium::Bacterium;

pub struct Utils;

impl Utils{

    pub fn truncate(num: f64, ciphers: i32) -> f64{
        let pos: f64 = 10.0.pow(ciphers);
        return (pos * num).trunc() / pos;
    }

    pub fn new_growth_rate( cv2: f64) -> f64{
        if cv2 == 0.0 { return 1.0 };

        return Utils::rand_gamma(1.0/cv2, cv2);
    }

    pub fn new_div_par(cv2: f64) -> f64 {
        if cv2 == 0.0 {return 0.5}

        let beta = 0.5 * ((1.0 / cv2) - 1.0);
        return Utils::rand_beta(beta, beta);
    }

    pub fn calculate_next_t(initial_size: f64, env_gr: f64, env_k: f64, env_lambda: f64, bacterium: &mut Bacterium) -> f64 {
        let mu = env_gr * bacterium.get_growth_rate();
        let k = env_k * bacterium.get_k();

        return (1.0 / (env_lambda * mu)) * (1.0-((env_lambda * mu) / (k * initial_size.powf(env_lambda)).ln() * bacterium.get_random().ln()));
    }

    pub fn rand_gamma(shape: f64, scale: f64) -> f64{
        let gamma_dist = Gamma::new(shape, scale).expect("Invalid Parameters");
        let mut rng = thread_rng();
        return gamma_dist.sample(&mut rng);
    }

    pub fn rand_beta(alpha: f64, beta: f64) -> f64 {
        let beta_dist = Beta::new(alpha, beta).unwrap();
        let mut rng = thread_rng();
        return beta_dist.sample(&mut rng);
    }

    pub fn get_random() -> f64{
        let mut rng = thread_rng();
        return rng.gen_range(0.0..1.0);
    }

}