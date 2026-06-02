use eframe::egui::{self, generate_loader_id};
use rfd::FileDialog;
use rfd::MessageButtons::Ok;
use std::{io::ErrorKind::NetworkDown, path::PathBuf, sync::mpsc};

use crate::proc::ProcessInfo;
use crate::proc::list;
use crate::proc::delete;
use crate::proc::state_of_proc;
mod proc;
struct MyAPP{
    proc_pid: Option<usize>,
    proc_list: Vec<ProcessInfo>,
    selected_info: String, 
}

impl Default for MyAPP  {
    fn default()->Self{
let initial_list = list().unwrap_or_default();

        Self{
            proc_list: initial_list,
        proc_pid: None,
        selected_info: "".to_string(),
        }
    }
}

fn main(){
    let options = eframe::NativeOptions::default();

    eframe::run_native("PROC-VIEW", options, Box::new(|c| Box::new(MyAPP::default())));
}


impl eframe::App for MyAPP{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("all procces");

            if ui.button("refres procces").clicked(){
                 if let std::result::Result::Ok(new_list) = list(){
                           self.proc_list = new_list;
                 }
            }



            ui.label(&self.selected_info);

            let row_height = 26.0;
            let total_rows = self.proc_list.len();

   egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show_rows(ui, row_height, total_rows, |ui, row_range| {

                    for row_idx in row_range{
                        if let Some(proc) = self.proc_list.get(row_idx){
                            let button_text = format!("PID: {:<6} | {}", proc.pid, proc.name);

                            ui.menu_button(&button_text, |ui|{
                                if ui.button("see stats").clicked(){
                                     let state_txt =  state_of_proc(proc.pid);
                                     self.selected_info = state_txt;
                                ui.close_menu();

                                     
                                }

                                if ui.button("kill proc").clicked() {
                                    delete(proc.pid); 
                                    
                                    self.selected_info = format!("proc {} (PID {}) killed!", proc.name, proc.pid);
                                    ui.close_menu();
                                }
                            });
                        }
                    }
                });

        });
    }
}