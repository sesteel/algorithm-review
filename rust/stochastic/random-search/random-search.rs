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
use core::rand::*;

// we are not using this, but this is how a struct looks
struct candidate  {
	cost: float,
	min:  float,
	max:  float
}

fn main() {	
 	static problemSize : int = 2; // constant int value just to show constants

 	let min         = -5.0;  // here we are using type inference, where rust assumes type float
 	let max : float =  5.0f; // here we are explicit
 	let iter        =  100i; 

 	// allocating a mutable 2D array on stack
 	let mut searchSpace: [[float, ..2], ..problemSize] = [[min, max], [min, max]];

 	// We must specify that the incrementor is mutable; everything 
 	// is immutable by default
 	let mut i = 0;
 	while i < problemSize {
 		searchSpace[i][0] = min;
 		searchSpace[i][1] = max;
 		i+=1;
 	} 

 	let result: (float, [float, ..2]) = search(searchSpace, iter);
 	
 	match result {
 		(best, vector) => io::println(~"C="+float::to_str(best)+~"  V=["+float::to_str(vector[0])+~", "+float::to_str(vector[1])+~"]")
 	}

}

// Search returns the tuple(best, vector)
fn search(searchSpace : [[float, ..2], ..2], iter: int) -> (float, [float, ..2]) {
	let mut vector : [float, ..2] = [0.0, 0.0];
	let mut best : float = -99999999.0;
	let mut first : bool = true;
	
	// for loop eq
	let mut i : int  = 0;
	while i < iter {

		vector = randomVector(searchSpace);
		let cost:float = objectiveFunc(vector);

		if first {
			first = false;
			best  = cost;
		} else if cost < best {
			best = cost;
		}
		
		io::println(~" > iteration=" + int::to_str(i) + ~" best="+ float::to_str(best) + ~" trial=" + float::to_str(cost));
		i+=1;
	}	
	
	return (best, vector);
}

fn randomVector(mm: [[float, ..2], ..2]) -> [float, ..2] {
	// create random number generator on the local heap
	let rand : @Rng = Rng();
	let mut ret : [float, ..2] = [0.0, 0.0];
	
	let mut i = 0;
	while i < mm.len() {
		ret[i] = mm[i][0] + ((mm[i][1] - mm[i][0]) * rand.gen_float());
		i+=1;
	} 
	
	return ret;
}


fn objectiveFunc(vector : [float, ..2]) -> float {
	let mut sum : float = 0.0;
	let mut i : uint = 0;
	while i < vector.len() {
		sum += vector[i] * vector[i];
		i+=1;
	}
	return sum;
}