use charts;
use std::io;
use std::path::Path;
use super::Sample;
use super::Opts;

#[allow(non_upper_case_globals)]
const chart_height : isize = 800;
#[allow(non_upper_case_globals)]
const chart_with : isize = 1500;

pub(crate) fn plot_graph<P: AsRef<Path>>(data: &Vec<Sample>, file_output: P, opts: &Opts) -> Result<(), io::Error> {
	let scale_x = generate_scale_x(data);
	let scale_y_cpu = generate_scale_cpu(data);
	let scale_y_mem = generate_scale_mem(data);

	let data_cpu = generate_data_cpu(data);
	let data_mem = generate_data_mem(data);

	let line_cpu = charts::LineSeriesView::new()
		.set_x_scale(&scale_x)
		.set_y_scale(&scale_y_cpu)
		.set_marker_type(charts::MarkerType::Circle)
		.set_colors(charts::Color::from_vec_of_hex_strings(vec!["firebrick"]))
		.set_custom_data_label("CPU".to_owned())
		.set_label_visibility(false)
		.load_data(&data_cpu)
		.unwrap();

	let title = match opts.pid {
		Some(p) => format!("PID {}", p),
		None => opts.command.join(" ")
	};

	let line_mem = charts::LineSeriesView::new()
		.set_x_scale(&scale_x)
		.set_y_scale(&scale_y_mem)
		.set_marker_type(charts::MarkerType::Circle)
		.set_colors(charts::Color::from_vec_of_hex_strings(vec!["steelblue"]))
		.set_custom_data_label("RSS".to_owned())
		.set_label_visibility(false)
		.load_data(&data_mem)
		.unwrap();

	charts::Chart::new()
    .set_width(chart_with + 200)
    .set_height(chart_height + 200)
    .set_margins(100, 100, 100, 100) 
		.add_title(title)
    .add_view(&line_cpu)
    .add_view(&line_mem)
    .add_axis_bottom(&scale_x)
    .add_axis_left(&scale_y_cpu)
    .add_axis_right(&scale_y_mem)
    .add_left_axis_label("CPU Usage [%]")
    .add_right_axis_label("RSS Memory Usage")
    .add_bottom_axis_label("Seconds after start")
    .add_legend_at(charts::AxisPosition::Bottom)
    .save(file_output).unwrap();

	Ok(())
}

fn generate_scale_x(data: &Vec<Sample>) -> charts::ScaleLinear {
	let max_ts = data[ data.len() - 1 ].ts;
	charts::ScaleLinear::new()
        .set_domain(vec![0_f32, max_ts])
        .set_range(vec![0, chart_with])
}

fn generate_scale_cpu(data: &Vec<Sample>) -> charts::ScaleLinear {
	let mut max = 0f32;
	for s in data {
		if let Some(std::cmp::Ordering::Less) = max.partial_cmp(&s.cpu) {
			max = s.cpu;
		}
	}
	charts::ScaleLinear::new()
        .set_domain(vec![0_f32, max])
        .set_range(vec![chart_height, 0])

}

fn generate_scale_mem(data: &Vec<Sample>) -> charts::ScaleLinear {
	let mut max = 0u64;
	for s in data {
		if let Some(std::cmp::Ordering::Less) = max.partial_cmp(&s.rss) {
			max = s.rss;
		}
	}
	eprintln!("Found maximum memory at {}", max);
	charts::ScaleLinear::new()
        .set_domain(vec![0f32, max as f32])
        .set_range(vec![chart_height, 0])

}

fn generate_data_cpu(data: &Vec<Sample>) -> Vec<(f32, f32)> {
	data.iter().map(|s| (s.ts as f32, s.cpu)).collect()
}


fn generate_data_mem(data: &Vec<Sample>) -> Vec<(f32, f32)> {
	data.iter().map(|s| (s.ts as f32, s.rss as f32)).collect()
}


