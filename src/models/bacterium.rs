use crate::utils::utils::Utils;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Bacterium {
    id: i32,
    size: f64,
    growth_rate: f64,
    div_par: f64,
    k: f64,
    next_t: f64,
    rv: f64,
    num_steps: i32,
    total_steps: i32,
}

impl Bacterium {
    pub fn new(
        id: i32,
        base_size: f64,
        growth_rate: f64,
        div_par: f64,
        k: f64,
        total_steps: i32,
    ) -> Self {
        Bacterium {
            id: id,
            size: base_size,
            growth_rate: growth_rate,
            div_par: div_par,
            k: k,
            next_t: 100.0 / growth_rate,
            rv: Utils::get_random(),
            num_steps: 0,
            total_steps: total_steps,
        }
    }

    pub fn change(&mut self, size:f64) -> () {
        self.size = size;
        self.rv = Utils::get_random();
    }

    pub fn division(&mut self, size: f64, division_par: f64, growth_rate: f64, k: f64){
        self.growth_rate = growth_rate;
        self.k = k;
        self.size = size * self.div_par;
        self.div_par = division_par;
        self.rv = Utils::get_random();
        self.num_steps = 0;
    }

    pub fn next_t(&mut self, base_size: f64, env_gr: f64, env_k: f64, env_lambda: f64) -> () {
        self.next_t = Utils::calculate_next_t(base_size, env_gr, env_k, env_lambda, self);
    }

    pub fn add_steps(&mut self) -> () {
        self.num_steps += 1;
    }

    //Getter And Setter

    pub fn get_id(&self) -> i32 {
        return self.id;
    }

    pub fn get_growth_rate(&self) -> f64 {
        return self.growth_rate;
    }

    pub fn get_size(&self) -> f64 {
        return self.size;
    }

    pub fn get_k(&self) -> f64 {
        return self.k;
    }

    pub fn get_random(&self) -> f64 {
        return self.rv;
    }

    pub fn get_next_time(&self) -> f64 {
        return self.next_t;
    }

    pub fn set_next_time(&mut self, next_t: f64) -> () {
        self.next_t = next_t;
    }

    pub fn get_step(&self) -> i32 {
        return self.num_steps;
    }

    pub fn get_total_steps(&self) -> i32 {
        return self.total_steps;
    }
}
