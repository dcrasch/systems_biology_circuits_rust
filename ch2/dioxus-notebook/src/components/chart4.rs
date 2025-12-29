use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use charming::{
    component::{Axis, DataZoom, DataZoomType, Legend},
    element::{AxisPointer, ItemStyle, LineStyle, NameLocation},
    series::Line,
    Chart, WasmRenderer,
};

use ode_solvers::dopri5::*;
use ode_solvers::*;

#[component]
pub fn LineChartSIR() -> Element {
    let mut r_infection = use_signal(|| 0.1 / 1000.0);
    let mut r_healing = use_signal(|| 0.01);
    let mut i0 = use_signal(|| 1.0_f64);
    let mut s0 = use_signal(|| 999.0_f64);
    let mut ti = use_signal(||250.0_f64);
    let series = use_resource(move || {
        let r_inf = *r_infection.read();
        let r_heal = *r_healing.read();
        let i0 = *i0.read();
        let s0 = *s0.read();
        let ti = *ti.read();
        async move { get_sir_data(r_inf, r_heal, s0, i0, ti).await.ok() }
    });

    let mut chart = use_signal(|| Chart::new());
    let renderer = use_signal(|| WasmRenderer::new(600, 400));
    let mut echarts = use_signal(|| None);

    use_effect(move || {
        if let Some(Some((series_s, series_i, series_r))) = &*series.read() {
            let updated_chart = Chart::new()
                .legend(Legend::new())
                .x_axis(
                    Axis::new()
                        .name("Time")
                        .name_gap(25)
                        .name_location(NameLocation::Middle)
                        .axis_pointer(AxisPointer::new().z(100)),
                )
                .y_axis(
                    Axis::new()
                        .name("S,I,R")
                        .name_gap(25)
                        .name_location(NameLocation::Middle),
                )
                .series(
                    Line::new()
                        .show_symbol(false)
                        .name("S")
                        .item_style(ItemStyle::new().color("blue"))
                        .line_style(LineStyle::new().color("blue"))
                        .data(series_s.to_vec()), // Use data safely
                )
                .series(
                    Line::new()
                        .show_symbol(false)
                        .item_style(ItemStyle::new().color("red"))
                        .name("I")
                        .line_style(LineStyle::new().color("red"))
                        .data(series_i.to_vec()), // Use data safely
                )
                .series(
                    Line::new()
                        .show_symbol(false)
                        .name("R")
                        .item_style(ItemStyle::new().color("green"))
                        .line_style(LineStyle::new().color("green"))
                        .data(series_r.to_vec()), // Use data safely
                )
                .data_zoom(DataZoom::new().type_(DataZoomType::Inside).realtime(true));

            chart.set(updated_chart);

            *echarts.write() = Some(
                renderer
                    .read_unchecked()
                    .render("chart3", &chart.read())
                    .unwrap(),
            );
        }
    });

    rsx! (
        div { style: "width: 100%; text-align: center;",
            h1 { style: "color:black", "SIR" }
            // Show loading spinner while waiting for data
            if series.read().is_none() {
                div { style: "padding: 20px;", "Loading data..." }
            } else if series.read().as_ref().unwrap().is_none() {
                div { style: "color: red;", "Failed to load data!" }
            } else {
                div { id: "chart3", style: "display: inline-block;" }
            }
        
        }
        div { class: "flex gap-4 justify-center mb-4",

            div { class: "flex flex-col",
                label { class: "text-sm text-gray-700", "Infection rate" }
                input {
                    class: "border rounded px-2 py-1 w-32",
                    r#type: "number",
                    step: "0.00001",
                    value: "{r_infection}",
                    oninput: move |e| {
                        if let Ok(v) = e.value().parse::<f64>() {
                            r_infection.set(v);
                        }
                    },
                }
            }

            div { class: "flex flex-col",
                label { class: "text-sm text-gray-700", "Recovery rate" }
                input {
                    class: "border rounded px-2 py-1 w-32",
                    r#type: "number",
                    step: "0.001",
                    value: "{r_healing}",
                    oninput: move |e| {
                        if let Ok(v) = e.value().parse::<f64>() {
                            r_healing.set(v);
                        }
                    },
                }
            }

            div { class: "flex flex-col",
                label { class: "text-sm text-gray-700", "Initial I" }
                input {
                    class: "border rounded px-2 py-1 w-32",
                    r#type: "number",
                    step: "1",
                    value: "{i0}",
                    oninput: move |e| {
                        if let Ok(v) = e.value().parse::<f64>() {
                            i0.set(v);
                        }
                    },
                }
            }

            div { class: "flex flex-col",
                label { class: "text-sm text-gray-700", "Initial S" }
                input {
                    class: "border rounded px-2 py-1 w-32",
                    r#type: "number",
                    step: "1",
                    value: "{s0}",
                    oninput: move |e| {
                        if let Ok(v) = e.value().parse::<f64>() {
                            s0.set(v);
                        }
                    },
                }
            }


            div { class: "flex flex-col",
                label { class: "text-sm text-gray-700", "ti" }
                input {
                    class: "border rounded px-2 py-1 w-32",
                    r#type: "number",
                    step: "1",
                    value: "{ti}",
                    oninput: move |e| {
                        if let Ok(v) = e.value().parse::<f64>() {
                            ti.set(v);
                        }
                    },
                }
            }
        }

    )
}

struct Model {
    beta : f64,
    gamma: f64,
}

type State = Vector3<f64>;
type Time = f64;

impl ode_solvers::System<f64, State> for Model {
    fn system(&self, _t: Time, x: &State, dx: &mut State) {
        let s = x[0];
        let i = x[1];

        dx[0] = -self.beta * s * i;
        dx[1] = self.beta * s * i - self.gamma * i;
        dx[2] = self.gamma * i;
    }
}

#[server]
async fn get_sir_data(
    beta: f64,
    gamma: f64,
    s0: f64,
    i0: f64,
    ti : f64,
) -> Result<(Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<Vec<f64>>), ServerFnError> {
    // Return some meaningful data or simulate a fetch error

        let system = Model {
            beta,
            gamma
        };
        let x = State::new(s0, i0, 0.0);
        let t = 0.0;
        let t_end = ti;
        let dt = 0.003; // Step size to get ~4000 points

        let mut stepper = Dopri5::new(system, t, t_end, dt, x, 1e-6_f64, 1e-6_f64);
        stepper.integrate().expect("failed integration");
        let series_s = stepper
            .x_out()
            .iter()
            .zip(stepper.y_out().iter())
            .map(|(x, y)| vec![*x, y[0]])
            .collect();
        let series_i = stepper
            .x_out()
            .iter()
            .zip(stepper.y_out().iter())
            .map(|(x, y)| vec![*x, y[1]])
            .collect();
        let series_r = stepper
            .x_out()
            .iter()
            .zip(stepper.y_out().iter())
            .map(|(x, y)| vec![*x, y[2]])
            .collect();  
    Ok((series_s, series_i, series_r))
}
