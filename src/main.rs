use plotters::prelude::*;

const OUT_FILE_NAME: &'static str = "plots/sample.png";

const DARK_BLUE: RGBColor = RGBColor(0, 69, 134);
const DARK_ORANGE: RGBColor = RGBColor(255, 66, 14);
const YELLOW: RGBColor = RGBColor(255, 211, 32);
const MED_GREEN: RGBColor = RGBColor(87, 157, 28);
const DARK_RED: RGBColor = RGBColor(126, 0, 33);
const LIGHT_BLUE: RGBColor = RGBColor(131, 202, 255);
const DARK_GREEN: RGBColor = RGBColor(49, 65, 4);
const LIGHT_GREEN: RGBColor = RGBColor(175, 207, 0);
const DARK_PURPLE: RGBColor = RGBColor(75, 31, 111);
const LIGHT_ORANGE: RGBColor = RGBColor(255, 150, 14);
const MED_RED: RGBColor = RGBColor(197, 0, 10);
const MED_BLUE: RGBColor = RGBColor(0, 132, 210);

fn collatz(x: u32) -> u32 {
    if x % 2 == 1 {
        3 * x + 1
    } else {
        x / 2
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_area = BitMapBackend::new(OUT_FILE_NAME, (1280, 800)).into_drawing_area();

    root_area.fill(&WHITE)?;

    let mut cc = ChartBuilder::on(&root_area)
        .margin(5)
        .set_all_label_area_size(50)
        .caption("Collatz Conjecture", ("sans-serif", 40))
        .build_cartesian_2d(0f32..25f32, 0f32..180f32)?;

    cc.configure_mesh()
        .x_labels(20)
        .y_labels(10)
        .disable_mesh()
        .x_label_formatter(&|v| format!("{:.1}", v))
        .y_label_formatter(&|v| format!("{:.1}", v))
        .draw()?;

    let colors = [
        &DARK_BLUE,
        &DARK_ORANGE,
        &YELLOW,
        &MED_GREEN,
        &DARK_RED,
        &LIGHT_BLUE,
        &DARK_GREEN,
        &LIGHT_GREEN,
        &DARK_PURPLE,
        &LIGHT_ORANGE,
        &MED_RED,
        &MED_BLUE,
    ];
    for n in 1..24 {
        let mut points: Vec<(u32, u32)> = Vec::new();

        println!("***** {n} *****");
        let mut p = n;
        let mut step = 1;
        points.push((step, p));
        step += 1;
        while p > 1 {
            let pn = collatz(p);

            points.push((step, pn));
            p = pn;
            step += 1;
        }

        for p in &points {
            println!("{:?}", &p)
        }
        println!("*************");

        let points: Vec<(f32, f32)> = points
            .iter()
            .cloned()
            .map(|p| (p.0 as f32, p.1 as f32))
            .collect();

        let style_index = n as usize % colors.len();
        let color = colors.get(style_index).unwrap().clone();
        let style = ShapeStyle {
            color: color.to_rgba(),
            filled: false,
            stroke_width: 2,
        };
        cc.draw_series(LineSeries::new(points, style))?
            .label(format!("{n}"))
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &color.to_owned()));
    }

    cc.configure_series_labels().border_style(&BLACK).draw()?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root_area.present().expect(
        "Unable to write result to file, please make sure 'plots' dir exists under current dir",
    );
    println!("Result has been saved to {}", OUT_FILE_NAME);
    Ok(())
}

#[test]
fn entry_point() {
    for n in 1..24 {
        dbg!(n, collatz(n));
    }
    // main().unwrap()
}
