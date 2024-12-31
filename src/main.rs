use iced::widget::{button, column, row, text_input, text, progress_bar, Space};
use iced::{Alignment, Element, Task, Color};
use iced::theme::{Theme};
use iced_futures::futures;
use futures::channel::mpsc;
extern crate chrono;
use std::process::Command as stdCommand;
use std::path::Path;
use std::io::{Write, BufRead, BufReader};
use std::fs::File;
use std::time::Duration as timeDuration;
use std::thread::sleep;
use chrono::{Duration, Utc};
use chrono::prelude::*;

mod get_winsize;
mod inputpress;
mod execpress;
use get_winsize::get_winsize;
use inputpress::inputpress;
use execpress::execpress;

pub fn main() -> iced::Result {

     let mut widthxx: f32 = 1350.0;
     let mut heightxx: f32 = 750.0;
     let (errcode, errstring, widtho, heighto) = get_winsize();
     if errcode == 0 {
         widthxx = widtho as f32 - 20.0;
         heightxx = heighto as f32 - 75.0;
         println!("{}", errstring);
     } else {
         println!("**ERROR {} get_winsize: {}", errcode, errstring);
     }
     iced::application(Megaparse::title, Megaparse::update, Megaparse::view)
        .window_size((widthxx, heightxx))
        .theme(Megaparse::theme)
        .run_with(Megaparse::new)

}

struct Megaparse {
    mega_value: String,
    mess_color: Color,
    msg_value: String,
    utc_value: String,
    rows_num: u64,
    rclone_value: String,
    do_progress: bool,
    progval: f32,
    tx_send: mpsc::UnboundedSender<String>,
    rx_receive: mpsc::UnboundedReceiver<String>,
}

#[derive(Debug, Clone)]
enum Message {
    MegaPressed,
    RclonePressed,
    UtcChanged(String),
    ExecPressed,
    ExecxFound(Result<Execx, Error>),
    ProgressPressed,
    ProgRtn(Result<Progstart, Error>),
}

impl Megaparse {
    fn new() -> (Megaparse, iced::Task<Message>) {
        let (tx_send, rx_receive) = mpsc::unbounded();
        ( Self { mega_value: "--".to_string(), msg_value: "no message".to_string(),
               rows_num: 0, mess_color: Color::from([0.0, 0.0, 1.0]), rclone_value: "--".to_string(), 
               utc_value: "-1".to_string(), do_progress: false, progval: 0.0, tx_send, rx_receive,
 
          },
          Task::none()
        )
    }

    fn title(&self) -> String {
        String::from("Mega parse of file list for just name and size, but creates another with time -- iced")
    }

    fn update(&mut self, message: Message) -> Task<Message>  {
        match message {
            Message::MegaPressed => {
               let mut inputstr: String = self.mega_value.clone();
               if !Path::new(&inputstr).exists() {
                   if Path::new(&self.rclone_value).exists() {
                       inputstr = self.rclone_value.clone();
                   }
               }
               let (errcode, errstr, newinput) = inputpress(inputstr);
               self.msg_value = errstr.to_string();
               if errcode == 0 {
                   if Path::new(&newinput).exists() {
                       self.mess_color = Color::from([0.0, 1.0, 0.0]);
                       self.mega_value = newinput.to_string();
                       self.rows_num = 0;
                       let mut bolok = true;
                       let file = File::open(newinput).unwrap();
                       let mut reader = BufReader::new(file);
                       let mut line = String::new();
                       let mut linenum: u64 = 0;
                       loop {
                          match reader.read_line(&mut line) {
                             Ok(bytes_read) => {
                                 // EOF: save last file address to restart from this address for next run
                                 if bytes_read == 0 {
                                     break;
                                 }
                                 linenum = linenum + 1;
                             }
                             Err(_err) => {
                                 self.msg_value = "error reading mega ".to_string();
                                 self.mess_color = Color::from([1.0, 0.0, 0.0]);
                                 bolok = false;   
                                 break;
                             }
                          };
                       }
                       if bolok {
                           self.rows_num = linenum;
                           self.mess_color = Color::from([0.0, 1.0, 0.0]);
                           self.msg_value = "got mega ls file and retrieved its number of rows".to_string();
                       } 
                   } else {
                       self.mess_color = Color::from([1.0, 0.0, 0.0]);
                       self.msg_value = format!("mega ls file does not exist: {}", newinput);
                   }
               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
               }
               Task::none()
           }
            Message::UtcChanged(value) => { self.utc_value = value; Task::none() }
            Message::RclonePressed => {
               let mut inputstr: String = self.rclone_value.clone();
               if !Path::new(&inputstr).exists() {
                   if Path::new(&self.mega_value).exists() {
                       inputstr = self.mega_value.clone();
                   }
               }
               let (errcode, errstr, newinput) = inputpress(inputstr);
               self.msg_value = errstr.to_string();
               if errcode == 0 {
                   self.rclone_value = newinput.to_string();
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
               }
               Task::none()
            }
            Message::ExecPressed => {
               let (errcode, errstr) = execpress(self.mega_value.clone(), self.rclone_value.clone(), self.rows_num.clone(), self.utc_value.clone());
               self.msg_value = errstr.to_string();
               if errcode == 0 {
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
                   Task::perform(Execx::execit(self.mega_value.clone(),self.rclone_value.clone(), self.rows_num.clone(), self.utc_value.clone(), self.tx_send.clone()), Message::ExecxFound)

               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
                   Task::none()
               }
            }
            Message::ExecxFound(Ok(exx)) => {
               self.msg_value = exx.errval.clone();
               if exx.errcd == 0 {
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
               }
               Task::none()
            }
            Message::ExecxFound(Err(_error)) => {
               self.msg_value = "error in copyx copyit routine".to_string();
               self.mess_color = Color::from([1.0, 0.0, 0.0]);
               Task::none()
            }
            Message::ProgressPressed => {
                   self.do_progress = true;
                   Task::perform(Progstart::pstart(), Message::ProgRtn)
            }
            Message::ProgRtn(Ok(_prx)) => {
              if self.do_progress {
                let mut inputval  = " ".to_string();
                let mut bgotmesg = false;
                while let Ok(Some(input)) = self.rx_receive.try_next() {
                   inputval = input;
                   bgotmesg = true;
                }
                if bgotmesg {
                    let progvec: Vec<&str> = inputval[0..].split("|").collect();
                    let lenpg1 = progvec.len();
                    if lenpg1 == 3 {
                        let prog1 = progvec[0].to_string();
                        if prog1 == "Progress" {
                            let num_int: i32 = progvec[1].parse().unwrap_or(-9999);
                            if num_int == -9999 {
                                println!("progress numeric not numeric: {}", inputval);
                            } else {
                                let dem_int: i32 = progvec[2].parse().unwrap_or(-9999);
                                if dem_int == -9999 {
                                    println!("progress numeric not numeric: {}", inputval);
                                } else {
                                    self.progval = 100.0 * (num_int as f32 / dem_int as f32);
                                    self.msg_value = format!("Convert progress: {}", self.progval);
                                    self.mess_color = Color::from([0.0, 0.0, 1.0]);
                                }
                            }
                        } else {
                            println!("message not progress: {}", inputval);
                        }
                    } else {
                        println!("message not progress: {}", inputval);
                    }
                }             
                Task::perform(Progstart::pstart(), Message::ProgRtn)
              } else {
                Task::none()
              }
            }
            Message::ProgRtn(Err(_error)) => {
                self.msg_value = "error in Progstart::pstart routine".to_string();
                self.mess_color = Color::from([1.0, 0.0, 0.0]);
               Task::none()
            }

        }
    }

    fn view(&self) -> Element<Message> {
        column![
            row![text("Message:").size(20),
                 text(&self.msg_value).size(30).color(*&self.mess_color),
            ].align_y(Alignment::Center).spacing(10).padding(10),
            row![button("mega-ls -lr input file Button").on_press(Message::MegaPressed),
                 text(&self.mega_value).size(20).width(1000)
            ].align_y(Alignment::Center).spacing(10).padding(10),
            row![text(format!("number of rows: {}", self.rows_num)).size(20), Space::with_width(100),
                 text("UTC offset: "),
                 text_input("No input....", &self.utc_value)
                            .on_input(Message::UtcChanged).padding(10).size(20),
            ].align_y(Alignment::Center).spacing(10).padding(10),
            row![button("rclone lsf ... --format ps input file Button").on_press(Message::RclonePressed),
                 text(&self.rclone_value).size(20).width(1000)
            ].align_y(Alignment::Center).spacing(10).padding(10),
            row![Space::with_width(200),
                 button("Exec Button").on_press(Message::ExecPressed),
            ].align_y(Alignment::Center).spacing(10).padding(10),
            row![button("Start Progress Button").on_press(Message::ProgressPressed),
                 progress_bar(0.0..=100.0,self.progval),
                 text(format!("{}%", &self.progval)).size(30),
            ].align_y(Alignment::Center).spacing(5).padding(10),
         ]
        .padding(5)
        .align_x(Alignment::Start)
        .into()
    }

    fn theme(&self) -> Theme {
       Theme::Dracula
    }
}

#[derive(Debug, Clone)]
struct Execx {
    errcd: u32,
    errval: String,
}

impl Execx {
//    const TOTAL: u16 = 807;

    async fn execit(mega_value: String, rclone_value: String, rows_num: u64, utc_value: String, tx_send: mpsc::UnboundedSender<String>,) -> Result<Execx, Error> {
     let mut errstring  = "begin of async".to_string();
     let mut errcode: u32 = 0;
     let mut bolok = true;
     let numrows: u64 = rows_num;
     let numutc = utc_value.parse().unwrap_or(-99);
     let targettfullname = format!("{}T.csv", mega_value);
     let targetntfullname = format!("{}nT.csv", mega_value);
     let file = File::open(mega_value).unwrap(); 
     let mut reader = BufReader::new(file);
     let mut targetfilet = File::create(targettfullname.clone()).unwrap();
     let mut targetfilent = File::create(targetntfullname.clone()).unwrap();
     let mut line = String::new();
     let mut linenum = 0;
     let mut sdir = "--".to_string();
     let mut topdir = String::new();
     loop {
        match reader.read_line(&mut line) {
           Ok(bytes_read) => {
               if bytes_read == 0 {
                   break;
               }
               linenum = linenum + 1;
               if !line.starts_with("d") {
                   if line.starts_with("---- ") || line.starts_with("-ep- ") {
                       let sizval = line.get(10..22).unwrap().to_string();
         		       let test_int: i64 = sizval.trim().parse().unwrap_or(-99);
                       let ssize;
         		       if test_int >= 0 {
         		           ssize = format!("{}",test_int);
         		       } else {
         		           ssize = format!("invalid size value: -{}-", sizval.trim());
         		       }
                       let mut sdate = line.get(23..33).unwrap().to_string();
                       let mut stime = line.get(34..42).unwrap().to_string();
                       if numutc != 0 {
                           let mut dateyr1: i64 = 0;
                           let mut datemo1: i64 = 0;
                           let mut dateday1: i64 = 0;
                           let mut datehr1: i32 = 0;
                           let mut datemin1: i32 = 0;
                           let mut datesec1: i32 = 0;
                           let date1ar1: Vec<&str> = sdate[0..].split("-").collect();
                           let lendat1 = date1ar1.len();
                           if (lendat1 > 3) | (lendat1 < 3) {
                               errstring = "invalid date".to_string();
                               errcode = 1;
                           } else {
                               let time1ar1: Vec<&str> = stime[0..].split(":").collect();
                               let lentime1 = time1ar1.len();
                               if (lentime1 > 3) | (lentime1 < 3) {
                                  errstring = "invalid time".to_string();
                                  errcode = 2;
                               } else {
                                   for indl in 0..lendat1 {
                                        let date_int: i32 = date1ar1[indl].parse().unwrap_or(-9999);
                                        if date_int == -9999 {
                                            errstring = "invalid time 2".to_string();
                                            errcode = 3;
                                        } else {
                                            match indl {
                                              0 => dateyr1 = date_int as i64,
                                              1 => datemo1 = date_int as i64,
                                              2 => dateday1 = date_int as i64,
                                              _ => errcode = 4,
                                            }
                                        }
                                   }
                                   if errcode == 0 {
                                       for indk in 0..lentime1 {
                                            let time_int: i32 = time1ar1[indk].parse().unwrap_or(-9999);
                                            if time_int == -9999 {
                                                errstring = "invalid time 3".to_string();
                                                errcode = 5;
                                            } else {
                                                match indk {
                                                  0 => datehr1 = time_int as i32,
                                                  1 => datemin1 = time_int as i32,
                                                  2 => datesec1 = time_int as i32,
                                                  _ => errcode = 6,
                                                }
                                            }
                                       }
                                   }
                               }      
                           }
                           if errcode == 0 {
//                                println!("yr -{}- mo -{}- da -{}- hr -{}- min -{}- sec -{}-", dateyr1, datemo1, dateday1, datehr1, datemin1, datesec1);

                               let mut dateto = Utc.with_ymd_and_hms(dateyr1 as i32, datemo1 as u32, dateday1 as u32, datehr1.try_into().unwrap(), datemin1.try_into().unwrap(), datesec1.try_into().unwrap()).unwrap();
                               dateto = dateto + Duration::hours(numutc);
                               sdate = format!("{}", dateto.format("%Y-%m-%d"));
                               stime = format!("{}", dateto.format("%H:%M:%S"));
                           } else {
                               sdate = format!("invalid date or time: {}", sdate);
                           }
                       }
                       let sfile = line.get(43..(bytes_read-1)).unwrap().to_string();
                       let stroutputt;
                       let stroutputnt;
                       if sdir == "" {
                           stroutputt = format!("{},{},{} {}", sfile, ssize, sdate, stime);
                           stroutputnt = format!("{},{}", sfile, ssize);
                       } else {
                           stroutputt = format!("{}/{},{},{} {}", sdir, sfile, ssize, sdate, stime);
                           stroutputnt = format!("{}/{},{}", sdir, sfile, ssize);
                       }
                       writeln!(&mut targetfilet, "{}", stroutputt).unwrap();
                       writeln!(&mut targetfilent, "{}", stroutputnt).unwrap();
                   } else {
                       if line.contains(":") {
//                         println!("sdir:-{}- topdir:-{}- line: {} value: {}", sdir, topdir, linenum, line.get(0..(bytes_read-1)).unwrap());
                           let lcurrpos = line.find(":").unwrap();
                           if sdir == "--" {
                               sdir = "".to_string();
                               topdir = line.get(..lcurrpos).unwrap().to_string();
                           } else {
                               let lcurrtop = line.find(&topdir).unwrap();
                               sdir = line.get((lcurrtop+topdir.len()+1)..lcurrpos).unwrap().to_string();
                           }
//                       } else {
                       }
                   }
               }
               let msgx = format!("Progress|{}|{}", linenum, numrows);
               tx_send.unbounded_send(msgx).unwrap();
               if linenum > numrows {
                   break;
               }
               line.clear();
           }
           Err(_err) => {
               errstring = "error reading mega-ls file: do file i and iconv".to_string();
               errcode = 1;
               bolok = false;   
               break;
           }
        }
     }
     let targetrcfullname = format!("{}nT.csv", rclone_value);
     if bolok {
         let filerc = File::open(rclone_value).unwrap(); 
         let mut readerrc = BufReader::new(filerc);
         let mut targetrcfilent = File::create(targetrcfullname.clone()).unwrap();
         let mut linerc = String::new();
         let mut linenumrc = 0;
         loop {
            match readerrc.read_line(&mut linerc) {
               Ok(bytes_readrc) => {
                   if bytes_readrc == 0 {
                       break;
                   }
                   linenumrc = linenumrc + 1;
                   let linerclen = linerc.len();
                   if !linerc.starts_with('"') {
                       let lineval = linerc.get(0..(linerclen-1)).unwrap().to_string();
                       writeln!(&mut targetrcfilent, "{}", lineval).unwrap();
                   } else {
                       let lendqtpos = linerc.rfind('"').unwrap();
                       if lendqtpos < 2 {
                           let lineval1 = linerc.get(0..(linerclen-1)).unwrap().to_string();
                           writeln!(&mut targetrcfilent, "{}", lineval1).unwrap();
                       } else {
                           let linercst = linerc.get(1..lendqtpos).unwrap().to_string();
                           let linercend = linerc.get((lendqtpos+1)..(linerclen-1)).unwrap().to_string();
                           writeln!(&mut targetrcfilent, "{}{}", linercst, linercend).unwrap();
                       }
                   }
                   linerc.clear();
                }
                Err(_err) => {
                   errstring = "error reading rclone file: do file i and iconv".to_string();
                   errcode = 1;
                   bolok = false;   
                   break;
                }
            }
         }
     }
     if bolok {
         errstring = "source file exists and read".to_string();
         let sortfullname = format!("{}__sort", targetntfullname);

         let _output = stdCommand::new("sort")
                                         .arg("-o")
                                         .arg(&sortfullname)
                                         .arg(&targetntfullname)
                                         .output()
                                         .expect("failed to execute process");
         stdCommand::new("meld")
                           .arg(&sortfullname)
                           .arg(&targetrcfullname)
                           .spawn()
                           .expect("failed to execute process");

     }
     Ok(Execx {
            errcd: errcode,
            errval: errstring,
        })
    }
}
#[derive(Debug, Clone)]
pub enum Error {
//    APIError,
//    LanguageError,
}

// loop thru by sleeping for 5 seconds
#[derive(Debug, Clone)]
pub struct Progstart {
//    errcolor: Color,
//    errval: String,
}

impl Progstart {

    pub async fn pstart() -> Result<Progstart, Error> {
//     let errstring  = " ".to_string();
//     let colorx = Color::from([0.0, 0.0, 0.0]);
     sleep(timeDuration::from_secs(5));
     Ok(Progstart {
//            errcolor: colorx,
//            errval: errstring,
        })
    }
}
