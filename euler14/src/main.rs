use std::fmt;

struct SequenceItem{
    number:u64,
    iteration:u32,
}

impl fmt::Display for SequenceItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " Sequence Item {}/{}", self.number,self.iteration)
    }
}

fn is_odd(number: u64) -> bool{
    return number.rem_euclid(2)==0;
}

fn apply_sequence(item: SequenceItem ) -> SequenceItem {
    if item.number >1 {
        if is_odd(item.number){
            let new_item = SequenceItem {number: item.number /2, iteration: item.iteration +1};
            return apply_sequence(new_item);
        }else{
            let new_item = SequenceItem {number: 3 *  item.number +1, iteration: item.iteration +1};
            return apply_sequence(new_item);
        }
    }else {
        return item;
    }
}




fn main() {
    let mut max= SequenceItem{number:0,iteration:0};
    let mut max_index = 1;
    for i in 1..1_000_000{
        let item = apply_sequence(SequenceItem { number: i, iteration: 1 });
        if item.iteration>max.iteration {
            max = SequenceItem{number:item.number,iteration:item.iteration};
            max_index = i;
        }
    }
    let max_number= max.number;
    let max_iteration = max.iteration;
    println!(" Max Sequence: {max_index:?} / {max_number:?} / {max_iteration:?} ");
}
