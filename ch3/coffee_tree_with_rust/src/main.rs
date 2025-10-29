use ode_solvers::dop853::*;
use ode_solvers::*;

use comrak::{Options, markdown_to_html};
use df_interchange::Interchange;
use plotlars::{LinePlot, Plot, Rgb};
use polars::prelude::*;

type State = Vector3<f64>;
type Time = f64;

struct Model {
    a: f64,
    beta: f64,
    k: f64,
    gamma: f64,
    b: f64,
    d: f64,
}

impl System<f64, State> for Model {
    fn system(&self, _t: Time, y: &State, dy: &mut State) {
        let y1 = y[0];
        let y2 = y[1];
        let y3 = y[2];

        dy[0] = self.a * y1 - self.beta * y1 * y2 + self.k * y3 * y2;
        dy[1] = self.beta * y1 * y2 - self.k * y3 * y2 - self.gamma * y2;
        dy[2] = self.b * y3 * y2 - self.d * y3;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let system = Model {
        a: 0.0000283 / (24.0 * 24.0),
        beta: 0.0298 / (24.0 * 24.0),
        k: 0.07333 / (24.0 * 24.0),
        gamma: 0.005 / (24.0 * 24.0),
        b: 0.0025 / (24.0 * 24.0),
        d: 0.00125,
    };

    let y0 = State::new(1000.0, 500.0, 0.0);
    let t_start = 0.0;
    let t_end = 700.0;
    let h_init = 1.0;

    let mut stepper = Dop853::new(system, t_start, t_end, h_init, y0, 1e-6, 1e-6);
    match stepper.integrate() {
        Ok(stats) => {
            println!("Integration successful: {}", stats);

            let t_series = Column::new("t".into(), stepper.x_out().to_vec());
            let y1_series = Column::new(
                "T<sub>s</sub>(t)".into(),
                stepper.y_out().iter().map(|v| v[0]).collect::<Vec<_>>(),
            );
            let y2_series = Column::new(
                "T<sub>i</sub>(t)".into(),
                stepper.y_out().iter().map(|v| v[1]).collect::<Vec<_>>(),
            );
            let y3_series = Column::new(
                "S(t)".into(),
                stepper.y_out().iter().map(|v| v[2]).collect::<Vec<_>>(),
            );

            let mut df = DataFrame::new(vec![t_series, y1_series, y2_series, y3_series])?;

            CsvWriter::new(std::fs::File::create("model_answer_polars.csv")?).finish(&mut df)?;

            let df_0_50 = Interchange::from_polars_0_51(df)?.to_polars_0_50()?;

            let mut html = r#"<!DOCTYPE html>
                            <html>
                            <head>
                               <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.25/dist/katex.min.css" integrity="sha384-WcoG4HRXMzYzfCgiyfrySxx90XSl2rxY5mnVY5TwtWE6KLrArNKn0T/mOgNL0Mmi" crossorigin="anonymous">

    <!-- The loading of KaTeX is deferred to speed up page rendering -->
    <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.25/dist/katex.min.js" integrity="sha384-J+9dG2KMoiR9hqcFao0IBLwxt6zpcyN68IgwzsCSkbreXUjmNVRhPFTssqdSGjwQ" crossorigin="anonymous"></script>

    <!-- To automatically render math in text elements, include the auto-render extension: -->
    <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.25/dist/contrib/auto-render.min.js" integrity="sha384-hCXGrW6PitJEwbkoStFjeJxv+fSOOQKOPbJxSfM6G5sWZjAyWhXiTIIAmQqnlLlh" crossorigin="anonymous"
        onload="renderMathInElement(document.body);"></script>

                            <style>
                                body {background-color: #ffffffff; ;}
                                div {width: 900px;}
                                h1 {color: black;}
                                h2 {color: black;}
                                p {
                                    color: black;
                                    text-align: justify;
                                }
                                table, th, td {
                                    color: black;
                                    border: 1px solid;
                                    border-collapse: collapse;
                                    padding: 3px;
                                    text-align: left;
                                }
                                table {
                                    margin-left: none;
                                    margin-right: auto;
                                }
                                img {
                                    display: block;
                                    margin-left: none;
                                    width: 80%;
                                }
                            </style>
                            </head>
                            <body>
                            <div>"#
                .to_string();

            LinePlot::builder()
                .data(&df_0_50)
                .x("t")
                .y("T<sub>s</sub>(t)")
                .additional_lines(vec!["T<sub>i</sub>(t)", "S(t)"])
                .size(12)
                .colors(vec![Rgb(0, 255, 0), Rgb(255, 0, 0), Rgb(0, 0, 255)])
                .plot_title("Base line")
                .x_title("Time [in hours]")
                .y_title("Population in size")
                .build()
                .write_image("p1.svg", 1000, 600, 1.0)?;

            let mut options = Options::default();
            options.extension.table = true;
            let mut markdown = "# Mathematical model of coffee tree's rust control using snail as biological agents\n".to_string();
            
            markdown.push_str(r#"$$
            \frac{dT_s(t)}{dt} = a T_s(t) - \beta T_s(t) T_i(t) + k S(t) T_i(t)
            $$
            "#);
            markdown.push_str(r#"$$
            \frac{dT_s(t)}{dt} = \beta T_s(t) T_i(t) - k S(t) T_i(t) - \gamma T_i(t)
            $$
            "#);
            markdown.push_str(r#"$$
            \frac{dT_s(t)}{dt} = b S(t) T_i(t) - d S(t)
            $$
            "#);
            markdown.push_str("![alt text](p1.svg) \n");

            html.push_str(markdown_to_html(markdown.as_str(), &options).as_str());
            html.push_str(
                r#"</div>
                  </body>
                  </html>"#,
            );

            std::fs::write("line.html", html)?;
        }
        Err(e) => println!("❌ Integration error: {}", e),
    }

    Ok(())
}
