use io::OutputContent;
use plotters::prelude::*;
use serde_json;
use std::fs::read_to_string;

use clap::Parser;
use miette::{Context, IntoDiagnostic, Result};

use crate::io::XicOutput;

mod cli;
mod io;

#[derive(Debug, Clone, Copy)]
pub(crate) struct XicPlot {
    pub(crate) rt_min: f32,
    pub(crate) rt_max: f32,
    pub(crate) intensity_min: f32,
    pub(crate) intensity_max: f32,
}

pub(crate) struct XicPlotBuilder {
    plot: Option<XicPlot>,
}

impl XicPlotBuilder {
    pub(crate) fn new() -> Self {
        Self { plot: None }
    }

    fn with_xic(self, xic: OutputContent) -> Self {
        let mut new_xic = XicPlot::new();
        // let mut rt_min = f32::MIN_POSITIVE;
        // let mut rt_max = f32::MAX;
        // let mut intensity_min = f32::MIN_POSITIVE;
        // let mut intensity_max = f32::MAX;
        match xic {
            OutputContent::Content {
                meta: _,
                retention_times,
                intensities,
            } => {
                if let Some(rt) = retention_times {
                    new_xic.rt_min = rt.first().unwrap().to_owned();
                    new_xic.rt_max = rt.last().unwrap().to_owned();
                }

                if let Some(intensities) = intensities {
                    new_xic.intensity_min = intensities
                        .iter()
                        .min_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap()
                        .to_owned();
                    new_xic.intensity_max = intensities
                        .iter()
                        .max_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap()
                        .to_owned();
                }
            }
            OutputContent::ContentBase64 {
                meta: _,
                retention_times: _,
                intensities: _,
            } => todo!(),
        };
        if let Some(plot) = self.plot {
            new_xic.rt_min = plot.rt_min.min(new_xic.rt_min);
            new_xic.rt_max = plot.rt_max.max(new_xic.rt_max);
            new_xic.intensity_min = plot.intensity_min.min(new_xic.intensity_min);
            new_xic.intensity_max = plot.intensity_max.max(new_xic.intensity_max);
        }

        Self {
            plot: Some(new_xic),
        }
    }

    fn build(self) -> XicPlot {
        self.plot.unwrap()
    }
}

impl XicPlot {
    pub(crate) fn new() -> Self {
        Self {
            rt_min: 0.0,
            rt_max: 0.0,
            intensity_min: 0.0,
            intensity_max: 0.0,
        }
    }
}

impl From<XicOutput> for XicPlot {
    fn from(value: XicOutput) -> Self {
        let mut plot_builder = XicPlotBuilder::new();
        for content in value.content {
            plot_builder = plot_builder.with_xic(content);
        }
        plot_builder.build()
    }
}

fn main() -> Result<()> {
    let args = cli::Args::parse();

    let xic_str = read_to_string(args.thermorawfileparser_xic_output)
        .into_diagnostic()
        .wrap_err("Failed to read ThermoRawFileParser xic output file")?;

    let xic: XicOutput = serde_json::from_str(&xic_str)
        .into_diagnostic()
        .wrap_err("Failed to parse ThermoRawFileParser xic output file")?;

    let xic_plot = XicPlot::from(xic.clone());

    let root_area =
        BitMapBackend::new(args.output.as_str(), (args.width, args.height)).into_drawing_area();

    root_area.fill(&WHITE).into_diagnostic()?;

    let top = xic_plot.clone().intensity_max + (5.0 / 100.0 * xic_plot.clone().intensity_max);

    let mut cc = ChartBuilder::on(&root_area)
        .margin(5)
        .set_all_label_area_size(40)
        .build_cartesian_2d(
            xic_plot.clone().rt_min..xic_plot.clone().rt_max,
            xic_plot.clone().intensity_min..top,
        )
        .into_diagnostic()?;

    cc.configure_mesh()
        .x_labels(20)
        .y_labels(10)
        .disable_mesh()
        // .x_label_formatter(&|v| format!("{:.1}", v))
        // .y_label_formatter(&|v| format!("{:.1}", v))
        .draw()
        .into_diagnostic()?;

    xic.content.into_iter().for_each(|x| match x {
        OutputContent::Content {
            meta,
            retention_times,
            intensities,
        } => {
            if let (Some(rt_vec), Some(intensities_vec)) = (retention_times, intensities) {
                let values = rt_vec.into_iter().zip(intensities_vec.into_iter());
                cc.draw_series(LineSeries::new(values, &BLUE)).unwrap();
            }
        }
        OutputContent::ContentBase64 {
            meta,
            retention_times,
            intensities,
        } => todo!(),
    });

    // cc.draw_series(PointSeries::of_element(
    //     (-3.0f32..2.1f32).step(1.0).values().map(|x| (x, x.sin())),
    //     5,
    //     ShapeStyle::from(&RED).filled(),
    //     &|coord, size, style| {
    //         EmptyElement::at(coord)
    //             + Circle::new((0, 0), size, style)
    //             + Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
    //     },
    // ))
    // .into_diagnostic()?;

    root_area.present().expect("Unable to write result to file");

    Ok(())
}
