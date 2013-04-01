// Copyright 2013 S. E. Steel
// As required by the original work, this work is licensed under a 
// Creative Commons Attribution-Noncommercial-Share Alike 2.5 Australia License.
// See http://creativecommons.org/licenses/by-nc-sa/2.5/au/ for details.

/*
random-search implements the random search algorithm as described in
"Clever Algorithms: Nature-Inspired Programming Recipes" by Jason Brownlee PhD.

For more details, see:
   http://www.cleveralgorithms.com/nature-inspired/stochastic/random_search.html
*/

package main

import (
	"flag"
	"fmt"
	"math/rand"
	"time"
)

type minmax [2]float64

type candidate struct {
	cost   float64
	vector minmax
}

func (c candidate) String() string {
	return fmt.Sprintf("C=%f, V=%v", c.cost, c.vector)
}

var problemSize int 
var min         int 
var max         int 
var iter        int

func init() {
	flag.IntVar(&problemSize, "p",    2, "problem size")
	flag.IntVar(&min,         "min", -5, "min")
	flag.IntVar(&max,         "max",  5, "max")
	flag.IntVar(&iter,        "i",  100, "maximum iterations")
}

func main() {
	flag.Parse()
	rand.Seed( time.Now().UTC().UnixNano() )
	fmt.Println(min, max)
	var searchSpace []minmax = make([]minmax, problemSize)
	for i:=0; i < problemSize; i++ {
		searchSpace[i][0] = float64(min)
		searchSpace[i][1] = float64(max)
	}
	best := search(searchSpace, iter)
	fmt.Println(best)
}

func search(searchSpace []minmax, iter int) (best candidate) {
	var candidate candidate
	firstIter:=true
	for i:=0; i < iter; i++ {
		candidate.vector = randomVector(searchSpace)
		candidate.cost   = objectiveFunc(candidate.vector)
		if firstIter {
			firstIter = false
			best = candidate
		} else if candidate.cost < best.cost {
			best = candidate
		}
		fmt.Println(" > iteration=",i+1,"best=",best.cost, "trial=", candidate.cost)
	}
	return
}

func randomVector(mm []minmax) minmax {
	var ret minmax
	for i:=0; i<len(mm); i++ {
		ret[i] = mm[i][0] + ((mm[i][1] - mm[i][0]) * rand.Float64())
	}
	return ret
}

func objectiveFunc(vector minmax) float64 {
	var sum float64
	for i:=0; i<len(vector); i++ {
		sum += vector[i]*vector[i]
	}
	return sum
}