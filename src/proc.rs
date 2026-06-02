use std::fs::DirEntry;
use std::io;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::usize;
pub struct ProcessInfo {
    pub pid: usize,
    pub name: String,
}
pub fn list()->std::io::Result<Vec<ProcessInfo>>  {
 

    let mut vec_proc: Vec<ProcessInfo> = Vec::new();

   let entires = fs::read_dir("/proc")?;

   for entry in entires{
       if let Ok(entry) = entry{

           if entry.file_type()?.is_dir(){
               let name = entry.file_name().to_string_lossy().into_owned();
           if  name.chars().all(|c| c.is_ascii_digit()){
               let path = entry.path();
            let comm_path =    path.join("comm");
            if let Ok(coom_name) = std::fs::read_to_string(comm_path){
              let clear_name = coom_name.trim();
              if let Ok(pid_num) = name.parse::<usize>(){
              let procc = ProcessInfo{
                  pid: pid_num,
                  name: clear_name.to_string()
              };
              vec_proc.push(procc);
              }
                  }
              }
            
            
           }
           }
       }
   Ok(vec_proc)
}


pub fn delete(pid: usize){
  if  let Ok(list) = list(){
      if let Some(proc) = list.iter().find(|p|  p.pid == pid){
          let _ = std::process::Command::new("kill")
                          .arg("-9")
                          .arg(&proc.pid.to_string())
                          .output();
                      println!("procces deleted");
      }else{
          println!("no procces")
      }
  }

   
}
pub fn state_of_proc(pid: usize)->String{
    if let Ok(list) = list(){
        if let Some(procc) = list.iter().find(|procc| procc.pid == pid){
            let pid_str = procc.pid.to_string();
            let state_str  =  fs::read_to_string(format!("/proc/{}/stat",pid_str)).ok();
           if let Some(state_str) = state_str{
               let str = parse_full_process_info(state_str);
               return str;
           }
        }
    }
"Process not found or unreadable".to_string()
}



fn parse_full_process_info(line: String) -> String {
    let fields: Vec<&str> = line.split_whitespace().collect();

    if fields.len() < 24 {
        return "error of state".to_string();
    }

    let pid = fields[0];
    let name = fields[1]; 
    let state_char = fields[2];

    let state = match state_char {
        "R" => "Running",
        "S" => "Sleeping",
        "D" => "D-state",
        "Z" => "Zombie",
        "T" => "Stopped",
        _ => state_char,
    };

    let v_mem_bytes: u64 = fields[22].parse().unwrap_or(0);
    let v_mem_mb = v_mem_bytes / 1024 / 1024;
    let rss_pages: u64 = fields[23].parse().unwrap_or(0);
    let rss_bytes = rss_pages * 4096; 
    let rss_mb = rss_bytes / 1024 / 1024;

    format!(
        "PROCCES: {}\n\
         PID: {}\n\
         STATE: {}\n\
         RAM MEMORY (RSS): {} МБ\n\
         VIRTUAL MEMORY: {} МБ",
        name, pid, state, rss_mb, v_mem_mb
    )
}

