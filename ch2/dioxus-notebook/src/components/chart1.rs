use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use ode_solvers::dopri5::*;
use ode_solvers::*;

use charming::{
    component::{
        Axis, Brush, BrushType, DataZoom, DataZoomType, Feature, Toolbox, ToolboxDataZoom, Legend, Title,
    },
    element::{
        AxisPointer, AxisType, Label, LabelPosition, MarkLine,
        MarkLineData, MarkLineVariant, MarkPoint, MarkPointData, NameLocation, Symbol, Tooltip,
        Trigger,
    },
    series::{Line, Scatter},
    Chart, ChartResize, HtmlRenderer, WasmRenderer,
};
#[derive(Copy, Clone, Debug)]
struct Model {
    beta_m: f64,
    gamma_m: f64,
    beta_p: f64,
    gamma_p: f64,
}

type State = Vector2<f64>;
type Time = f64;

impl ode_solvers::System<f64, State> for Model {
    // x(t) =  (self.beta / self.gamma) * (1.0- (-self.gamma*_t).exp());
    fn system(&self, _t: Time, x: &State, dx: &mut State) {
        dx[0] = self.beta_m - self.gamma_m * x[0];
	dx[1] = self.beta_p * x[0] - self.gamma_p * x[1];
    }
}

#[component]
pub fn LineChart1() -> Element {
    let chart = use_signal(|| {
        let system = Model {
            beta_m: 1.0,
            gamma_m: 1.0,
	    beta_p: 1.0,
	    gamma_p: 0.1,
        };
        let x = State::new(0.0,0.0);
        let t = 0.0;
        let t_end = 50.0;
        let dt = 0.0015; // Step size to get ~4000 points

        let mut stepper = Dopri5::new(system, t, t_end, dt, x, 1e-6_f64, 1e-6_f64);
        stepper.integrate().expect("failed integration");
        //let t0 = 1.0 / system.gamma;
        //let x0 = system.beta / system.gamma * (1.0 - (-1.0_f64).exp());
        let series_m = stepper
            .x_out()
            .iter()
            .zip(stepper.y_out().iter())
            .map(|(x, y)| vec![*x, y[0]])
            .collect();
	let series_p = stepper
            .x_out()
            .iter()
            .zip(stepper.y_out().iter())
            .map(|(x, y)| vec![*x, y[1]])
            .collect();
        Chart::new()
	    //.title(Title::new().text("Unregulated Expression").item_gap(25))
	    .legend(Legend::new())
            .x_axis(
                Axis::new()
                    .name("Time")
                    .name_gap(25)
                    .name_location(NameLocation::Middle)
                    .min(t)
                    .max(t_end)
                    .axis_pointer(AxisPointer::new().z(100)),
            )
            //.tooltip(Tooltip::new().trigger(Trigger::Axis))
            .y_axis(
                Axis::new()
                    .name("Concentration")
                    .min(0)
                    .max(10)
                    .name_gap(25)
                    .name_location(NameLocation::Middle),
            )
            .series(
                Line::new()
                    .data(series_m) // blue
                    .show_symbol(false)
                    .name("Concentration m")
	    )
	    .series(
                Line::new()
                    .data(series_p)
                    .show_symbol(false)
                    .name("Concentration p")
	    )
            .data_zoom(DataZoom::new().type_(DataZoomType::Inside).realtime(true))
    });
    let renderer = use_signal(|| WasmRenderer::new(600, 400));
    let mut echarts = use_signal(|| None);
    use_effect(move || {
        *echarts.write() = Some(
            renderer
                .read_unchecked()
                .render("chart", &chart.read())
                .unwrap(),
        )
    });
    rsx! (
        document::Script { src: asset!("/assets/echarts/echarts.min.js") }
        div { style: "width: 100%; text-align: center;",
            h1 { style: "color:black", "Unregulated Expression" }
            div { id: "chart", style: "display: inline-block;" }
        }
    )
}
