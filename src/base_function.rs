

pub trait BaseFunction {
     fn new(index: usize, elements_count: usize, range: std::ops::Range<f64>) -> Self;

     fn regular(&self, x: f64) -> f64;
     fn derivative(&self, x: f64) -> f64;
}

pub struct SimpleBaseFunction {
    index: usize,
    elements_count: usize,
    range: std::ops::Range<f64>,
}

impl BaseFunction for SimpleBaseFunction {

    fn new(index: usize, elements_count: usize, range: std::ops::Range<f64>) -> Self {
        Self { index, elements_count, range }
    }
        
    fn regular(&self, x: f64) -> f64 {
        let elem = (self.range.end - self.range.start) / (self.elements_count as f64);

        let mid = (self.index as f64) * elem;
        let low = mid - elem;
        let high = mid + elem;


        if low <= x && x <= mid {
            return (x - low) / elem;
        } else if mid < x && x <= high {
            return (high - x) / elem;
        }
               
        return 0.0f64;
    }

    fn derivative(&self, x: f64) -> f64 {
        let elem = (self.range.end - self.range.start) / (self.elements_count as f64);
        
        let mid = (self.index as f64) * elem;
        let low = mid - elem;
        let high = mid + elem;

        if low <= x && x <= mid {
            return elem;
        } else if mid < x && x <= high {
            return -elem;
        }

        return 0f64;
    }

}

