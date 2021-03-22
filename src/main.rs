extern crate clap;
extern crate reqwest;
mod mangadex_api;

use clap::{App, Arg};
use std::fs;
use std::fs::File;
use std::{io, thread, time};
use druid::{PlatformError, AppLauncher, Data, WindowDesc, Widget, LocalizedString, WidgetExt, WidgetPod, Point, Color};
use druid::widget::prelude::*;
use druid::widget::{Label, Button, Flex, Scroll, Painter};

fn main() -> Result<(), PlatformError> {
    // command line arguments
    // let args = App::new("mangadex-scraper")
    //     .version("0.6.0")
    //     .author("dyedquartz <dyedquartz@gmail.com>")
    //     .about("Scrapes manga off of mangadex.org")
    //     .arg(
    //         Arg::with_name("id")
    //             .help("ID of the item to download")
    //             .required(true)
    //             .index(1),
    //     )
    //     .arg(
    //         Arg::with_name("lang")
    //             .short("l")
    //             .long("language")
    //             .value_name("LANGUAGE")
    //             .help("Downloads chapters for specific languages")
    //             .takes_value(true),
    //     )
    //     .arg(
    //         Arg::with_name("chapter")
    //             .short("c")
    //             .long("chapter")
    //             .help("Downloads a single chapter"),
    //     )
    //     .arg(
    //         Arg::with_name("volume")
    //             .short("e")
    //             .long("volume")
    //             .takes_value(true)
    //             .value_name("VOLUME")
    //             .help("Downloads an entire volume"),
    //     )
    //     /*
    //     .arg(
    //         Arg::with_name("archive")
    //             .short("a")
    //             .long("archive")
    //             .help("archives into a zip"),
    //     )
    //     */
    //     .get_matches();

    let main_window = WindowDesc::new(ui_builder)
        .title("Hello, Druid!")
        .window_size((200.0, 100.0));

    let data = AppData {
        counter: 0,
    };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)

//     if args.is_present("chapter") && args.is_present("volume") {
//         println!("Both chapter and volume cannot be used at the same time");
//         std::process::exit(1);
//     }
//
//     let client = reqwest::blocking::Client::new();
//
//     if args.is_present("chapter") {
//         let chapter_data = mangadex_api::get_chapter_data(&client, args.value_of("id").unwrap());
//         let manga_data = mangadex_api::get_manga_data(&client, &chapter_data.manga_id.to_string());
//         let data = manga_data
//             .chapter
//             .get(&chapter_data.id.to_string())
//             .unwrap();
//         println!(
//             "Scraping '{} Vol. {} Ch. {} in {} from {}'",
//             manga_data.manga.title, data.volume, data.chapter, data.lang_code, data.group_name
//         );
//         download_chapter(&client, chapter_data.id.to_string(), data, &manga_data);
//     } else if args.is_present("volume") {
//         let volume = args.value_of("volume").unwrap();
//         let manga_data = mangadex_api::get_manga_data(&client, args.value_of("id").unwrap());
//         println!("Scraping '{} Vol. {}'", manga_data.manga.title, volume);
//         for (name, data) in &manga_data.chapter {
//             if data.volume != volume {
//                 continue;
//             }
//             if args.is_present("lang") {
//                 if data.lang_code != args.value_of("lang").unwrap() {
//                     continue;
//                 }
//             }
//
//             download_chapter(&client, name.to_string(), &data, &manga_data);
//         }
//     } else {
//         let manga_data = mangadex_api::get_manga_data(&client, args.value_of("id").unwrap());
//         //let mut chapter_count = 0;
//         println!(
//             "Scraping '{}' in {}",
//             manga_data.manga.title,
//             if !args.is_present("lang") {
//                 "All"
//             } else {
//                 args.value_of("lang").unwrap()
//             }
//         );
//
//         for (name, data) in &manga_data.chapter {
//             if args.is_present("lang") {
//                 if data.lang_code != args.value_of("lang").unwrap() {
//                     //chapter_count += 1;
//                     continue;
//                 }
//             }
//
//             download_chapter(&client, name.to_string(), data, &manga_data);
//             //chapter_count += 1;
//         }
//     }
//     Ok(())
}
//
// fn strip_characters(original: &str, to_strip: &str) -> String {
//     original
//         .chars()
//         .filter(|&c| !to_strip.contains(c))
//         .collect()
// }
//
// fn clean_title(original: &str) -> String {
//     original.replace(":", "-")
// }


// fn download_chapter(
//     client: &reqwest::blocking::Client,
//     name: String,
//     data: &mangadex_api::Chapter,
//     manga_data: &mangadex_api::MangaData,
// ) {
//     let chapter_data = mangadex_api::get_chapter_data(&client, &name);
//     //let mut page_count = 0;
//     //let page_length = &chapter_data.page_array.len();
//
//     for page in chapter_data.page_array {
//         let current_time = time::Instant::now();
//         let page_name = format!("{:0>8}", page.trim_start_matches(char::is_alphabetic));
//
//         let url = if chapter_data.server == "/data/" {
//             reqwest::Url::parse(&*format!(
//                 "https://mangadex.org/data/{}/{}",
//                 chapter_data.hash, page
//             ))
//             .unwrap()
//         } else {
//             reqwest::Url::parse(&*format!(
//                 "{}{}/{}",
//                 chapter_data.server, chapter_data.hash, page
//             ))
//             .unwrap()
//         };
//         //println!("downloading {}", &url);
//         let mut resp = client.get(url).send().unwrap();
//         fs::create_dir_all(strip_characters(
//             &*format!(
//                 "{} Vol. {} Ch. {} - {} ({})",
//                 clean_title(&*manga_data.manga.title),
//                 format!("{:0>4}", data.volume),
//                 format!("{:0>4}", data.chapter),
//                 data.group_name,
//                 data.lang_code
//             ),
//             "/",
//         ))
//         .unwrap();
//         let mut out = File::create(
//             std::path::Path::new(&*strip_characters(
//                 &*format!(
//                     "{} Vol. {} Ch. {} - {} ({})",
//                     clean_title(&*manga_data.manga.title),
//                     format!("{:0>4}", data.volume),
//                     format!("{:0>4}", data.chapter),
//                     data.group_name,
//                     data.lang_code,
//                 ),
//                 "/",
//             ))
//             .join(&page_name),
//         )
//         .expect("failure to create image");
//         let _copy = io::copy(&mut resp, &mut out);
//         let _copy = match _copy {
//             Ok(file) => file,
//             Err(error) => {
//                 println!("Error Copying to File, trying again: {}", error);
//                 std::fs::remove_file(
//                     std::path::Path::new(&*strip_characters(
//                         &*format!(
//                             "{} Vol. {} Ch. {} - {} ({})",
//                             clean_title(&*manga_data.manga.title),
//                             format!("{:0>4}", data.volume),
//                             format!("{:0>4}", data.chapter),
//                             data.group_name,
//                             data.lang_code,
//                         ),
//                         "/",
//                     ))
//                     .join(&page_name),
//                 )
//                 .unwrap();
//                 let url = if chapter_data.server == "/data/" {
//                     reqwest::Url::parse(&*format!(
//                         "https://mangadex.org/data/{}/{}",
//                         chapter_data.hash, page
//                     ))
//                     .unwrap()
//                 } else {
//                     reqwest::Url::parse(&*format!(
//                         "{}{}/{}",
//                         chapter_data.server, chapter_data.hash, page
//                     ))
//                     .unwrap()
//                 };
//                 //println!("downloading {}", &url);
//                 let mut resp = client.get(url).send().unwrap();
//                 fs::create_dir_all(strip_characters(
//                     &*format!(
//                         "{} Vol. {} Ch. {} - {} ({})",
//                         clean_title(&*manga_data.manga.title),
//                         format!("{:0>4}", data.volume),
//                         format!("{:0>4}", data.chapter),
//                         data.group_name,
//                         data.lang_code
//                     ),
//                     "/",
//                 ))
//                 .unwrap();
//                 let mut out = File::create(
//                     std::path::Path::new(&*strip_characters(
//                         &*format!(
//                             "{} Vol. {} Ch. {} - {} ({})",
//                             clean_title(&*manga_data.manga.title),
//                             format!("{:0>4}", data.volume),
//                             format!("{:0>4}", data.chapter),
//                             data.group_name,
//                             data.lang_code,
//                         ),
//                         "/",
//                     ))
//                     .join(&page_name),
//                 )
//                 .expect("failure to create image");
//                 io::copy(&mut resp, &mut out).expect("failure to copy to image a second time");
//                 0
//             }
//         };
//         //page_count += 1;
//         while time::Instant::now()
//             .duration_since(current_time)
//             .as_millis()
//             <= 1000
//         {
//             thread::sleep(time::Duration::from_millis(100));
//         }
//     }
//
//     println!(
//         "Downloaded '{} Vol. {} Ch. {} in {} from {}'",
//         manga_data.manga.title, data.volume, data.chapter, data.lang_code, data.group_name
//     );
// }

#[derive(Clone, Data)]
struct AppData {
    counter: i32,
}

fn ui_builder() -> impl Widget<AppData> {
    let text = LocalizedString::new("hello-counter")
        .with_arg("count", |data: &AppData, _env| (*data).counter.into());
    let label = Label::new(text).padding(5.0).center();

    let button_plus = Button::new("+1")
        .on_click(|_ctx, data: &mut AppData, _env| (*data).counter += 1)
        .padding(5.0);
    let button_minus = Button::new("-1")
        .on_click(|_ctx, data: &mut AppData, _env| (*data).counter -= 1)
        .padding(5.0);

    let flex = Flex::row()
        .with_child(button_plus)
        .with_spacer(1.0)
        .with_child(button_minus);

    // Flex::column()
    //     .with_child(label)
    //     .with_child(flex);

    Flex::column()
        .with_child(Label::new("Mangadex Scraper").padding(20.0))
        .with_spacer(1.0)
        .with_child(
        Scroll::new(
            SquaresGrid::new()
                .with_cell_size(Size::new(200.0, 240.0))
                .with_spacing(20.0)
                .with_child(label)
                .with_child(label_widget(
                    Label::new(LocalizedString::new("hi")).padding(5.0).center(),
                "Label2"
                ))
                .with_child(label_widget(
                    flex,
                    "Counter",
                ))
            )
            .vertical()
            .fix_height(480.0)
            .border(Color::WHITE, 1.0)
            .padding(20.0),
        )
}

fn label_widget<T: Data>(widget: impl Widget<T> + 'static, label: &str) -> impl Widget<T> {
    Flex::column()
        .must_fill_main_axis(true)
        .with_flex_child(widget.center(), 1.0)
        .with_child(
            Painter::new(|ctx, _: &_, _: &_| {
                let size = ctx.size().to_rect();
                ctx.fill(size, &Color::WHITE)
            }).fix_height(1.0),
        )
        .with_child(Label::new(label).center().fix_height(40.0))
        .border(Color::WHITE, 1.0)
}

// Grid widget

const DEFAULT_GRID_CELL_SIZE: Size = Size::new(100.0, 100.0);
const DEFAULT_GRID_SPACING: f64 = 10.0;

pub struct SquaresGrid<T> {
    widgets: Vec<WidgetPod<T, Box<dyn Widget<T>>>>,
    /// The number of widgets we can fit in the grid given the grid size.
    drawable_widgets: usize,
    cell_size: Size,
    spacing: f64,
}

impl<T> SquaresGrid<T> {
    pub fn new() -> Self {
        SquaresGrid {
            widgets: vec![],
            drawable_widgets: 0,
            cell_size: DEFAULT_GRID_CELL_SIZE,
            spacing: DEFAULT_GRID_SPACING,
        }
    }

    pub fn with_spacing(mut self, spacing: f64) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn with_cell_size(mut self, cell_size: Size) -> Self {
        self.cell_size = cell_size;
        self
    }

    pub fn with_child(mut self, widget: impl Widget<T> + 'static) -> Self {
        self.widgets.push(WidgetPod::new(Box::new(widget)));
        self
    }
}

impl<T> Default for SquaresGrid<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Data> Widget<T> for SquaresGrid<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        for widget in self.widgets.iter_mut() {
            widget.event(ctx, event, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        for widget in self.widgets.iter_mut() {
            widget.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        for widget in self.widgets.iter_mut() {
            widget.update(ctx, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let count = self.widgets.len() as f64;
        // The space needed to lay all elements out on a single line.
        let ideal_width = (self.cell_size.width + self.spacing + 1.0) * count + self.spacing;
        // Constrain the width.
        let width = ideal_width.min(bc.max().width).max(bc.min().width);
        // Given the width, the space needed to lay out all elements (as many as possible on each
        // line).
        let cells_in_row =
            ((width - self.spacing) / (self.cell_size.width + self.spacing)).floor() as usize;
        let (height, rows) = if cells_in_row > 0 {
            let mut rows = (count / cells_in_row as f64).ceil() as usize;
            let height_from_rows =
                |rows: usize| (rows as f64) * (self.cell_size.height + self.spacing) + self.spacing;
            let ideal_height = height_from_rows(rows);

            // Constrain the height
            let height = ideal_height.max(bc.min().height).min(bc.max().height);
            // Now calcuate how many rows we can actually fit in
            while height_from_rows(rows) > height && rows > 0 {
                rows -= 1;
            }
            // println!("count:{},  ideal_width: {},   width: {},      cells_in_row: {},    height: {},    rows: {}", count, ideal_width, width, cells_in_row, height, rows);
            (height, rows)
        } else {
            (bc.min().height, 0)
        };
        // Constrain the number of drawn widgets by the number there is space to draw.
        self.drawable_widgets = self.widgets.len().min(rows * cells_in_row);
        // Now we have the width and height, we can lay out the children.
        let mut x_position = self.spacing;
        let mut y_position = self.spacing;
        for (idx, widget) in self
            .widgets
            .iter_mut()
            .take(self.drawable_widgets)
            .enumerate()
        {
            widget.layout(
                ctx,
                &BoxConstraints::new(self.cell_size, self.cell_size),
                data,
                env,
            );
            widget.set_origin(ctx, data, env, Point::new(x_position, y_position));
            // Increment position for the next cell
            x_position += self.cell_size.width + self.spacing;
            // If we can't fit in another cell in this row ...
            if (idx + 1) % cells_in_row == 0 {
                // ... then start a new row.
                x_position = self.spacing;
                y_position += self.cell_size.height + self.spacing;
            }
        }
        Size { width, height }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        for widget in self.widgets.iter_mut().take(self.drawable_widgets) {
            widget.paint(ctx, data, env);
        }
    }
}