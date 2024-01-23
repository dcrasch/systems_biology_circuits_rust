use ode_solvers::*;
use plotters::prelude::*;

type State = Vector1<f64>;
type Time = f64;

fn autorepressive(
    x: f64,
    t: f64,
    beta0: f64,
    gamma: f64,
    k: f64,
    n: f64,
    ks: f64,
    ns: f64,
    s: f64,
) -> f64 {
    beta0 * (s / ks).powf(ns) / (1.0 + (s / ks).powf(ns)) / (1.0 + (x / k).powf(n)) - gamma * x	
}

fn unregulated(
    t: f64,
    beta0: f64,
    gamma: f64,
    k: f64,
    n: f64,
    ks: f64,
    ns: f64,
    s: f64,
) -> f64 {
    beta0 / gamma * (1.0 - (-gamma * t).exp())
}

struct Cs {
    beta0: f64,
    gamma: f64,
    k: f64,
    n: f64,
    ks: f64,
    ns: f64,
    s: f64,
}

impl ode_solvers::System<f64, State> for Cs {
    fn system(&self, t: Time, y: &State, dy: &mut State) {
        dy[0] = autorepressive(
            y[0], t, self.beta0, self.gamma,
	    self.k, self.n,
	    self.ks, self.ns, self.s,
        );
    }
}

fn main() {
    let step_size = 0.05;
    let y0 = State::new(0.0);

    let system = Cs {
        beta0: 100.0,
        gamma: 1.0,
        k: 1.0,
        n: 1.0,
        ns: 10.0,
        ks: 0.1,
        s: 100.0,
    };
    // 0-10 200
    let mut stepper = Rk4::new(system, 0.0, y0, 10.0, step_size);
    let res = stepper.integrate();
    let (x_out, y_out) = stepper.results().get();
    
    let root = BitMapBackend::new("myplot.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    
    let mut chart = ChartBuilder::on(&root)
        .caption("Constant-input dynamics", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0f64..10.0f64, 0.0f64..100.0f64).unwrap();

    chart.configure_mesh().draw().unwrap();

    let points = x_out.iter().zip(y_out.iter())
	.map(|(x,y)|(*x,y[0]))
	.collect::<Vec<(f64,f64)>>();
    chart
        .draw_series(LineSeries::new(
            points,
            &BLUE,
        )).unwrap()
	.label("autorepressive")
	.legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    
    let points = (0..200).map(|t|t as f64 * 0.05).map(|t|(t,unregulated(t,100.0,1.0,1.0,1.0,10.0,0.1,100.0)))
	.collect::<Vec<(f64,f64)>>();
    chart
        .draw_series(LineSeries::new(
            points,
            &RED,
        )).unwrap()
	.label("unregulated")
	.legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw().unwrap();

    root.present().unwrap();
}
