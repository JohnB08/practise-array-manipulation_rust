





fn main() {
    println!("Hello, world!");
    let mut num_arr = [4,2,3,1,0];
    let string_arr = ["hi", ", ", "how ", "are ", "you?"];
    let sum = num_arr[0] + num_arr[1];
    println!("{}", sum);
    for mut element in string_arr{
        if element == "hi"{
            element = "Hey";
        }
        if element == ", "{
            element = "! "
        }
        println!("{}", element);
    };
    /* SORTS! */

    /* Bubble sort */
    /* Takes the value at the array location, and walks it in the dirrection you choose, in this case up.
     Stops for that index when swapped is still false at the end of the loop. */
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 0..num_arr.len()-1{
            if num_arr[i] > num_arr[i+1]{
                num_arr.swap(i, i+1);
                swapped = true;
            }
        }
    }
    for i in num_arr{
        println!("{}", i);
    }

    /* Insertion sort */

    /* Takes an element, and inserts it at a location deppending on size comparison. */

    for i in 1..num_arr.len(){
        let mut j = i;
        while j > 0 && num_arr[j-1]<num_arr[j]{
            num_arr.swap(j, j-1);
            j -= 1;
        }
    }
    for i in num_arr{
        println!("{}", i);
    }

    /* Selection sort */

    /* Finds minimum value, compares other indexes to minimum value. */

    for i in 0..num_arr.len(){
        /* We assume the first index is the smallest. */
        let mut min_i = i;
        /* Then we compare to elements further in than current minimum. */
        for j in (i+1)..num_arr.len(){
            if num_arr[j] < num_arr[min_i]{
                min_i = j;
            }
        }
        /* Swaps if we found a new minimum. */
        if min_i != i{
            num_arr.swap(i, min_i);
        }
    }
    for i in num_arr{
        println!("{}", i);
    }
    /* Sorts that are effective for bigger datasets: */

    /* Merge Sort */
    /* This takes an array and recursively splits, compares and merges. */
    /* Since it's a recursive function it's not included here. */

    /* Quick sort */
    /* This is another really fast recursive sort for big datasets. */
    /* It works by choosing a pivot point, and comparing the entries next to it, then incrementing the pivot point. */
    /* Since it's recursive and requires its own function, i'm not going to write it here. */
}
