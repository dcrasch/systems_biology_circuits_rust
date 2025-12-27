use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use charming::{
    component::{Axis, DataZoom, DataZoomType, Legend},
    element::{AxisPointer, ItemStyle, LineStyle, NameLocation},
    series::Line,
    Chart, WasmRenderer,
};

#[component]
pub fn LineChartSIR() -> Element {
    let mut r_infection = use_signal(|| 0.1 / 1000.0);
    let mut r_healing = use_signal(|| 0.01);
    let mut i0 = use_signal(|| 1_isize);
    let mut s0 = use_signal(|| 999_isize);
    let mut ti = use_signal(||250_isize);
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
        if let Some(Some(data)) = &*series.read() {
            let series_s = data
                .0
                .iter()
                .zip(data.1.iter())
                .map(|(x, y)| vec![*x, *y])
                .collect();
            let series_i = data
                .0
                .iter()
                .zip(data.2.iter())
                .map(|(x, y)| vec![*x, *y])
                .collect();
            let series_r = data
                .0
                .iter()
                .zip(data.3.iter())
                .map(|(x, y)| vec![*x, *y])
                .collect();
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
                        .data(series_s), // Use data safely
                )
                .series(
                    Line::new()
                        .show_symbol(false)
                        .item_style(ItemStyle::new().color("red"))
                        .name("I")
                        .line_style(LineStyle::new().color("red"))
                        .data(series_i), // Use data safely
                )
                .series(
                    Line::new()
                        .show_symbol(false)
                        .name("R")
                        .item_style(ItemStyle::new().color("green"))
                        .line_style(LineStyle::new().color("green"))
                        .data(series_r), // Use data safely
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
                }
            }
        }

        div { class: "flex flex-col",
            label { class: "text-sm text-gray-700", "Healing rate" }
            input {
                class: "border rounded px-2 py-1 w-32",
                r#type: "number",
                step: "0.001",
                value: "{r_healing}",
                oninput: move |e| {
                    if let Ok(v) = e.value().parse::<f64>() {
                        r_healing.set(v);
                    }
                }
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
                    if let Ok(v) = e.value().parse::<isize>() {
                        i0.set(v);
                    }
                }
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
                    if let Ok(v) = e.value().parse::<isize>() {
                        s0.set(v);
                    }
                }
            }
        }

        div { class: "flex flex-col block",
            label { class: "text-sm text-gray-700", "ti" }
            input {
                class: "border rounded px-2 py-1 w-32",
                r#type: "number",
                step: "1",
                value: "{ti}",
                oninput: move |e| {
                    if let Ok(v) = e.value().parse::<isize>() {
                        ti.set(v);
                    }
                }
            }
        }
    }

        )
}

#[server]
async fn get_sir_data(
    r_inf: f64,
    r_healing: f64,
    s0: isize,
    i0: isize,
    ti : isize,
) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>), ServerFnError> {
    // Return some meaningful data or simulate a fetch error

    #![allow(non_snake_case)]
    use rebop::define_system;

    define_system! {
        r_infection r_healing;
        SIR { S, I, R }
        infection   : S + I => 2 I @ r_infection
        healing     : I     => R   @ r_healing
    }

    let mut series_t = Vec::new();
    let mut series_s = Vec::new();
    let mut series_i = Vec::new();
    let mut series_r = Vec::new();

    let mut problem = SIR::new();
    problem.r_infection = r_inf;
    problem.r_healing = r_healing;
    problem.S = s0;
    problem.I = i0;

    for t in 0..ti {
        problem.advance_until(t as f64);
        series_s.push(problem.S as f64);
        series_i.push(problem.I as f64);
        series_r.push(problem.R as f64);
        series_t.push(problem.t as f64);
    }
    Ok((series_t, series_s, series_i, series_r))
}
