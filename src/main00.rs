use iced::widget::{button, column, row, text_input, text, horizontal_space, progress_bar};
use iced::{Alignment, Element, Command, Application, Length, Settings, Color};
use iced::theme::{self, Theme};
use iced::executor;
use iced::window;
use iced_futures::futures;
use futures::channel::mpsc;

use std::process::Command as stdCommand;
use std::path::{Path};
use std::io::{Write, BufRead, BufReader};
use std::fs::File;
use std::time::{Duration, Instant};
use std::thread::sleep;
/*
mod get_dirlist;
mod dirpressx;
mod diroutpressx;
mod create_mergelist;
mod parse_moddate;
mod dump_file;
mod get_strvector;
mod mergepressx;
mod copypressx;


use get_dirlist::get_dirlist;
use dirpressx::dirpressx;
use diroutpressx::diroutpressx;
use mergepressx::mergepressx;
use copypressx::copypressx;
use create_mergelist::create_mergelist;
*/
mod get_winsize;
mod inputpress;
// mod targetdirpress;
mod execpress;
use get_winsize::get_winsize;
use inputpress::inputpress;
// use targetdirpress::targetdirpress;
use execpress::execpress;

pub fn main() -> iced::Result {

     let mut widthxx: u32 = 1350;
     let mut heightxx: u32 = 750;
     let (errcode, errstring, widtho, heighto) = get_winsize();
     if errcode == 0 {
         widthxx = widtho - 20;
         heightxx = heighto - 75;
         println!("{}", errstring);
     } else {
         println!("**ERROR {} get_winsize: {}", errcode, errstring);
     }

     Megaparse::run(Settings {
        window: window::Settings {
            size: (widthxx, heightxx),
            ..window::Settings::default()
        },
        ..Settings::default()
     })
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
    Gotrows(Result<Getrowsx, Error>),
    ProgressPressed,
    ProgRtn(Result<Progstart, Error>),
}

impl Application for Megaparse {
    type Message = Message;
    type Theme = Theme;
    type Flags = ();
    type Executor = executor::Default;
    fn new(_flags: Self::Flags) -> (Megaparse, iced::Command<Message>) {
        let (tx_send, rx_receive) = mpsc::unbounded();
        let mut heightxx: f32 = 190.0;
        let (errcode, errstring, _widtho, heighto) = get_winsize();
        if errcode == 0 {
            heightxx = 190.0 + ((heighto as f32 - 768.0) / 2.0);
            println!("{}", errstring);
        } else {
         println!("**ERROR {} get_winsize: {}", errcode, errstring);
        }
        ( Self { mega_value: "--".to_string(), msg_value: "no message".to_string(),
               rows_num: 0, mess_color: Color::from([0.0, 0.0, 0.0]), rclone_value: "no directory".to_string(), 
               utc_value: "-1".to_string(), do_progress: false, progval: 0.0, tx_send, rx_receive,
 
          },
          Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Mega parse of file list -- iced")
    }

    fn update(&mut self, message: Message) -> Command<Message>  {
        match message {
            Message::MegaPressed => {
               let (errcode, errstr, newinput) = inputpress(self.mega_value.clone());
               self.msg_value = errstr.to_string();
               if errcode == 0 {
                   if Path::new(&newinput).exists() {
                       self.mess_color = Color::from([0.0, 1.0, 0.0]);
                       self.mega_value = newinput.to_string();
                       Command::perform(Getrowsx::getrows(self.mega_value.clone(), self.tx_send.clone()), Message::Gotrows)
                   } else {
                       self.mess_color = Color::from([1.0, 0.0, 0.0]);
                       self.msg_value = format!("mega ls file does not exist: {}", newinput);
                       Command::none()
                   }
               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
                   Command::none()
               }
            }
            Message::Gotrows(Ok(gotrows)) => {
               self.msg_value = gotrows.errval.clone();
               if gotrows.errcode == 0 {
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
               }
               self.rows_num = gotrows.rowsnum.clone();
               Command::none()
            }
            Message::Gotrows(Err(_error)) => {
               self.msg_value = "error in Gotrows routine".to_string();
               self.mess_color = Color::from([1.0, 0.0, 0.0]);
               Command::none()
            }
            Message::UtcChanged(value) => { self.utc_value = value; Command::none() }
            Message::RclonePressed => {
               let (errcode, errstr, newinput) = inputpress(self.rclone_value.clone());
               self.msg_value = errstr.to_string();
               if errcode == 0 {
                   self.rclone_value = newinput.to_string();
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
               }
               Command::none()
            }
            Message::ExecPressed => {
               let (errcode, errstr) = execpress(self.mega_value.clone(), self.rclone_value.clone(), self.rows_num.clone(), self.utc_value.clone());
               self.msg_value = errstr.to_string();
               if errcode == 0 {
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
                   Command::perform(Execx::execit(self.mega_value.clone(),self.rclone_value.clone(), self.rows_num.clone(), self.utc_value.clone()), Message::ExecxFound)

               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
                   Command::none()
               }
            }
            Message::ExecxFound(Ok(copyx)) => {
               self.msg_value = copyx.errval.clone();
               self.mess_color = copyx.errcolor.clone();
               Command::none()
            }
            Message::ExecxFound(Err(_error)) => {
               self.msg_value = "error in copyx copyit routine".to_string();
               self.mess_color = Color::from([1.0, 0.0, 0.0]);
               Command::none()
            }
            Message::ProgressPressed => {
                   self.do_progress = true;
                   Command::perform(Progstart::pstart(), Message::ProgRtn)
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
                        let prog1 = progvec[0].clone().to_string();
                        if prog1 == "Progress" {
                            let num_int: i32 = progvec[1].clone().parse().unwrap_or(-9999);
                            if num_int == -9999 {
                                println!("progress numeric not numeric: {}", inputval);
                            } else {
                                let dem_int: i32 = progvec[2].clone().parse().unwrap_or(-9999);
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
                Command::perform(Progstart::pstart(), Message::ProgRtn)
              } else {
                Command::none()
              }
            }
            Message::ProgRtn(Err(_error)) => {
                self.msg_value = "error in Progstart::pstart routine".to_string();
                self.mess_color = Color::from([1.0, 0.0, 0.0]);
               Command::none()
            }

        }
    }

    fn view(&self) -> Element<Message> {
        column![
            row![text("Message:").size(20),
                 text(&self.msg_value).size(30).style(*&self.mess_color),
            ].align_items(Alignment::Center).spacing(10).padding(10),
            row![button("mega-ls -lr input file Button").on_press(Message::MegaPressed).style(theme::Button::Secondary),
                 text(&self.mega_value).size(30), horizontal_space(20),
            ].align_items(Alignment::Center).spacing(10).padding(10),
            row![text(format!("number of rows: {}", self.rows_num)).size(20), horizontal_space(100),
                 text("UTC offset: "),
                 text_input("No input....", &self.utc_value)
                            .on_input(Message::UtcChanged).padding(10).size(20),
            ].align_items(Alignment::Center).spacing(10).padding(10),
            row![button("rclone lsf input file Button").on_press(Message::RclonePressed).style(theme::Button::Secondary),
                 text(&self.rclone_value).size(20),
            ].align_items(Alignment::Center).spacing(10).padding(10),
            row![horizontal_space(200),
                 button("Exec Button").on_press(Message::ExecPressed).style(theme::Button::Secondary),
            ].align_items(Alignment::Center).spacing(10).padding(10),
            row![button("Start Progress Button").on_press(Message::ProgressPressed),
                 progress_bar(0.0..=100.0,self.progval),
                 text(format!("{}%", &self.progval)).size(30),
            ].align_items(Alignment::Center).spacing(5).padding(10),
         ]
        .padding(5)
        .align_items(Alignment::Start)
        .into()
    }

    fn theme(&self) -> Theme {
//       Theme::Light
          Theme::custom(theme::Palette {
                        background: Color::from_rgb8(240, 240, 240),
                        text: Color::BLACK,
                        primary: Color::from_rgb8(230, 230, 230),
                        success: Color::from_rgb(0.0, 1.0, 0.0),
                        danger: Color::from_rgb(1.0, 0.0, 0.0),
                    })
               
    }
}

#[derive(Debug, Clone)]
struct Getrowsx {
    rowsnum: u64,
    errcode: u32,
    errval: String,
}

impl Getrowsx {

    async fn getrows(mega_value: String, tx_send: mpsc::UnboundedSender<String>,) -> Result<Getrowsx, Error> {
     let mut errstring  = " ".to_string();
     let mut errcd =  0;
     let mut bolok = true;
     let mut numrow = 0;
     let mut numprocess = 0;
     let file = File::open(mega_value).unwrap();
     let mut reader = BufReader::new(file);
     let mut line = String::new();
     let mut linenum: u64 = 0;
     let mut count: u64 = 0;
     let mut incrcount: u64 = 100000;
     loop {
        match reader.read_line(&mut line) {
           Ok(bytes_read) => {
               // EOF: save last file address to restart from this address for next run
               if bytes_read == 0 {
                   break;
               }
               linenum = linenum + 1;
               count = count + 1;
               if count > incrcount {
                   incrcount = incrcount + 100000;
                   let msgx = format!("Progress|{}|{}", count, 100000000);
                   tx_send.unbounded_send(msgx).unwrap();
               }
           }
           Err(_err) => {
               errstring = "error reading mega ".to_string();
               errcd = 1;
               bolok = false;   
               break;
           }
        };
     }
     if bolok {       
         errstring = "number of rows has been set".to_string();
         let msgx = format!("Progress|{}|{}", 10, 10);
         tx_send.unbounded_send(msgx).unwrap();
     }
     Ok(Getrowsx {
            rowsnum: linenum,
            errcode: errcd,
            errval: errstring,
        })
    }
}


#[derive(Debug, Clone)]
struct Execx {
    errcolor: Color,
    errval: String,
}

impl Execx {
//    const TOTAL: u16 = 807;

    async fn execit(mega_value: String, rclone_value: String, rows_num: u64, utc_value: String) -> Result<Execx, Error> {
     let mut errstring  = " ".to_string();
     let mut colorx = Color::from([0.0, 0.0, 0.0]);
     let mut bolok = true;
     let mut numrow = 0;
     let mut numprocess = 0;
/*     let mergelistvec: Vec<&str> = mergescrol_value[0..].split("\n").collect();
     let mut lenmg1 = mergelistvec.len();
     lenmg1 = lenmg1 -1;
     for indl in 0..lenmg1 {
          let str_cur_dirfrom = dir_value.clone();
          let linestr = mergelistvec[indl].clone();
          let lineparse: Vec<&str> = linestr[0..].split(" | ").collect();
          let filefromx = lineparse[1].clone().to_string();
          let fullfrom = str_cur_dirfrom.clone() + "/" + &filefromx;
          if !Path::new(&fullfrom).exists() {
              errstring = format!("********* convert Copy: ERROR {} does not exist **********",fullfrom);
              colorx = Color::from([1.0, 0.0, 0.0]);
              bolok = false;
              break;
          }
          let str_cur_dirout = outdir_value.clone();
          let fileprex = lineparse[2].clone().to_string();
          let filetox = lineparse[3].clone().to_string();
          let fullto = str_cur_dirout.clone() + "/" + &fileprex + "_" + &filetox;
          if Path::new(&fullto).exists() {
              errstring = format!("********* convert Copy: ERROR {} already exists **********", fullto);
              colorx = Color::from([1.0, 0.0, 0.0]);
              bolok = false;
              break;
          }
          if numprocess < 4 {
              stdCommand::new("cp")
                           .arg("-p")
                           .arg(&fullfrom)
                           .arg(&fullto)
                           .spawn()
                           .expect("failed to execute process");
              numprocess = numprocess + 1;
          } else {
              let _output = stdCommand::new("cp")
                                         .arg("-p")
                                         .arg(&fullfrom)
                                         .arg(&fullto)
                                         .output()
                                         .expect("failed to execute process");
              numprocess = 0;
          }

//                                    println!("cp -p {} {}", fullfrom, fullto);

          numrow = numrow + 1;
     } */
     if bolok {
//         errstring = format!("convert copy copied {} files", lenmg1);
         colorx = Color::from([0.0, 0.0, 0.0]);
     }
     Ok(Execx {
            errcolor: colorx,
            errval: errstring,
        })
    }
}
#[derive(Debug, Clone)]
enum Error {
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
     sleep(Duration::from_secs(5));
     Ok(Progstart {
//            errcolor: colorx,
//            errval: errstring,
        })
    }
}
