fn main() {
    // let input = [23, 82, 16, 45, 21, 94, 12, 34];
    let input: [i32;0] = [];
    // let mut minim = input[0];
    // let mut maxim = input[0];
    // for x in input {
    //     if(x > maxim){
    //         maxim = x
    //     }
    //     if(x < minim) {
    //         minim = x
    //     }
    // }
    let minim = input.iter().min().unwrap_or(&0);
    let maxim = input.iter().max().unwrap_or(&1000);
    println!("{} is largest and {} is smallest", maxim, minim);
}
