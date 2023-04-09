use std::{io::{stdin, Read}, process::Command};

use prettytable::{Table, row, format};
use sysinfo::{self, System as syspro, SystemExt, CpuExt, DiskExt};

pub fn main()
{
    let sys = syspro::new_all();
    let hostname = sys.host_name().unwrap() + "\n\n";
    let osystem = "OS: ".to_owned() + &sys.name().unwrap() + " " + &sys.os_version().unwrap() + "\n";
    let kernel = "Kernel version: ".to_owned() + &sys.kernel_version().unwrap() + "\n";

    let mut de = String::new();
    if cfg!(target_os = "linux") {
        de = "Desktop Enviroment: ".to_owned() + &std::env::var("XDG_CURRENT_DESKTOP").unwrap() + "\n";

    } else if cfg!(windows) {
        de = "Desktop Enviroment: Windows\n".to_owned();
    }

    let processes = "Processes: ".to_owned() + &sys.processes().len().to_string() + "\n";
    let boottime = "Time since boot: ".to_owned() + &(sys.uptime() / 3600).to_string() + " hours\n";



   let cpun = "CPU: ".to_owned() + &sys.global_cpu_info().brand() + "\n";
   let cpufreq = "CPU frequency: ".to_owned() + &sys.global_cpu_info().frequency().to_string() + "MHz\n";
   let cputh = "CPU threads: ".to_owned() + &sys.cpus().len().to_string() + "\n";
   let cpuco = "CPU cores: ".to_owned() + &sys.physical_core_count().unwrap().to_string()+ "\n";



    let mut gpu = String::new();
    if cfg!(target_os = "linux") {
        let res = Command::new("sh")
        .args(&["-c", "lspci | grep -I 'VGA\\|Display\\|3D'"])
        .output()
        .unwrap();

        gpu = "GPU: ".to_owned() + &String::from_utf8(res.stdout).unwrap_or("failed to get gpu info".to_string());
   } else if cfg!(windows) {
         let res = Command::new("wmic")
         .args(["path", "win32_VideoController", "get", "name"])
         .output()
         .unwrap();
        let gpuinfo = String::from_utf8(res.stdout).unwrap_or("\nfailed to get gpu info ".to_string());
        let mut lines = gpuinfo.lines().collect::<Vec<_>>();
        lines.remove(0);

        let gpuinfo = lines.join("\n");

        gpu = "GPU: ".to_owned() + &gpuinfo;
        gpu = gpu + "\n"
   }


   let mut disks = "Disks: ".to_owned();

    for disk in sys.disks(){
        disks = disks + &((disk.name().to_str().unwrap().to_string() + " " + &(disk.total_space() / 1073741824).to_string() + "GiB "));
    }
    disks = disks + "\n";
    
    

    let memory = "Memory: ".to_owned() + &(((sys.total_memory() as f32 / 1073741824.0 * 100.0).trunc()) / 100.0).to_string() + "GiB\n";
    let memoryuse = "Memory usage: ".to_owned() + &(((sys.used_memory() as f32 / 1073741824.0 * 100.0).trunc()) / 100.0).to_string() + "GiB\n";
    let swap = "Swap: ".to_owned() + &(((sys.total_swap() as f32 / 1073741824.0 * 100.0).trunc()) / 100.0).to_string() + "GiB\n";
    let swapuse = "Swap usage: ".to_owned() + &(((sys.used_swap() as f32 / 1073741824.0 * 100.0).trunc()) / 100.0).to_string() + "GiB\n";


    let mut info = String::new();
    info.push_str(&hostname);
    info.push_str(&osystem);
    info.push_str(&kernel);
    info.push_str(&de);

    info.push_str(&processes);
    info.push_str(&boottime);

    info.push_str(&cpun);
    info.push_str(&cpufreq);
    info.push_str(&cputh);
    info.push_str(&cpuco);

    info.push_str(&gpu);

    info.push_str(&disks);


    info.push_str(&memory);
    info.push_str(&memoryuse);
    info.push_str(&swap);
    info.push_str(&swapuse);
    
    let mut table = Table::new();
    table.add_row(row![Fcb->logo(), Fgb->info]);
    table.set_format(*format::consts::FORMAT_CLEAN);


    table.printstd();
    logo();

    if cfg!(windows){
        stdin().read(&mut [0]).unwrap();
    }
}

fn logo() -> String{
    let windows = "

                            .oodMMMM
                   .oodMMMMMMMMMMMMM
       ..oodMMM  MMMMMMMMMMMMMMMMMMM
 oodMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
 MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
 MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
 MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
 MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
 MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
 
 MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
 MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
 MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
 MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
 MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
 `^^^^^^MMMMMMM  MMMMMMMMMMMMMMMMMMM
       ````^^^^  ^^MMMMMMMMMMMMMMMMM
                      ````^^^^^^MMMM
    ";

    let linuxlogo = "
                  .88888888:.
                88888888.88888.
              .8888888888888888.
              888888888888888888
              88' _`88'_  `88888
              88 88 88 88  88888
              88_88_::_88_:88888
              88:::,::,:::::8888
              88`:::::::::'`8888
             .88  `::::'    8:88.
            8888            `8:888.
          .8888'             `888888.
         .8888:..  .::.  ...:'8888888:.
        .8888.'     :'     `'::`88:88888
       .8888        '         `.888:8888.
      888:8         .           888:88888
    .888:88        .:           888:88888:
    8888888.       ::           88:888888
    `.::.888.      ::          .88888888
   .::::::.888.    ::         :::`8888'.:.
  ::::::::::.888   '         .::::::::::::
  ::::::::::::.8    '      .:8::::::::::::.
 .::::::::::::::.        .:888:::::::::::::
 :::::::::::::::88:.__..:88888:::::::::::'
  `'.:::::::::::88888888888.88:::::::::'
        `':::_:' -- '' -'-' `':_::::'`
    ";
    if cfg!(target_os = "linux")
    {
        return linuxlogo.to_string();
    } else if cfg!(windows)
    {
        return windows.to_string();
    } else {
        panic!();
    }
}



