use butterworth::{Filter, Cutoff};

use plotly::common::Mode;
use plotly::common::Title;
use plotly::layout::{Axis, Layout};
use plotly::{Plot, Scatter};

use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit};
//use spectrum_analyzer::windows::hann_window;
use spectrum_analyzer::scaling::divide_by_N_sqrt;
use core::f32::consts::PI;
// replacement for std functions like sin and cos in no_std-environments
use libm::cosf;

/// Minimal example.
/// 
pub fn hann_window_mia(samples:&[f64]) -> Vec<f32> {
    let mut windowed_samples = Vec::with_capacity(samples.len());
    let samples_len_f32 = samples.len() as f32;
    for (i, sample) in samples.iter().enumerate() {
        let two_pi_i = 2.0 * PI * i as f32;
        let idontknowthename = cosf(two_pi_i / samples_len_f32);
        let multiplier = 0.5 * (1.0 - idontknowthename);
        windowed_samples.push(multiplier * (*sample as f32));
    }
    windowed_samples
}


fn add_to_plot(plot:&mut Plot, y:&Vec<f64>, name:&str)
{
    let x_value: Vec<f64> = (0..=y.len()).map(|x| x as f64).map(|x| (x + (1) as f64)).collect();


    //let trace1 = Scatter::new(x_value, y).color(Black).mode(Mode::Markers);

    let trace1 = Scatter::new(x_value.clone(), y.clone())
    .mode(Mode::Lines)
    .name("Trace 1")
    .line(plotly::common::Line::new().width(5 as f64).dash(plotly::common::DashType::Dot)).name(name);

    plot.add_trace(trace1);
}

fn add_to_plot_freq(plot:&mut Plot, x:Vec<f32>, y:Vec<f32>, name:&str)
{

    //let trace1 = Scatter::new(x_value, y).color(Black).mode(Mode::Markers);

    let trace1 = Scatter::new(x.clone(), y.clone())
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

    let mut plot_freq = Plot::new();//creamos un plot
    let layout = Layout::new().x_axis(Axis::new().title(Title::from("X Axis")))
        .y_axis(Axis::new().title(Title::from("Y Axis")))
        .title(Title::from("Señal Original Frequencia"));
    plot_freq.set_layout(layout);

    // Create a mix of low and high frequency sine functions
    let data = (0..=128).map(|x| x as f64).map(|x| (x * 0.1).sin() + (x * 0.75).sin()).collect();
    // Assuming the sample rate is 100 Hz, design a 4th order lowpass filter with an 8 Hz cutoff
    let filter_bajo = Filter::new(4, 100., Cutoff::LowPass(8.)).unwrap();
    // Apply a bidirectional filter to the data
    let filtered_data_paso_bajo = filter_bajo.bidirectional(&data).unwrap();

    // Assuming the sample rate is 100 Hz, design a 4th order highpass filter with an 8 Hz cutoff
    let filter_alto = Filter::new(4, 100., Cutoff::HighPass(8.)).unwrap();
    // Apply a bidirectional filter to the data
    let filtered_data_paso_alto = filter_alto.bidirectional(&data).unwrap();


    add_to_plot(&mut plot_inicial, &data, "Original");
    add_to_plot(&mut plot_final, &filtered_data_paso_bajo, "Filtro paso bajo");
    add_to_plot(&mut plot_final, &filtered_data_paso_alto, "Filtro paso alto");

    plot_inicial.show();
    plot_final.show();

    //ANALISIS DE FREQÜENCIA

    // YOU need to implement the samples source; get microphone input for example
    let data: Vec<f64>= (0..=128).map(|x| x as f64).map(|x| (x * 0.1).sin() + (x * 0.75).sin()).collect();
    // apply hann window for smoothing; length must be a power of 2 for the FFT
    // 2048 is a good starting point with 44100 kHz
    let hann_window = hann_window_mia(&data[0..128]);
    // calc spectrum
    let spectrum_hann_window = samples_fft_to_spectrum(
        // (windowed) samples
        &hann_window,
        // sampling rate
        44100,
        // optional frequency limit: e.g. only interested in frequencies 50 <= f <= 150?
        FrequencyLimit::All,
        // optional scale
        Some(&divide_by_N_sqrt),
    ).unwrap();

    let mut x_freq: Vec<f32> = Vec::new();
    let mut y_freq: Vec<f32>= Vec::new();
    println!("Frecuencias Originales");
    for (fr, fr_val) in spectrum_hann_window.data().iter() {
        println!("{}Hz => {}", fr, fr_val);
        x_freq.push(fr.val());
        y_freq.push(fr_val.val());

    }
    add_to_plot_freq(&mut plot_freq, x_freq, y_freq, "Frequencia sin filtrar");

    // apply hann window for smoothing; length must be a power of 2 for the FFT
    // 2048 is a good starting point with 44100 kHz
    let hann_window = hann_window_mia(&filtered_data_paso_alto[0..128]);
    // calc spectrum
    let spectrum_hann_window = samples_fft_to_spectrum(
        // (windowed) samples
        &hann_window,
        // sampling rate
        44100,
        // optional frequency limit: e.g. only interested in frequencies 50 <= f <= 150?
        FrequencyLimit::All,
        // optional scale
        Some(&divide_by_N_sqrt),
    ).unwrap();

    let mut x_freq: Vec<f32> = Vec::new();
    let mut y_freq: Vec<f32>= Vec::new();
    println!("Frecuencia Filtro paso Alto");
    for (fr, fr_val) in spectrum_hann_window.data().iter() {
        println!("{}Hz => {}", fr, fr_val);
        x_freq.push(fr.val());
        y_freq.push(fr_val.val());

    }
    add_to_plot_freq(&mut plot_freq, x_freq, y_freq, "Frequencia Filtro Paso Alto");


    plot_freq.show();
}
