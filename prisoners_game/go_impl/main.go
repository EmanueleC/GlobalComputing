package main

func main() {
  var on = make(chan int)
  var off = make(chan int)
  var bulb Bulb = Bulb {
    true,
    on,
    off}

  go bulb.Run()

  for i := 0; i < 9; i++ {
    go Prisoner(i, off)
  }

  CounterPrisoner(10, on)

}
