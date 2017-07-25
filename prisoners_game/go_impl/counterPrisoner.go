package main

import "fmt"
import "time"
import "math/rand"

func CounterPrisoner (n int, on chan int) {
  for i := 0; i < 2*(n-1); i++ {
    on <- 1
    fmt.Printf("CounterPrisoner switch the light on \n")
    time.Sleep(time.Duration(rand.Int63n(1e9)))
  }
  fmt.Printf("Freedom!")
}
