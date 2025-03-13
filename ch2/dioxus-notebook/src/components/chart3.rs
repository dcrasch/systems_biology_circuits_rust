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
                        .name("Concentration")
                        .name_gap(25)
                        .name_location(NameLocation::Middle),
                )
                .series(
                    Line::new()
                        .data(data.clone()) // Use data safely
                        .show_symbol(false)
                        .name("Concentration m"),
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
async fn get_sir_data() ->  Result<Vec<f64>, ServerFnError> {
 // Return some meaningful data or simulate a fetch error

 #![allow(non_snake_case)]
use rebop::define_system;

define_system! {
    r_inf r_heal;
    SIR { S, I, R }
    infection   : S + I => 2 I @ r_inf
    healing     : I     => R   @ r_heal
}

    let mut num = Vec::new();
    for _ in 0..10000 {
        let mut problem = SIR::new();
        problem.r_inf = 0.1 / 1000.;
        problem.r_heal = 0.01;
        problem.S = 999;
        problem.I = 1;
        problem.advance_until(250.);
        num.push(problem.R as f64);
    }
    Ok(num)
}