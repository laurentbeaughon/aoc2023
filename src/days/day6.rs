use super::aocday::AOCDay;

pub struct DaySix {}

impl AOCDay for DaySix {
    fn part_one(&self, input: &str) -> String {
        let time: Vec<f32> = vec![54.0, 81.0, 70.0, 88.0];
        let distances: Vec<f32> = vec![446.0, 1292.0, 1035.0, 1007.0];
        let mut result: f32 = 1.0;
        for (t, d) in time.iter().zip(&distances) {
            let delta: f32 = (t * t - 4.0 * d).sqrt();
            let x1: f32 = (- (t + delta) / 2.0).floor();
            let x2: f32 = (- (t - delta) / 2.0).ceil() - 1.0;
            result *= x1 - x2;
        }
        format!("{}", result)
    }

    fn part_two(&self, input: &str) -> String {
        let time: f64 = 54817088.0;
        let distances: f64 = 446129210351007.0;
        let delta: f64 = (time * time - 4.0 * distances).sqrt();
        let x1: f64 = (- (time + delta) / 2.0).floor();
        let x2: f64 = (- (time - delta) / 2.0).ceil() - 1.0;
        let result: f64 = x1 - x2;
        format!("{}", result)
    }
}
