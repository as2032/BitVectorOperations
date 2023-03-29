Alexander Straub 

Requirements (in cargo.toml):
```
bincode = "1.2.1"
bit-vec = { version = "0.6", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
get-size = { version = "^0.1", features = ["derive"] }
``` 


Basic input for Rank and Select operations.
The command line optionally takes 1 argument, an input file name. This file needs to be stored in the input_files/ directory. This input file must contain a sequence of 0's and 1's only. An example is found in input_files/inputexample. This input file is needed to be able to use Rankvec or Selectvec


This library consists of operations supporting Bit Vectors.

1) Rankvec

The Rankvec structure allows for constant time rank1 operations on the supplied bit vector. 

Use:
The code below assumes an already created Bit Vector (represented as bv here) of type BitVec from the bit-vec library. Interface for this structure is done in the main() function in main.rs
```
//Creating data for constant time rank operation
let rankbitvecdata = generate_rank_ds(bv.clone());
//Creating wrapper for the Bit Vector
let bitvecwrap = Bvec{
    bitvec:bv
};
//Creating Rankvec Structure
let rankbitvec = Rankvec{
    sbvec:bitvecwrap,
    data: Some(rankbitvecdata)
};
//Performing rank operation
rankbitvec.rank1(7);
```

2) Selectvec

The Selectvec structure allows for logrithmic time select1 operations on the supplied bit vector.

Use: 
The code below assumes an already created Bit Vector and Rankvec structure (represented as rankbitvec here). Interface for this structure is done in the main() function in main.rs

```
//Creating Selectvec Structure
let selectbitvec =  Selectvec{
    rankvec : rankbitvec
};
//Performing select1 operation
selectbitvec.select1(1);

//Performing rank1 operation through Selectvec
selectbitvec.rankvec.rank1(25);
```

3) Sparsevec
The Sparsevec structure allows for representation of strings using a BitVector. Unlike Rankvec and Selectvec, this structure does not rely on an already created bit vector, or input file, rather creates it itself. This structure also implements supporting operations on the sparse vector.

Use:

```
//Creating an empty sparse vector 
let mut sparsevec = Sparsevec::create(size);

//Adding elements to sparse vector at various positions
sparsevec.append("Rob".to_string(), 1);
sparsevec.append("Rob2".to_string(), 2);
sparsevec.append("Rob3".to_string(), 3);
//Finalizing the vector so that we can perform operations on it
sparsevec.finalize();
//Performing various operations on the sparsevec
sparsevec.get_index_of(2);
let mut elem= "".to_string();
sparsevec.get_at_rank(2, &mut elem);
// elem now obtains a reference to sparsevec rank 2 string

sparsevec.num_elem();
```

Sources:
bit-vec: https://crates.io/crates/bit-vec
serde: https://serde.rs/derive.html
get_size: https://crates.io/crates/get-size
rand: https://crates.io/crates/rand
bincode: https://docs.rs/bincode/latest/bincode/





