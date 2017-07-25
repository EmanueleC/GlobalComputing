package main

import "fmt"

// Bulb type
type Bulb struct {
  on bool
  // (public) channels
  On chan int // switch it on
  Off chan int // switch it off
}

// Bulb behaviour
func (self *Bulb) Run () {
  for {
    if self.on {
      <- self.Off
      fmt.Printf("Light switched off \n")
      self.on = false;
    } else {
      <- self.On
      fmt.Printf("Light switched on \n")
      self.on = true;
    }
  }
}
