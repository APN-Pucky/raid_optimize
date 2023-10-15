use crate::wave::stat::Stat;
use crate::wave::Result;
use enum_map::EnumMap;

pub fn get_mean(sum: f64, n: u64) -> f64 {
    (sum) / n as f64
}

pub fn get_standard_deviation(sum: f64, sum_sq: f64, n: u64) -> f64 {
    ((sum_sq - sum * sum / n as f64) / n as f64).sqrt()
}

pub fn get_mean_and_standard_deviation(sum: f64, sum_sq: f64, n: u64) -> (f64, f64) {
    (get_mean(sum, n), get_standard_deviation(sum, sum_sq, n))
}

#[derive(Debug, Clone, Default)]
pub struct CombinedResult {
    pub iterations: u64,
    pub wins: u64,
    pub losses: u64,
    pub stalls: u64,
    pub statistics: Vec<CombinedStatistics>, // TODO strong type allies and emeies ?
}

//impl Default for CombinedResult {
//    fn default() -> Self {
//        Self {
//            iterations: 0,
//            wins: 0,
//            losses: 0,
//            stalls: 0,
//            statistics: Vec::new(),
//        }
//    }
//}
#[derive(Debug, Clone, Copy)]
pub struct CombinedStatistics {
    pub hm: EnumMap<Stat, f64>,
    pub hm_sq: EnumMap<Stat, f64>,
}

impl CombinedResult {
    pub fn new(results: &Vec<Result>) -> CombinedResult {
        let mut cr = CombinedResult {
            iterations: 0,
            wins: 0,
            losses: 0,
            stalls: 0,
            statistics: Vec::new(),
        };
        for r in results {
            cr.add_result(r);
        }
        cr
    }

    pub fn add_combined_result(result: &mut CombinedResult, added: &CombinedResult) {
        result.iterations += added.iterations;
        result.wins += added.wins;
        result.losses += added.losses;
        result.stalls += added.stalls;
        Self::add_combined_statistics(&mut result.statistics, &added.statistics);
    }

    pub fn add_result(self: &mut CombinedResult, added: &Result) {
        self.iterations += 1;
        if added.win {
            self.wins += 1;
        } else if added.loss {
            self.losses += 1;
        } else if added.stall {
            self.stalls += 1;
        }
        Self::add_statistics(&mut self.statistics, &added.statistics);
    }
    ///*

    pub fn add_combined_statistics(
        statistics: &mut Vec<CombinedStatistics>,
        added: &Vec<CombinedStatistics>,
    ) {
        if statistics.len() < added.len() {
            for _i in statistics.len()..added.len() {
                statistics.push(CombinedStatistics {
                    hm: EnumMap::default(),
                    hm_sq: EnumMap::default(),
                });
            }
        }
        for (s, a) in statistics.iter_mut().zip(added.iter()) {
            for (key, _value) in a.hm.iter() {
                s.hm[key] += a.hm[key];
                s.hm_sq[key] += a.hm_sq[key];
            }
        }
    }

    pub fn add_statistics(
        statistics: &mut Vec<CombinedStatistics>,
        added: &Vec<EnumMap<Stat, f32>>,
    ) {
        if statistics.len() < added.len() {
            for _i in statistics.len()..added.len() {
                statistics.push(CombinedStatistics {
                    hm: EnumMap::default(),
                    hm_sq: EnumMap::default(),
                });
            }
        }
        for (s, a) in statistics.iter_mut().zip(added.iter()) {
            for (key, value) in a {
                let v = *value as f64;
                s.hm[key] += v;
                s.hm_sq[key] += v * v;
            }
        }
    }

    pub fn to_string(&self, index: usize) -> String {
        let mut s = String::new();
        for (key, value) in self.statistics[index].hm.iter() {
            //let value = self.statistics[index].hm[key];
            s += &format!(
                "\t {}: {:.2} +- {:.2}\n",
                key,
                get_mean(*value, self.iterations),
                get_standard_deviation(*value, self.statistics[index].hm_sq[key], self.iterations)
            );
        }
        s
    }

    pub fn print_statistics(&self, index: usize) {
        for (key, value) in self.statistics[index].hm.iter() {
            //let value = self.statistics[index].hm[key];
            println!(
                "\t {}: {:.2} +- {:.2}",
                key,
                get_mean(*value, self.iterations),
                get_standard_deviation(*value, self.statistics[index].hm_sq[key], self.iterations)
            );
        }
    }

    pub fn get_mean(&self, index: usize, key: Stat) -> f64 {
        let hm = &self.statistics[index].hm;
        // hm has key? else return 0.0
        get_mean(hm[key], self.iterations)
    }

    pub fn get_std(&self, index: usize, key: Stat) -> f64 {
        let hm = &self.statistics[index].hm;
        get_standard_deviation(hm[key], self.statistics[index].hm_sq[key], self.iterations)
    }
    //*/
}
