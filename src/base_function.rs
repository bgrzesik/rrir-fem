

pub trait BaseFunction {
     fn new(index: usize, elements_count: usize, range: std::ops::Range<f64>) -> Self;

     fn regular(&self, x: f64) -> f64;
     fn derivative(&self, x: f64) -> f64;

     fn non_zero_range(&self) -> std::ops::Range<f64>;
}

pub struct SimpleBaseFunction {
    index: usize,
    elements_count: usize,
    range: std::ops::Range<f64>,
}

impl SimpleBaseFunction {

    fn get_elem_size(&self) -> f64 {
        (self.range.end - self.range.start) / (self.elements_count as f64)
    }
    
    fn get_points(&self) -> (f64, f64, f64) {
        let elem = self.get_elem_size();

        let mid = (self.index as f64) * elem;
        let low = mid - elem;
        let high = mid + elem;

        return (low, mid, high);
    }
}

impl BaseFunction for SimpleBaseFunction {

    fn new(index: usize, elements_count: usize, range: std::ops::Range<f64>) -> Self {
        Self { index, elements_count, range }
    }
        
    fn regular(&self, x: f64) -> f64 {
        let elem = self.get_elem_size();
        let (low, mid, high) = self.get_points();

        if low <= x && x <= mid {
            return (x - low) / elem;
        } else if mid < x && x <= high {
            return (high - x) / elem;
        }
               
        return 0.0f64;
    }

    fn derivative(&self, x: f64) -> f64 {
        let elem = self.get_elem_size();
        let (low, mid, high) = self.get_points();

        if low <= x && x <= mid {
            return 1f64 / elem;
        } else if mid < x && x <= high {
            return 1f64 / -elem;
        }

        return 0f64;
    }

    fn non_zero_range(&self) -> std::ops::Range<f64> {
        let (low, _, high) = self.get_points();
        return low..high;
    }

}

