package main

import "fmt"
import "time"
import "math/rand"

func Prisoner (id int, off chan int) {
  off <- 1
  fmt.Printf("Prisoner %d switched the light off once \n", id)
  time.Sleep(time.Duration(rand.Int63n(1e9)))
  off <- 1
  fmt.Printf("Prisoner %d switched the light off twice \n", id)
}
