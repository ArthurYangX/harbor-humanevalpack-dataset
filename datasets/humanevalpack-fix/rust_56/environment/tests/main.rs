#![allow(unused)]
use std::collections::*;
use std::cmp::*;
use std::io::*;
use std::str::*;
use std::any::Any;
use std::mem::replace;
use rand::prelude::*;
use regex::Regex;
use md5;


// Solution Code
/*
 brackets is a string of "<" and ">".
    return True if every opening bracket has a corresponding closing bracket.
    
*/
fn correct_bracketing(bkts:&str) -> bool{let mut level:i32=0;

    for i in 0..bkts.len(){

        if bkts.chars().nth(i).unwrap()== '<' {level+=1;}
        
        if bkts.chars().nth(i).unwrap() == '>' {  level-=1;}
      
        if level<0 {return false;} 
    }
    if level!=0 {return false;}
    return true;
}

// Test Code

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_bracketing() {
        assert!(correct_bracketing("<>"));
        assert!(correct_bracketing("<<><>>"));
        assert!(correct_bracketing("<><><<><>><>"));
        assert!(correct_bracketing("<><><<<><><>><>><<><><<>>>"));
        assert!(!(correct_bracketing("<<<><>>>>")));
        assert!(!(correct_bracketing("><<>")));
        assert!(!(correct_bracketing("<")));
        assert!(!(correct_bracketing("<<<<")));
        assert!(!(correct_bracketing(">")));
        assert!(!(correct_bracketing("<<>")));
        assert!(!(correct_bracketing("<><><<><>><>><<>")));
        assert!(!(correct_bracketing("<><><<><>><>>><>")));
    }

}


// Binary entry point (required for cargo test)
fn main() {}
