use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use ode_solvers::dopri5::*;
use ode_solvers::*;

use charming::{
    component::{
        Axis, Brush, BrushType, DataZoom, DataZoomType, Feature, Toolbox, ToolboxDataZoom,
    },
    element::{
        formatter::FormatterFunction, AxisPointer, AxisType, Label, LabelPosition, MarkLine,
        MarkLineData, MarkLineVariant, MarkPoint, MarkPointData, NameLocation, Symbol, Tooltip,
        Trigger,
    },
    series::{Line, Scatter},
    Chart, ChartResize, HtmlRenderer, WasmRenderer,
};
#[derive(Copy, Clone, Debug)]
struct Model {
    beta: f64,
    gamma: f64,
}

type State = Vector1<f64>;
type Time = f64;

impl ode_solvers::System<f64, State> for Model {
    // x(t) =  (self.beta / self.gamma) * (1.0- (-self.gamma*_t).exp());
    fn system(&self, _t: Time, x: &State, dx: &mut State) {
        dx[0] = self.beta - self.gamma * x[0];
    }
}

#[component]
pub fn LineChart() -> Element {
    let chart = use_signal(|| {
        let system = Model {
            beta: 100.0,
            gamma: 1.0,
        };
        let x = State::new(0.0);
        let t = 0.0;
        let t_end = 6.0;
        let dt = 0.015; // Step size to get ~400 points

        let mut stepper = Dopri5::new(system, t, t_end, dt, x, 1e-6_f64, 1e-6_f64);
        stepper.integrate().expect("failed integration");
        let t0 = 1.0 / system.gamma;
        let x0 = system.beta / system.gamma * (1.0 - (-1.0_f64).exp());
        let series = stepper
            .x_out()
            .iter()
            .zip(stepper.y_out().iter())
            .map(|(x, y)| vec![*x, y[0]])
            .collect();
        Chart::new()
            .x_axis(
                Axis::new()
                    .name("Time")
                    .name_gap(25)
                    .name_location(NameLocation::Middle)
                    .min(0)
                    .max(6)
                    .axis_pointer(AxisPointer::new().z(100)),
            )
            //.tooltip(Tooltip::new().trigger(Trigger::Axis))
            .y_axis(
                Axis::new()
                    .name("x(t)")
                    .min(-10)
                    .max(100)
                    .name_gap(25)
                    .name_location(NameLocation::Middle),
            )
            .series(
                Line::new()
                    .data(series)
                    .show_symbol(false)
                    .name("x(t)")
                    .mark_line(
                        MarkLine::new()
                            .symbol(vec![Symbol::None, Symbol::None])
                            .data(vec![MarkLineVariant::Simple(
                                MarkLineData::new().x_axis(t0),
                            )]),
                    ),
            )
            .series(
                Scatter::new()
                    .data(vec![vec![t0, x0]]) // Data point
                    .label(
                        Label::new()
                            .show(true)
                            .formatter("response time = 1/γ")
                            .position(LabelPosition::Right),
                    ),
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
    document::Script { src: asset!("/assets/echarts/echarts.min.js") },
        div { style: "width: 100%; text-align: center;",
              div { id: "chart", style: "display: inline-block;",
            }
          }
    )
}
