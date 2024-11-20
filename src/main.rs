use std::collections::HashSet;

use std::path::PathBuf;

use code_report_rs::ReportGen;
use iced::widget::{button, checkbox, column, container, scrollable, text};
use iced::{Alignment, Element, Length};
use rfd::{FileDialog, MessageDialog};
use walkdir::WalkDir;

#[tokio::main]
pub async fn main() -> iced::Result {
    iced::application("Генератор сводной таблицы", Example::update, Example::view).run()
}

#[derive(Default, Debug)]
struct Example {
    folder_path: PathBuf,
    extensions: Vec<String>,
    selected_extensions: Vec<String>,
}

#[derive(Debug, Clone)]
enum Message {
    PickFolder,
    Generate,
    ToggleExtension(String),
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleExtension(ext) => {
                if self.selected_extensions.contains(&ext) {
                    self.selected_extensions.retain(|value| *value != ext);
                } else {
                    self.selected_extensions.push(ext);
                    self.selected_extensions.sort();
                }
            }

            Message::PickFolder => {
                let folder = FileDialog::new().pick_folder();

                if let Some(path) = folder {
                    // Get file extensions
                    let mut extensions_set: HashSet<String> = HashSet::new();

                    for e in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
                        if e.metadata().unwrap().is_file() && e.path().extension().is_some() {
                            println!("{}", e.path().display());

                            extensions_set.insert(
                                e.path()
                                    .extension()
                                    .unwrap_or_default()
                                    .to_str()
                                    .unwrap_or_default()
                                    .to_string(),
                            );
                        }
                    }

                    self.folder_path = path;

                    self.extensions = extensions_set.into_iter().collect();
                    self.extensions.sort();
                } else {
                    self.extensions.clear();
                }
            }
            Message::Generate => {
                // Placeholder action for "Generate" button
                if let Some(save_path) = FileDialog::new()
                    .set_title("Сохранить отчет")
                    .add_filter("Word File", &["docx"])
                    .set_can_create_directories(true)
                    .save_file()
                {
                    let future = tokio::task::spawn_blocking({
                        let selected_extensions = self.selected_extensions.clone();
                        let folder_path = self.folder_path.clone();
                        move || {
                            ReportGen::new_from_path(&folder_path, &selected_extensions)
                                .unwrap()
                                .save_file(&save_path)
                                .unwrap();
                            MessageDialog::new()
                                .set_level(rfd::MessageLevel::Info)
                                .set_title("Сохранено успешно")
                                .set_description("Файл был успешно сохранен")
                                .show()
                        }
                    });
                    drop(future);
                    // self.running_tasks.push(future);
                }
            } // Message::Update =>{

              //     if !self.running_tasks.is_empty() {
              //         println!("check");
              //         self.running_tasks.retain(|task| !task.is_finished());
              //     }

              // },
        }
    }

    fn view(&self) -> Element<Message> {
        let extensions_list = scrollable(
            column(
                self.extensions
                    .iter()
                    .map(|ext| {
                        container(
                            checkbox(format!(".{ext}"), self.selected_extensions.contains(ext))
                                .on_toggle(|_| Message::ToggleExtension(ext.clone())),
                        )
                        .style(container::dark)
                        .padding(10)
                        .into()
                    })
                    .collect::<Vec<_>>(),
            )
            .spacing(10)
            .align_x(Alignment::Center)
            .width(Length::Fill)
            .padding(10),
        )
        .height(Length::Fill);

        // Main layout
        container(
            column![
                text("Генератор сводной таблицы исходного кода").size(30),
                button("Выбрать папку").on_press(Message::PickFolder),
                extensions_list,
                button("Сгенерировать").on_press(Message::Generate),
                // container({
                //     if self.running_tasks.is_empty() {
                //         container("")
                //     } else {
                //         container(text(format!("{} задач в работе", self.running_tasks.len())))
                //     }
                // }),
            ]
            .spacing(10)
            .align_x(Alignment::Center),
        )
        .padding(20)
        .height(Length::Fill)
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .into()
    }
}
