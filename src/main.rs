use std::sync::Arc;
use get_size::GetSize;
use rand::Rng;
use std::env;
use std::collections::HashMap;
use bincode;
use bit_vec::BitVec;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use std::time::{Instant};
use serde::{Serialize, Deserialize};

fn get_full_checkpoints(bitvec: BitVec, low:usize, high:usize) ->Vec<usize>{
    let mut checkpoints = Vec::new();
    let n  = bitvec.len();
    let schunk = (n as f32).log2().powf(2.0).ceil() as usize;
    let mut curcount = 0;
    let mut cchunk = 1;
    checkpoints.push(0);
    if n>0{
        for i in low..high{
            if bitvec[i]==true{
                curcount+=1;
            }
            if (i-low)==schunk*cchunk{
                checkpoints.push(curcount);
                cchunk+=1;
                curcount = 0;
            }
        }
    }
    checkpoints
}
fn get_chunk_checkpoints(bitvec: BitVec, low:usize, high:usize) ->(Vec<usize>, Vec<Vec<usize>>){
    let mut checkpoints = Vec::new();
    let mut subcheckpoints = Vec::new();
    let mut high = high;
    let sschunk = ((bitvec.len() as f32).log2()/2_f32).ceil()  as usize;
    let mut curcount = 0;
    let mut subchunkcount = 0;
    let mut subchunkc = 1;
    checkpoints.push(0);
    if high>bitvec.len(){
        high = bitvec.len();
    }
    let mut ssvec = Vec::new();
    for i in low..high{

        if (i-low) == sschunk*subchunkc{
            subchunkcount = 0;
            subcheckpoints.push(ssvec);
            checkpoints.push(curcount);
            ssvec = Vec::new();
            subchunkc+=1;
        }
        ssvec.push(subchunkcount);
        if bitvec[i]==true{
            curcount+=1;
            subchunkcount+=1;
        }

    }
    subcheckpoints.push(ssvec);
    (checkpoints, subcheckpoints)
}

#[derive(Serialize, Deserialize, Clone)]
struct Bvec{
    bitvec: BitVec
}
#[derive(Serialize, Deserialize)]
struct Rankvec{
    sbvec:Bvec,
    data: Option<Rankds>
    // rankhelp: Vec<usize>}
}
#[derive(Serialize, Deserialize)]
struct Selectvec{
    // sbvec: Bvec,
    rankvec:Rankvec,
}
#[derive(Serialize, Deserialize, GetSize)]
struct Rankds{
    checkpoints: Vec<usize>,
    chunk_checkpoints: Vec<Vec<usize>>,
    chunk_chunk_checkpoints: Vec<Vec<Vec<usize>>>
}

trait Bvecops{
    fn new ()-> Bvec;
    fn save(&self, fname: &str);
    fn load(&mut self, fname: &str);
}

impl Bvecops for Bvec{
    fn new()->Bvec{
        Bvec{
            bitvec: BitVec::new()
        }
    }
    fn save(&self, fname: &str){
        let file = File::create(fname).unwrap();  
        let mut binwriter = BufWriter::new(file);
        let _res = bincode::serialize_into(&mut binwriter, &self.bitvec);
    }
    fn load(&mut self, fname: &str){
        let file = File::open(fname).unwrap(); 
        let binreader = BufReader::new(file);
        let rvload: BitVec = bincode::deserialize_from(binreader).unwrap();
        self.bitvec = rvload;
    }
}


trait Rvops{
    fn rank1(&self, i:usize) -> usize;
    fn overhead(&self) -> usize;
    fn save(&self, fname: &str);
    fn load(&mut self, fname: &str);
}

impl Rvops for Rankvec{
    fn rank1(&self, i:usize) -> usize {
        let chunknum = i/((self.sbvec.bitvec.len() as f32).log2().powf(2.0).ceil() as usize);
        let cp_outer = self.data.as_ref().unwrap().checkpoints[chunknum];
        let chunklow = ((chunknum as f32)*(self.sbvec.bitvec.len() as f32).log2().powf(2.0).ceil()) as usize;
        let reli = i-chunklow;
        let subchunknum = reli/(((self.sbvec.bitvec.len() as f32).log2()/2_f32).ceil() as usize);
        let c2 = self.data.as_ref().unwrap().chunk_checkpoints[chunknum][subchunknum];
        let relisubchunk = i-(chunklow+(subchunknum*((self.sbvec.bitvec.len() as f32).log2()/2_f32).ceil() as usize));
        let c3 = self.data.as_ref().unwrap().chunk_chunk_checkpoints[chunknum][subchunknum][relisubchunk];
        cp_outer+c2+c3
    }
    fn overhead(&self) -> usize {
        let size = self.data.as_ref().unwrap().get_heap_size();
        size
    }
    fn save(&self, fname: &str){
        let file = File::create(fname).unwrap();  
        let mut binwriter = BufWriter::new(file);
        let _res = bincode::serialize_into(&mut binwriter, &self);
    }
    fn load(&mut self, fname: &str){
        let file = File::open(fname).unwrap();  
        let binreader = BufReader::new(file);
        let rvload: Rankvec = bincode::deserialize_from(binreader).unwrap();
        self.data = rvload.data;
        self.sbvec = rvload.sbvec;
    }
}
trait Selops{
    fn select1(&self, i: usize)-> usize;
    fn overhead(&self)->usize;
    fn save(&self, fname: &str);
    fn load(&mut self, fname:&str);
}
impl Selops for Selectvec{
    fn select1(&self, i: usize)-> usize{
        let mut l = 0;
        let mut h = self.rankvec.sbvec.bitvec.len();
        while l<h{
            let m = (h+l)/2;
            // dbg!(m);
            if self.rankvec.rank1(m)>i{
                h = m;
            }else if self.rankvec.rank1(m)<i {
                l = m+1;
            }else{
                if self.rankvec.sbvec.bitvec[m]==true{
                    return m;
                }else{
                    let mut j = m;
                    while j<self.rankvec.sbvec.bitvec.len(){
                        if self.rankvec.sbvec.bitvec[j]==true{
                            return j;
                        }else{
                            j+=1;
                        }
                    }
                }
                
            }
        }
        self.rankvec.sbvec.bitvec.len()
    }
    fn overhead(&self)->usize{
        self.rankvec.data.as_ref().unwrap().get_heap_size()
    }
    fn save(&self, fname: &str){
        let file = File::create(fname).unwrap();  
        let mut binwriter = BufWriter::new(file);
        let _res = bincode::serialize_into(&mut binwriter, &self);
    }
    fn load(&mut self, fname:&str){
        let file = File::open(fname).unwrap();  
        let binreader = BufReader::new(file);
        let svload: Selectvec = bincode::deserialize_from(binreader).unwrap();
        self.rankvec = svload.rankvec;
    }
}



// #[derive(Serialize, Deserialize, GetSize)]
#[derive(Serialize, Deserialize)]
struct Sparsevec{
    help: Option<Selectvec>,
    // rankHelp: Option<Rankvec>,
    hashtrack: HashMap<usize, Option<String>>
}

trait Sparseops{
    fn create(i: usize) -> Self;
    fn append(& mut self, elem: String, pos: usize);
    fn finalize(& mut self);
    fn get_at_rank(&self, r: usize, elem: &mut String)->bool;
    fn get_at_index(&self, r:usize, elem: &mut String)->bool;
    fn get_index_of(&self, r:usize)->usize;
    fn num_elem_at(&self, r:usize)->usize;
    fn size(&self)->usize;
    fn num_elem(&self)->usize;
    fn save(&self, fname: String);
    fn load(& mut self, fname: String);
}

impl Sparseops for Sparsevec{
    fn create(i: usize) -> Self{

        let a= Some(Rankvec{sbvec: Bvec{bitvec: BitVec::from_elem(i,false)}, data:None});
        let b = Some(Selectvec{rankvec: a.unwrap()});
        let c = Sparsevec{help:b, hashtrack:HashMap::new()};
        return c;
    }
    fn append(& mut self, elem: String, pos: usize){ 
        self.hashtrack.insert(pos, Some(elem),);
        self.help.as_mut().unwrap().rankvec.sbvec.bitvec.set(pos, true);
    }
    fn finalize(&mut self){
        self.help.as_mut().unwrap().rankvec.data = Some(generate_rank_ds(self.help.as_mut().unwrap().rankvec.sbvec.bitvec.clone()));
    }
    fn get_at_rank(&self, r: usize, elem:  &mut String)->bool{
        let pos = self.help.as_ref().unwrap().select1(r);
        if !self.hashtrack.contains_key(&pos){
            return false
        }else{
            *elem = self.hashtrack.get(&pos).unwrap().clone().unwrap().to_string();
            return true;
        }
    }
    fn get_at_index(&self, r:usize, elem: &mut String)->bool{
        if self.help.as_ref().unwrap().rankvec.sbvec.bitvec[r]{
            *elem = self.hashtrack.get(&r).unwrap().clone().unwrap().to_string();
            return true;
        }else{
            return false;
        }
       
    }
    fn get_index_of(&self, r:usize)->usize{
        if r<self.help.as_ref().unwrap().rankvec.sbvec.bitvec.len(){
            self.help.as_ref().unwrap().select1(r-1)
        }else{
            return usize::MAX
        }
    }
    fn num_elem_at(&self, r:usize)->usize{
        self.help.as_ref().unwrap().rankvec.rank1(r+1)
    }
    fn size(&self)->usize{
        let size = self.hashtrack.get_heap_size()+self.help.as_ref().unwrap().rankvec.data.get_heap_size();
        size
    }
    fn num_elem(&self)->usize{
        self.hashtrack.keys().len()
    }
    fn save(&self, fname: String){
        let file = File::create(fname).unwrap();  
        let mut binwriter = BufWriter::new(file);
        let _res = bincode::serialize_into(&mut binwriter, &self);
    }
    fn load(& mut self, fname: String){
        let file = File::open(fname).unwrap();  
        let binreader = BufReader::new(file);
        let svload: Sparsevec = bincode::deserialize_from(binreader).unwrap();
        self.help = svload.help;
        self.hashtrack = svload.hashtrack;
    }
}

fn generate_rank_ds(bvec: BitVec)->Rankds{
    let mut cp_chunks = Vec::new();
    let mut cp_cp_table = Vec::new();
    let checkpoints_full = get_full_checkpoints(bvec.clone(),0,bvec.len());
    let mut low = 0;
    let mut high:usize= (bvec.len() as f32).log2().powf(2.0).ceil() as usize;
    let topcheckpoint = checkpoints_full.len()*high+high;
    let mut numcheckpoints = checkpoints_full.len();
    if topcheckpoint<bvec.len(){
        numcheckpoints+=1;
    }
    for _i in 0..numcheckpoints{
        let (cp_chunk, cp_table) = get_chunk_checkpoints(bvec.clone(), low, high);
        if cp_table.len()>0{
            cp_cp_table.push(cp_table);
        }
        if cp_chunk.len()>0{
            cp_chunks.push(cp_chunk);
        }
        low+= (bvec.len() as f32).log2().powf(2.0).ceil() as usize;
        high+= (bvec.len() as f32).log2().powf(2.0).ceil() as usize;
    }
    let rankbvdata = Rankds{
        checkpoints: checkpoints_full,
        chunk_checkpoints:cp_chunks,
        chunk_chunk_checkpoints: cp_cp_table
    };
    rankbvdata
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut bv = BitVec::new();
    if args.len()>0{
        let fname = &args[1];
        let mut full_file_path = "input_files/".to_string();
        full_file_path.push_str(fname);
        let input_file = File::open(full_file_path).unwrap();
        let input_reader = BufReader::new(input_file);
        let mut bvstring = String::new();
        for line in input_reader.lines() {
            let line = line.unwrap();
            bvstring.push_str(&line);
        }

        let bv_size = bvstring.len();
        bv = BitVec::from_elem(bv_size, false);
        let charvec: Vec<char> = bvstring.chars().collect();
        for i in 0..bv_size{
            if charvec[i]=='1'{
                bv.set(i, true);
            }
        }
    }
    
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

    //Creating Selectvec Structure
    let selectbitvec =  Selectvec{
        rankvec : rankbitvec
    };
    //Performing select1 operation
    selectbitvec.select1(1);

    //Defining size of the sparse vector
    let size = 10;

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

// VERBOSE TESTING OF SPARSEVEC WITH VARYING INPUT SIZE AND SPARSITY
    
    //Performing rank1 operation through Selectvec
    // selectbitvec.rankvec.rank1(25);
    // let currentrank = Instant::now();
    // selectbitvec.rankvec.rank1(40);
    // selectbitvec.rankvec.rank1(25);
    // selectbitvec.rankvec.rank1(35);
    // selectbitvec.rankvec.rank1(1);
    // selectbitvec.rankvec.rank1(3);
    // selectbitvec.rankvec.rank1(7);
    // selectbitvec.rankvec.rank1(5);
    // selectbitvec.rankvec.rank1(2);
    // selectbitvec.rankvec.rank1(9);
    // selectbitvec.rankvec.rank1(12);
    // let duration = currentrank.elapsed();
    // println!("Time Elapsed performing rank ops is {:?}", duration);
    // dbg!(selectbitvec.overhead());
    // let currentsel = Instant::now();
    // selectbitvec.select1(60);
    // selectbitvec.select1(25);
    // selectbitvec.select1(35);
    // selectbitvec.select1(1);
    // selectbitvec.select1(3);
    // selectbitvec.select1(7);
    // selectbitvec.select1(5);
    // selectbitvec.select1(2);
    // selectbitvec.select1(9);
    // selectbitvec.select1(12);
    // let duration = currentsel.elapsed();
    // println!("Time Elapsed performing select ops is {:?}", duration);
    // println!("N=1000, 1%");
    // let size = 1000;
    // let mut vec1 = Sparsevec::create(size);
    // use rand::{thread_rng, Rng};
    // let mut rng = thread_rng();
    // for i in 0..size{
    //     let bool0 = rng.gen_bool(0.01);
    //     if !bool0{
    //         vec1.append("yo".to_string(), i)
    //     }
    // }
    // let curfinalize = Instant::now();
    // vec1.finalize();
    // let mut s1= "".to_string();
    // let durfinalize = curfinalize.elapsed();
    // let curgar = Instant::now();
    // vec1.get_at_rank(16, &mut s1);
    // let durgar = curgar.elapsed();
    // let curgio = Instant::now();
    // vec1.get_index_of(615);
    // let durgio = curgio.elapsed();
    // let curnea = Instant::now();
    // vec1.num_elem_at(280);
    // let durnea = curnea.elapsed();
    // dbg!(durfinalize, durgar, durgio, durnea);

    // println!("N=1000, 5%");
    // let size = 1000;
    // let mut vec1 = Sparsevec::create(size);
    // let mut rng = thread_rng();
    // for i in 0..size{
    //     let bool0 = rng.gen_bool(0.05);
    //     if !bool0{
    //         vec1.append("yo".to_string(), i)
    //     }
    // }
    // let curfinalize = Instant::now();
    // vec1.finalize();
    // let mut s1= "".to_string();
    // let durfinalize = curfinalize.elapsed();
    // let curgar = Instant::now();
    // vec1.get_at_rank(16, &mut s1);
    // let durgar = curgar.elapsed();
    // let curgio = Instant::now();
    // vec1.get_index_of(615);
    // let durgio = curgio.elapsed();
    // let curnea = Instant::now();
    // vec1.num_elem_at(280);
    // let durnea = curnea.elapsed();
    // dbg!(durfinalize, durgar, durgio, durnea);

    // println!("N=1000, 10%");
    // let size = 1000;
    // let mut vec1 = Sparsevec::create(size);
    // let mut rng = thread_rng();
    // for i in 0..size{
    //     let bool0 = rng.gen_bool(0.1);
    //     if !bool0{
    //         vec1.append("yo".to_string(), i)
    //     }
    // }
    // let curfinalize = Instant::now();
    // vec1.finalize();
    // let mut s1= "".to_string();
    // let durfinalize = curfinalize.elapsed();
    // let curgar = Instant::now();
    // vec1.get_at_rank(16, &mut s1);
    // let durgar = curgar.elapsed();
    // let curgio = Instant::now();
    // vec1.get_index_of(615);
    // let durgio = curgio.elapsed();
    // let curnea = Instant::now();
    // vec1.num_elem_at(280);
    // let durnea = curnea.elapsed();
    // dbg!(durfinalize, durgar, durgio, durnea);


    // println!("N=10000, 1%");
    // let size = 10000;
    // let mut vec1 = Sparsevec::create(size);
    // let mut rng = thread_rng();
    // for i in 0..size{
    //     let bool0 = rng.gen_bool(0.01);
    //     if !bool0{
    //         vec1.append("yo".to_string(), i)
    //     }
    // }
    // dbg!("b4finalize");
    // let curfinalize = Instant::now();
    // vec1.finalize();
    // let mut s1= "".to_string();
    // let durfinalize = curfinalize.elapsed();
    // let curgar = Instant::now();
    // vec1.get_at_rank(16, &mut s1);
    // let durgar = curgar.elapsed();
    // let curgio = Instant::now();
    // vec1.get_index_of(615);
    // let durgio = curgio.elapsed();
    // let curnea = Instant::now();
    // vec1.num_elem_at(280);
    // let durnea = curnea.elapsed();
    // dbg!(durfinalize, durgar, durgio, durnea);

    // println!("N=10000, 5%");
    // let size = 10000;
    // let mut vec1 = Sparsevec::create(size);
    // let mut rng = thread_rng();
    // for i in 0..size{
    //     let bool0 = rng.gen_bool(0.05);
    //     if !bool0{
    //         vec1.append("yo".to_string(), i)
    //     }
    // }
    // let curfinalize = Instant::now();
    // vec1.finalize();
    // let mut s1= "".to_string();
    // let durfinalize = curfinalize.elapsed();
    // let curgar = Instant::now();
    // vec1.get_at_rank(16, &mut s1);
    // let durgar = curgar.elapsed();
    // let curgio = Instant::now();
    // vec1.get_index_of(615);
    // let durgio = curgio.elapsed();
    // let curnea = Instant::now();
    // vec1.num_elem_at(280);
    // let durnea = curnea.elapsed();
    // dbg!(durfinalize, durgar, durgio, durnea);

    // println!("N=10000, 10%");
    // let size = 10000;
    // let mut vec1 = Sparsevec::create(size);
    // let mut rng = thread_rng();
    // for i in 0..size{
    //     let bool0 = rng.gen_bool(0.1);
    //     if !bool0{
    //         vec1.append("yo".to_string(), i)
    //     }
    // }
    // let curfinalize = Instant::now();
    // vec1.finalize();
    // let mut s1= "".to_string();
    // let durfinalize = curfinalize.elapsed();
    // let curgar = Instant::now();
    // vec1.get_at_rank(16, &mut s1);
    // let durgar = curgar.elapsed();
    // let curgio = Instant::now();
    // vec1.get_index_of(615);
    // let durgio = curgio.elapsed();
    // let curnea = Instant::now();
    // vec1.num_elem_at(280);
    // let durnea = curnea.elapsed();
    // dbg!(durfinalize, durgar, durgio, durnea);


    // println!("N=100000, 1%");
    // let size = 100000;
    // let mut vec1 = Sparsevec::create(size);
    // let mut rng = thread_rng();
    // for i in 0..size{
    //     let bool0 = rng.gen_bool(0.01);
    //     if !bool0{
    //         vec1.append("yo".to_string(), i)
    //     }
    // }
    // let curfinalize = Instant::now();
    // vec1.finalize();
    // let mut s1= "".to_string();
    // let durfinalize = curfinalize.elapsed();
    // let curgar = Instant::now();
    // vec1.get_at_rank(16, &mut s1);
    // let durgar = curgar.elapsed();
    // let curgio = Instant::now();
    // vec1.get_index_of(615);
    // let durgio = curgio.elapsed();
    // let curnea = Instant::now();
    // vec1.num_elem_at(280);
    // let durnea = curnea.elapsed();
    // dbg!(durfinalize, durgar, durgio, durnea);

    // println!("N=100000, 5%");
    // let size = 100000;
    // let mut vec1 = Sparsevec::create(size);
    // let mut rng = thread_rng();
    // for i in 0..size{
    //     let bool0 = rng.gen_bool(0.05);
    //     if !bool0{
    //         vec1.append("yo".to_string(), i)
    //     }
    // }
    // let curfinalize = Instant::now();
    // vec1.finalize();
    // let mut s1= "".to_string();
    // let durfinalize = curfinalize.elapsed();
    // let curgar = Instant::now();
    // vec1.get_at_rank(16, &mut s1);
    // let durgar = curgar.elapsed();
    // let curgio = Instant::now();
    // vec1.get_index_of(615);
    // let durgio = curgio.elapsed();
    // let curnea = Instant::now();
    // vec1.num_elem_at(280);
    // let durnea = curnea.elapsed();
    // dbg!(durfinalize, durgar, durgio, durnea);

    // println!("N=100000, 10%");
    // let size = 100000;
    // let mut vec1 = Sparsevec::create(size);
    // let mut rng = thread_rng();
    // for i in 0..size{
    //     let bool0 = rng.gen_bool(0.1);
    //     if !bool0{
    //         vec1.append("yo".to_string(), i)
    //     }
    // }
    // let curfinalize = Instant::now();
    // vec1.finalize();
    // let mut s1= "".to_string();
    // let durfinalize = curfinalize.elapsed();
    // let curgar = Instant::now();
    // vec1.get_at_rank(16, &mut s1);
    // let durgar = curgar.elapsed();
    // let curgio = Instant::now();
    // vec1.get_index_of(615);
    // let durgio = curgio.elapsed();
    // let curnea = Instant::now();
    // vec1.num_elem_at(280);
    // let durnea = curnea.elapsed();
    // dbg!(durfinalize, durgar, durgio, durnea);

    // println!("N=1000000, 1%");
    // let size = 1000000;
    // let mut vec1 = Sparsevec::create(size);
    // let mut rng = thread_rng();
    // for i in 0..size{
    //     let bool0 = rng.gen_bool(0.01);
    //     if !bool0{
    //         vec1.append("yo".to_string(), i)
    //     }
    // }
    // let curfinalize = Instant::now();
    // vec1.finalize();
    // let mut s1= "".to_string();
    // let durfinalize = curfinalize.elapsed();
    // let curgar = Instant::now();
    // vec1.get_at_rank(16, &mut s1);
    // let durgar = curgar.elapsed();
    // let curgio = Instant::now();
    // vec1.get_index_of(615);
    // let durgio = curgio.elapsed();
    // let curnea = Instant::now();
    // vec1.num_elem_at(280);
    // let durnea = curnea.elapsed();
    // dbg!(durfinalize, durgar, durgio, durnea);

    // println!("N=1000000, 5%");
    // let size = 1000000;
    // let mut vec1 = Sparsevec::create(size);
    // let mut rng = thread_rng();
    // for i in 0..size{
    //     let bool0 = rng.gen_bool(0.05);
    //     if !bool0{
    //         vec1.append("yo".to_string(), i)
    //     }
    // }
    // let curfinalize = Instant::now();
    // vec1.finalize();
    // let mut s1= "".to_string();
    // let durfinalize = curfinalize.elapsed();
    // let curgar = Instant::now();
    // vec1.get_at_rank(16, &mut s1);
    // let durgar = curgar.elapsed();
    // let curgio = Instant::now();
    // vec1.get_index_of(615);
    // let durgio = curgio.elapsed();
    // let curnea = Instant::now();
    // vec1.num_elem_at(280);
    // let durnea = curnea.elapsed();
    // dbg!(durfinalize, durgar, durgio, durnea);

    // println!("N=1000000, 10%");
    // let size = 1000000;
    // let mut vec1 = Sparsevec::create(size);
    // let mut rng = thread_rng();
    // for i in 0..size{
    //     let bool0 = rng.gen_bool(0.1);
    //     if !bool0{
    //         vec1.append("yo".to_string(), i)
    //     }
    // }
    // let curfinalize = Instant::now();
    // vec1.finalize();
    // let mut s1= "".to_string();
    // let durfinalize = curfinalize.elapsed();
    // let curgar = Instant::now();
    // vec1.get_at_rank(16, &mut s1);
    // let durgar = curgar.elapsed();
    // let curgio = Instant::now();
    // vec1.get_index_of(615);
    // let durgio = curgio.elapsed();
    // let curnea = Instant::now();
    // vec1.num_elem_at(280);
    // let durnea = curnea.elapsed();
    // dbg!(durfinalize, durgar, durgio, durnea);




























    // vec1.append("yo".to_string(), 3);
    // vec1.append("yo2".to_string(), 1);
    // vec1.finalize();
    // dbg!(vec1.sparsevec.clone());
    // dbg!(vec1.get_at_rank(1, &mut s1));
    // dbg!(s1);
    // dbg!(vec1.num_elem());
    // // let b1 = vec1.get_at_index(1, &mut "".to_string());
    // dbg!(vec1.get_index_of(2));


    




    println!("Hello, world!");

}



