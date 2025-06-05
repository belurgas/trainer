use std::vec;

#[derive(Debug, Clone)]
struct TrainingSample {
    x: Vec<f64>, // признаки (вектор)
    y: f64, // метка
}

#[derive(Debug)]
struct TrainingSet {
    samples: Vec<TrainingSample>, // список всех обущающих примеров
}

// Преобразование в признаковое пространство
impl TrainingSet {
    fn feature_matrix(&self) -> Vec<Vec<f64>> {
        let safe: Vec<Vec<f64>> = self.samples.iter().map(|sample| sample.x.clone()).collect();
        safe
    }

    fn label(&self) -> Vec<f64> {
        self.samples.iter().map(|sample| sample.y).collect()
    }
}

fn main() {
    let sample1 = TrainingSample {
        x: vec![1., 2., 3.],
        y: 1.,
    };

    let sample2 = TrainingSample {
        x: vec![4., 5., 6.],
        y: 0.,
    };

    let dataset = TrainingSet {
        samples: vec![sample1, sample2],
    };

    let features = dataset.feature_matrix();
    let labels = dataset.label();

    println!("Feature matrix: {:?}", features);
    println!("Labels: {:?}", labels);
}