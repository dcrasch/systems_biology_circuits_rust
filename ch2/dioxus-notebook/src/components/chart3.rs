use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use ode_solvers::dopri5::*;
use ode_solvers::*;

use charming::{
    component::{
        Axis, Brush, BrushType, DataZoom, DataZoomType, Feature, Legend, Title, Toolbox,
        ToolboxDataZoom,
    },
    element::{
        formatter::FormatterFunction, AxisPointer, AxisType, Label, LabelPosition, MarkLine,
        MarkLineData, MarkLineVariant, MarkPoint, MarkPointData, NameLocation, Symbol, Tooltip,
        Trigger,
    },
    series::{Line, Scatter},
    Chart, ChartResize, HtmlRenderer, WasmRenderer,
};

#[component]
pub fn LineChart3() -> Element {
    let series = use_resource(|| async move {
        get_sir_data().await.ok()
    });

    let mut chart = use_signal(|| Chart::new());
    let renderer = use_signal(|| WasmRenderer::new(600, 400));
    let mut echarts = use_signal(|| None);

    use_effect(move || {
        if let Some(Some(data)) = &*series.read() {

            let series_s = data.0.iter().zip(data.1.iter()).map(|(x,y)|vec![*x,*y]).collect();
            let series_i = data.0.iter().zip(data.2.iter()).map(|(x,y)|vec![*x,*y]).collect();
            let series_r = data.0.iter().zip(data.3.iter()).map(|(x,y)|vec![*x,*y]).collect();
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
                        .data(series_s) // Use data safely
                )
                .series(
                    Line::new()
                        .show_symbol(false)
                        .name("I")
                        .data(series_i) // Use data safely
                )
                .series(
                    Line::new()
                        .show_symbol(false)
                        .name("R")
                        .data(series_r) // Use data safely
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
        document::Script { src: asset!("/assets/echarts/echarts.min.js") }
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
    )
}

#[server]
async fn get_sir_data() ->  Result<(Vec<f64>,Vec<f64>,Vec<f64>,Vec<f64>), ServerFnError> {
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
    problem.r_infection = 0.1 / 1000.;
    problem.r_healing = 0.01;
    problem.S = 999;
    problem.I = 1;
    for t in 0..250 {
        problem.advance_until(t as f64);
        series_s.push(problem.S as f64);
        series_i.push(problem.I as f64);
        series_r.push(problem.R as f64);
        series_t.push(problem.t as f64);
    }
    Ok((series_t,series_s,series_i,series_r))
}