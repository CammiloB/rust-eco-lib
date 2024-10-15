use crate::models::bacterium::Bacterium;
use crate::models::result_json::ResultJson;
use crate::utils::utils::Utils;

pub struct Simulator{

    pub n_cells: i32,
    pub growth_rate: f64,
    pub base_size: f64,
    pub total_steps: i32,

    pub cv2_div: f64,
    pub cv2_gr: f64,
    pub lambda: f64,
    pub sampling_time: f64,
    pub k: f64,
    pub bacteriums: Vec<Bacterium>,
    pub sizes: Vec<ResultJson>,

}

impl Simulator{
    pub fn new(n_cells: i32, growth_rate: f64, base_size: f64, total_steps: i32, cv2_div: f64, cv2_gr: f64) -> Self {
        Simulator {
            n_cells: n_cells,
            growth_rate: growth_rate,
            base_size: base_size,
            total_steps: total_steps,
            cv2_div: cv2_div,
            cv2_gr: cv2_gr,
            lambda: 1.0,
            sampling_time: 0.0,
            k: total_steps as f64 * growth_rate / base_size,
            bacteriums: Vec::new(),
            sizes: Vec::new(),
        }
    }

    pub fn initialize_cells(&mut self) -> () {
        for i in 0..self.n_cells {
           let growth_rate = Utils::new_growth_rate(self.cv2_gr);
           let div_par = Utils::new_div_par(self.cv2_div);
           let mut bacterium = Bacterium::new(i, self.base_size, growth_rate, div_par, growth_rate, self.total_steps);
           bacterium.next_t(self.base_size, self.growth_rate, self.k, self.lambda);
           self.bacteriums.push(bacterium);
        }
    }

    pub fn size_dynamics(&mut self, max_time: f64, sample_time: f64) {
        self.initialize_cells();
        self.sampling_time = sample_time;
        let mut time: f64 = 0.0;
        let mut counter: f64 = 0.0;
        let target_time: f64 = 0.0;

        while time < max_time {
            self.simulate(self.sampling_time);
            time += self.sampling_time;

            for bacterium in self.bacteriums.iter_mut(){
                if bacterium.get_id() == 0{
                    self.sizes.push(ResultJson::new(time, Utils::truncate(bacterium.get_size(), 4)));
                }
            }
            

            counter += self.sampling_time;
            if counter >= target_time {
                println!("{} %", (100.0 * time / max_time) as i32 );
            }
        }
    }

    pub fn simulate(&mut self, max_time: f64) -> () {
        for bacterium in self.bacteriums.iter_mut() {
            let mut time = 0.0;
            while time < max_time {
                let tt = bacterium.get_next_time();
                if(time + tt) <= max_time {
                        bacterium.add_steps();
                        let size = bacterium.get_size() * f64::exp(self.growth_rate * bacterium.get_growth_rate() * tt);
                        if bacterium.get_step() >= bacterium.get_total_steps(){ 
                            let division_par = Utils::new_div_par(self.cv2_div);
                            let new_growth_rate = Utils::new_growth_rate(self.cv2_gr);
                            bacterium.division(size, division_par, new_growth_rate, new_growth_rate);
                        } else {
                            println!("Size: {}", size);
                            bacterium.change(size);
                        }

                        bacterium.next_t(bacterium.get_size(), self.growth_rate, self.k, self.lambda);
                } else {
                    let size = bacterium.get_size() * f64::exp(self.growth_rate * bacterium.get_growth_rate() * (max_time - time));
                    bacterium.change(size);
                    bacterium.set_next_time(bacterium.get_next_time() - (max_time - time));
                }

                time += tt;
            }
        }
    }
}