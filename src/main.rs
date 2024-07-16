use butterworth::{Filter, Cutoff};

use plotly::common::Mode;
use plotly::common::Title;
use plotly::layout::{Axis, Layout};
use plotly::{Plot, Scatter};


fn add_to_plot(plot:&mut Plot, y:Vec<f64>, name:&str)
{
    let x_value: Vec<f64> = (0..=y.len()).map(|x| x as f64).map(|x| (x + (1) as f64)).collect();


    //let trace1 = Scatter::new(x_value, y).color(Black).mode(Mode::Markers);

    let trace1 = Scatter::new(x_value.clone(), y.clone())
    .mode(Mode::Lines)
    .name("Trace 1")
    .line(plotly::common::Line::new().width(5 as f64).dash(plotly::common::DashType::Dot)).name(name);

    plot.add_trace(trace1);
}

fn main() {
    let mut plot_inicial = Plot::new();//creamos un plot
    let layout = Layout::new().x_axis(Axis::new().title(Title::from("X Axis")))
        .y_axis(Axis::new().title(Title::from("Y Axis")))
        .title(Title::from("Señales iniciales"));
    plot_inicial.set_layout(layout);

    let mut plot_final = Plot::new();//creamos un plot
    let layout = Layout::new().x_axis(Axis::new().title(Title::from("X Axis")))
        .y_axis(Axis::new().title(Title::from("Y Axis")))
        .title(Title::from("Señales filtradas"));
    plot_final.set_layout(layout);

    // Create a mix of low and high frequency sine functions
    let data = (0..=100).map(|x| x as f64).map(|x| (x * 0.1).sin() + (x * 0.75).sin()).collect();
    // Assuming the sample rate is 100 Hz, design a 4th order lowpass filter with an 8 Hz cutoff
    let filter_bajo = Filter::new(4, 100., Cutoff::LowPass(8.)).unwrap();
    // Apply a bidirectional filter to the data
    let filtered_data_paso_bajo = filter_bajo.bidirectional(&data).unwrap();

    // Assuming the sample rate is 100 Hz, design a 4th order lowpass filter with an 8 Hz cutoff
    let filter_alto = Filter::new(4, 100., Cutoff::HighPass(8.)).unwrap();
    // Apply a bidirectional filter to the data
    let filtered_data_paso_alto = filter_alto.bidirectional(&data).unwrap();


    add_to_plot(&mut plot_inicial, data, "Original");
    add_to_plot(&mut plot_final, filtered_data_paso_bajo, "Filtro paso bajo");
    add_to_plot(&mut plot_final, filtered_data_paso_alto, "Filtro paso alto");

    plot_inicial.show();
    plot_final.show();

}
