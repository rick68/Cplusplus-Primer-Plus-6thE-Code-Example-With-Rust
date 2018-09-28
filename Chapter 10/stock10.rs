// stock00.rs -- implementing sth Stock struct

use std::io;
use std::io::prelude::*;
use std::default::Default;
use std::ptr;
use std::ops::Drop;
use std::str;
use std::cmp;

#[derive(Clone)]
pub struct Stock
{
    company: [u8; 30],
    shares: i32,
    share_val: f64,
    total_val: f64,
}

impl Default for Stock {
    fn default() -> Self {
        let mut stdout: io::Stdout = io::stdout();
        write!(stdout, "Default constructor called\n");
        Stock {
            company: {
                let mut buf: [u8; 30] = [0; 30];
                let name: &str = "no name";
                unsafe {
                    ptr::copy_nonoverlapping(name.as_ptr(), buf.as_mut_ptr(), name.len());
                }
                buf
            },
            shares: 0,
            share_val: 0.0,
            total_val: 0.0,
        }
    }
}

impl Drop for Stock {
    fn drop(&mut self) {
        let mut stdout: io::Stdout = io::stdout();
        write!(stdout, "Bye, {}!\n", str::from_utf8(&self.company).unwrap());
    }
}


impl Stock
{
    fn set_tot(&mut self) {
        self.total_val = self.share_val;
    }

    pub fn new(co: &str, n: i32, pr: f64) -> Self {
        let mut stdout: io::Stdout = io::stdout();

        let mut obj: Stock = Stock::default();

        write!(stdout, "Constructor using {} called\n", co);
        unsafe {
            ptr::copy_nonoverlapping(co.as_ptr(), obj.company.as_mut_ptr(), cmp::min(obj.company.len(), co.len()));
        }
        if n < 0 {
            let _: usize = stdout.write(b"Number of shares can't be negative; ").unwrap();
            write!(stdout, "{} shares set to 0.\n", str::from_utf8(&obj.company).unwrap());
            obj.shares = 0;
        } else {
            obj.shares = n;
        }
        obj.share_val = pr;
        obj.set_tot();
        obj
    }

    pub fn buy(&mut self, num: i32, price: f64) {
        let mut stdout: io::Stdout = io::stdout();

        if num < 0 {
            let _: usize = stdout.write(b"Number of shares purchased can't be negative; ").unwrap();
            let _: usize = stdout.write(b"Transaction is aborted.\n").unwrap();
        } else {
            self.shares += num;
            self.share_val = price;
            self.set_tot();
        }
    }

    pub fn sell(&mut self, num: i32, price: f64) {
        let mut stdout: io::Stdout = io::stdout();

        if num < 0 {
            let _: usize = stdout.write(b"Number of shares sold can't be negative. ").unwrap();
            let _: usize = stdout.write(b"Transaction is aborted.\n").unwrap();
        } else if num > self.shares {
            let _: usize = stdout.write(b"You can't sell more than you have! ").unwrap();
            let _: usize = stdout.write(b"Transaction is aborted.\n").unwrap();
        } else {
            self.shares -= num;
            self.share_val = price;
            self.set_tot();
        }
    }

    pub fn update(&mut self, price: f64) {
        self.share_val = price;
        self.set_tot();
    }

    pub fn show(&self) {
        let mut stdout: io::Stdout = io::stdout();

        write!(stdout, "Company: {}\n", str::from_utf8(&self.company).unwrap());
        write!(stdout, "  Shares: {}\n", self.shares);
        write!(stdout, "  Share Price: ${:.*}\n", 3, self.share_val);
        write!(stdout, "  Total Worth: ${:.*}\n", 2, self.total_val);
    }
}
