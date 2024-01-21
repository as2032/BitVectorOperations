# Bit Vector Operations Library

## Author: Alexander Straub

### Requirements (in cargo.toml):
```toml
bincode = "1.2.1"
bit-vec = { version = "0.6", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
get-size = { version = "^0.1", features = ["derive"] }
```
## Introduction
This library provides operations for working with Bit Vectors, including Rank and Select operations. The command-line interface optionally accepts an input file name, which should be stored in the input_files/ directory. The input file must contain a sequence of 0's and 1's only. Example input can be found in input_files/inputexample

## Library Operations
This library consists of operations supporting Bit Vectors.

### Rankvec
 
The Rankvec structure allows for constant time rank1 operations on the supplied bit vector.

#### Use:
The code below assumes an already created Bit Vector (represented as bv here) of type BitVec from the bit-vec library. Interface for this structure is done in the main() function in main.rs

Creating data for constant time rank operation
```
let rankbitvecdata = generate_rank_ds(bv.clone());
```
Creating wrapper for the Bit Vector
```
let bitvecwrap = Bvec{
    bitvec:bv
};
```
Creating Rankvec Structure
```
let mut rankbitvec = Rankvec{
    sbvec:bitvecwrap,
    data: Some(rankbitvecdata)
};
```
Performing rank operation
```
rankbitvec.rank1(7);
```
Getting memory overhead of Rank Data
```
rankbitvec.overhead();
```
Saving Rankvec structure to file 
```rankbitvec.save("RankVec.out");```
Loading Rankvec structure from file;
```rankbitvec.load("RankVec.out");```


### Selectvec

The Selectvec structure allows for logrithmic time select1 operations on the supplied bit vector.

#### Use: 
The code below assumes an already created Bit Vector and Rankvec structure (represented as rankbitvec here). Interface for this structure is done in the main() function in main.rs


Creating Selectvec Structure
```let mut selectbitvec =  Selectvec{
    rankvec : rankbitvec
};```
Performing select1 operation
```selectbitvec.select1(1);```
Getting memory overhead of Selectvec
```selectbitvec.overhead();```
Saving Selectvec Structure to file
```selectbitvec.save("SelectVec.out");```
Loading Selectvec Structure from file
```selectbitvec.load("SelectVec.out");```


### Sparsevec
The Sparsevec structure allows for representation of strings using a BitVector. Unlike Rankvec and Selectvec, this structure does not rely on an already created bit vector, or input file, rather creates it itself. This structure also implements supporting operations on the sparse vector.

#### Use:


Defining size of the sparse vector
```let size = 10;```

Creating an empty sparse vector 
```let mut sparsevec = Sparsevec::create(size);```

Adding elements to sparse vector at various positions
```sparsevec.append("Rob".to_string(), 1);
sparsevec.append("Rob2".to_string(), 2);
sparsevec.append("Rob3".to_string(), 3);```
Finalizing the vector so that we can perform operations on it
```sparsevec.finalize();```
Performing various operations on the sparsevec
```sparsevec.get_index_of(2);
let mut elem= "".to_string();
sparsevec.get_at_rank(2, &mut elem);```
* elem now obtains a reference to sparsevec rank 2 string
Getting the total number of elements in the bitvector
```sparsevec.num_elem();```
Getting number of elements up to index 5 inclusive
```sparsevec.num_elem_at(5);
let mut elem= "".to_string();```
Getting string representation of the Sparse Vec at index 3
```sparsevec.get_at_index(3, &mut elem);```
* elem now obtains a reference to sparsevec index 3 string

Saving Sparsevec to file
```sparsevec.save("Sparsevec.out".to_string());```
Loading Sparsevec from file
```sparsevec.load("Sparsevec.out".to_string());```

Getting the size of the sparse vector
```sparsevec.size();```


## Sources:
* bit-vec: https://crates.io/crates/bit-vec
* serde: https://serde.rs/derive.html
* get_size: https://crates.io/crates/get-size
* rand: https://crates.io/crates/rand
* bincode: https://docs.rs/bincode/latest/bincode/












