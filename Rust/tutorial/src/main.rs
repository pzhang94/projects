#[allow(dead_code)]
#[allow(unused_variables)]

extern crate phrases;

mod stack_and_heap;
mod pattern_matching;
use std::mem;
use rand::Rng;
use std::io::stdin;
use std::collections::HashMap;
use std::collections::HashSet;
use std::thread;
use std::time;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Neg};
use std::cmp::PartialEq;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;

const MEANING_OF_LIFE: u8 = 42; // no fixed address

static mut Z: i32 = 123;    // static variables are held in a precise memory location in program
                            // all references to the static refer to the same memory location

fn core_data_types() 
{
    let a: u8 = 123;
    println!("a = {}", a);
    
    let mut b: i8 = 0;
    println!("b = {} before", b);
    b = 42;
    println!("b = {} after", b);

    let mut c: i32 = 123456789;
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}", c);

    let z: isize = 123;
    let size_of_z: usize = mem::size_of_val(&z);
    println!("z = {}, takes up {}, {}-bit OS", z, size_of_z, size_of_z * 8);

    let d: char = 'x';
    println!("{} is a char, size = {} bytes", d, mem::size_of_val(&d));

    let e: f64 = 2.5;
    println!("{}, size = {} bytes", e, mem::size_of_val(&e));

    let g: bool = false;
    println!("{}, size = {} bytes", g, mem::size_of_val(&g));
}

fn operators()
{
    // arithmetic
    let mut a: i32 = 2 + 3 * 4;
    println!("{}", a);

    a = a + 1;
    a -= 2;
    println!("remainder of {} / {} = {}", a, 3, a % 3);

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    // bitwise
    let c = 1 | 2;
    println!("1 | 2 = {}", c);

    let two_to_10 = 1 << 10;
    println!("2^10 = {}", two_to_10);

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0;
    let x = 5;
    let x_is_5 = x == 5;
}

fn declaring_and_using_constants()
{
    println!("The meaning of life is {} with address {}", MEANING_OF_LIFE, &MEANING_OF_LIFE);
    unsafe  // mentions that this is an unsafe usage and allows Z to be mutable
            // this is unsafe since Z is static and mutable (not copied inline but available to all to use/update)
    {
        println!("Z = {} with address {}", Z, &Z);
    }
}

fn if_statement()
{
    let temp = 5;
    if temp > 30
    {
        println!("really hot outside!");
    }
    else if temp < 10
    {
        println!("really cold!");
    }
    else
    {
        println!("temperature is OK");
    } 

    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("today is {}", day);

    println!("it is {}", 
        if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"OK"});

    println!("it is {}",
        if temp > 20 {
            if temp > 30 {"very hot"} else {"hot"}
        } else if temp < 10 {"cold"} else {"OK"});
}

fn while_and_loop()
{
    let mut x = 1;
    while x < 1000
    {
        x *= 2;

        if x == 64 {continue};

        println!("x = {}", x);
    }

    let mut y = 1;
    loop    // while true
    {
        y *= 2;
        println!("y = {}", y);

        if y == 1 << 10 {break;}
    }
}

fn for_loop()
{
    for x in 1..11  // stop at 11: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
    {
        if x == 3 {continue;}

        if x == 8 {break;}

        println!("x = {}", x);
    }

    for (pos, y) in (30..41).enumerate()
    {
        println!("{}: {}", pos, y);
    }
}

fn match_statement()
{
    let country_code = 44;

    let country = match country_code {  // match must cover all cases
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "unknown", // ..= is an inclusive range (includes 1000)
        _ => "invalid" // _ is a catch-all (like a default case)
    };

    println!("the country code with code {} is {}", country_code, country);

    let x = false;
    let s = match x {
        true => "yes",
        false => "no"
    };
}

enum State
{
    Locked,
    Failed,
    Unlocked
}

fn combination_lock()
{
    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();

    loop 
    {
        match state 
        {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input)
                {
                    Ok(_) => {
                        entry.push_str(&input.trim_end());

                    }
                    Err(_) => continue                    
                }

                if entry == code 
                {
                    state = State::Unlocked;
                    continue;
                }
                
                if !code.starts_with(&entry)
                {
                    state = State::Failed;
                }
            }
            State::Failed => {
                println!("FAILED");
                entry.clear();
                state = State::Locked;
            }
            State::Unlocked => {
                println!("UNLOCKED");
                return;
            }
        }    
    }
}

struct Point
{
    x: f64,
    y: f64
}

struct Line
{
    start: Point,
    end: Point
}

impl Line
{
    fn len(&self) ->f64 // first arg has to be &self
    {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx * dx + dy * dy).sqrt()
    }
}

fn structures()
{
    let p = Point{x: 3.0, y: 4.0};
    println!("point p is at ({}, {})", p.x, p.y);

    let p2 = Point{x: 5.0, y: 10.0};

    let myline = Line{start: p, end: p2};
}

enum Color
{
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),   // tuple definition
    Cmyk{cyan: u8, magenta: u8, yellow: u8, black: u8}  // struct definition
}

fn enums()
{
    let c: Color = Color::Cmyk{cyan: 0, magenta: 128, yellow: 0, black: 255};

    match c
    {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0) | 
            Color::Cmyk{cyan: _, magenta: _, yellow: _, black: 255} => println!("black"),
            // can acheive the same thing above via: Color::Cmyk{black:255, ..}
        Color::RgbColor(r, g, b) => println!("rgb({}, {}, {})", r, g, b),
        _ => () // do nothing
    }
}

union IntOrFloat    // 32 bits (either i32 or f32)
{
    i: i32,
    f: f32
}

fn unions()
{
    let mut iof = IntOrFloat{i: 123};
    iof.i = 234;

    let value = unsafe {iof.i}; // Needs unsafe block because user will assume risk of retreiving data from union (dont know if data is in iof.i or iof.f)
    println!("iof.i = {}", value);

    process_value(IntOrFloat{i: 5});
}

fn process_value(iof: IntOrFloat)
{
    unsafe {
        match iof {
            IntOrFloat{i: 42} => {
                println!("meaning of life value");
            }

            IntOrFloat{f} => {
                println!("value = {}", f);
            }
        }
    }
}

fn options()
{
    let x = 3.0;
    let y = 0.0;

    // Option -> Some(v) | None
    let result = if y != 0.0 {Some(x/y)} else {None};

    match result {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("cannot divide by 0")
    }

    if let Some(z) = result {   // if the let call succeeds, perform body statements
        println!("result = {}", z)
    }

}

fn arrays()
{
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    let mut b = [1, 2, 3, 4, 5];

    println!("a has {} elements, first is {}", a.len(), a[0]);

    a[0] = 321;
    println!("a[0] = {}", a[0]);

    println!("{:?}", a);

    if a != [1, 2, 3, 4, 5] // comparisons need to be same length
    {
        println!("does not match");
    }

    let b = [1u8; 10];
    for i in 0..b.len()
    {
        println!("{}", b[i]);
    }

    println!("b took up {} bytes", mem::size_of_val(&b));

    let mtx: [[f32; 3]; 2] = 
    [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];
    println!("{:?}", mtx);

    for i in 0..mtx.len()
    {
        for j in 0..mtx[i].len()
        {
            if i == j
            {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
}

fn use_slice(slice: &mut [i32])
{
    println!("first elem = {}, len = {}", slice[0], slice.len());
    slice[0] = 4321;
}

fn slices()
{
    let mut data = [1, 2, 3, 4, 5];
    use_slice(&mut data[1..4]);
    println!("{:?}", data);
    use_slice(&mut data);
    println!("{:?}", data);
}

fn sum_and_product(x: i32, y: i32) -> (i32, i32)
{
    (x + y, x * y)  // tuple elements can be different types (arrays cannot)
}

fn tuples()
{
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);
    println!("sp = {:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    // destructuring: taking existing tuple elements and assigning them to variables
    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);

    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2);
    println!("{:?}", combined);
    println!("last elem = {}", (combined.1).1);

    let ((c, d), (e, f)) = combined;

    let foo = (true, 42.0, -1i8);
    println!("{:?}", foo);

    let meaning = (42,); // tuple of single element (requires comma)
    println!("{:?}", meaning);
}

struct GenericPoint<T, V>
{
    x: T,
    y: V
}

fn generics()
{
    let a: GenericPoint<u16, i32> = GenericPoint{x: 0, y: 0};
    let b = GenericPoint{y: 1.2, x: 3.4};   // order of members does not matter if specified

    println!("a = ({}, {}), b = ({}, {})", a.x, a.y, b.x, b.y);
}

fn vectors()
{
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);
 
    a.push(44);
    println!("a = {:?}", a);

    let idx: usize = 0; // index type is usize
    a[idx] = 321;   
    println!("a[0] = {}", a[0]);
    
    // Returns an option type
    match a.get(6)
    {
        Some(x) => println!("a[6] = {}", x),
        None => println!("error, no such element")
    }

    for x in &a 
    {
        println!("{}", x);
    }

    a.push(77);
    println!("a = {:?}", a);

    // Returns option
    let last_elem = a.pop();
    println!("last elem is {:?}, a = {:?}", last_elem, a);

    while let Some(x) = a.pop()
    {
        println!("{}", x);
    }
}

fn hash_map()
{
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    println!("a square has {} sides", shapes["square".into()]);

    for (key, value) in & shapes
    {
        println!("{}: {}", key, value);
    }

    shapes.insert("square".into(), 5);

    println!("{:?}", shapes);

    shapes.entry("circle".into()).or_insert(1);
    
    println!("{:?}", shapes);
    // New scope
    {
        let actual = shapes.entry("circle".into()).or_insert(2);
        *actual = 0;
    }
    println!("{:?}", shapes);
}

fn hash_set()
{
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    println!("{:?}", greeks);

    greeks.insert("delta");
    println!("{:?}", greeks);

    let added_vega = greeks.insert("vega");
    if added_vega
    {
        println!("we added vega! hooray");
    }

    if !greeks.contains("kappa")
    {
        println!("we don't have kappa");
    }

    let removed = greeks.remove("delta");
    if removed
    {
        println!("we removed delta");
    }
    println!("{:?}", greeks);

    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    // checking subsets
    println!("Is {:?} a subset of {:?}? {}", _2_8, _1_10, _2_8.is_subset(&_1_10));

    // checking disjoint (not sharing any elements)
    println!("Is {:?} disjoint from {:?}? {}", _1_5, _6_10, _1_5.is_disjoint(&_6_10));
    
    // union
    println!("union of {:?} and {:?} is {:?}", _2_8, _6_10, _2_8.union(&_6_10));

    // intersection
    println!("intersection of {:?} and {:?} is {:?}", _2_8, _6_10, _2_8.intersection(&_6_10));

    // difference
    println!("difference of {:?} and {:?} is {:?}", _2_8, _6_10, _2_8.difference(&_6_10));

    // symmetric_difference
    println!("symmetric_difference of {:?} and {:?} is {:?}", _2_8, _6_10, _2_8.symmetric_difference(&_6_10));
}

fn iterators()
{
    let mut vec = vec![3, 2, 1];

    for x in &vec   // borrow vector via reference so that it can be used after as well
    {
        println!("{}", x);
    }

    for x in vec.iter_mut() // mutable iterator (non mutable one: .iter())
    {
        println!("{}", *x);
        *x += 2;
        println!("{}", x);  // this is the same, rust will be able to tell what you want to do
    }

    println!("{:?}", vec);

    for x in vec.iter().rev()
    {
        println!("in reverse {}", x);
    }

    let mut vec2 = vec![1, 2, 3];
    // let it = vec.into_iter();   // moves the vector (cannot use vector afterwards)
    vec2.extend(vec);   // extend will use into_iter()
    println!("vec2: {:?}", vec2);

}

fn strings()
{
    // &str = string slice (not super flexible)
    let s: &'static str = "hello there!";

    for c in s.chars().rev()
    {
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0)
    {
        println!("first letter is {}", first_char);
    }

    // String = heap allocated construct (more flexible) (essentially vector of chars)
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8)
    {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    println!("{}", letters);

    // &str <-> String
    let u: &str = &letters;
    let abc1 = String::from("hello world");
    let abc2 = "hello world".to_string();

    // concatenation
    // String + str
    let z = letters + "abc";

    // String + String;
    let y = abc2 + &abc1; // adding & in front of String will turn it to &str
}

fn string_formatting()
{
    let name = "Philip";
    let greeting = format!("hi, i'm {}, nice to meet you", name);
    println!("{}", greeting);

    let hello = "hello";
    let rust = "rust";
    let hello_rust = format!("{}, {}!", hello, rust);
    println!("{}", hello_rust);

    let run = "run";
    let forest = "forest";
    let rfr = format!("{}, {}, {}!", run, forest, run);
    println!("{}", rfr);
    let rfr2 = format!("{0}, {1}, {0}!", run, forest);
    println!("{}", rfr2);

    let info = format!("the name's {last}. {first} {last}", first = "james", last = "bond");
    println!("{}", info);

    let mixed = format!("{1} {} {0} {} {data}", "alpha", "beta", data = "delta");
    println!("{}", mixed);

}

fn number_guessing_game()
{
    let number: i64 = rand::thread_rng().gen_range(1..101); // upper bound is exclusive
    loop 
    {
        println!("Enter your guess: ");

        let mut buffer = String::new();

        match stdin().read_line(&mut buffer)
        {
            Ok(_) => {
                let parsed = buffer.trim_end().parse::<i64>();
                match parsed 
                {
                    Ok(guess) => {
                        if guess < 1 || guess > 100
                        {
                            println!("Your guess is our of range");
                        }
                        else if guess < number
                        {
                            println!("Your guess is too low");
                        }
                        else if guess > number
                        {
                            println!("Your guess is too high");
                        }
                        else 
                        {
                            println!("Correct!!!");
                            return;
                        }  
                    }
                    Err(e) => {
                        println!("Could not read your input. {}. Try again!", e);
                    }
                }
            },
            Err(_) => continue
        }
    }
}

fn print_value(x: i32)
{
    println!("value = {}", x);
}

fn increase(x: &mut i32)
{
    *x += 1;
}

fn product(x: i32, y: i32) -> i32   // define the return type
{
    x * y // statement without ; will return it
}

fn functions()
{
    print_value(33);

    let mut z = 1;
    increase(&mut z);
    println!("z = {}", z);

    let a = 2;
    let b = 5;
    let p = product(a, b);
    println!("{} * {} = {}", a, b, p);
}

fn methods()
{
    let p = Point{x: 3.0, y: 4.0};
    let p2 = Point{x: 5.0, y: 10.0};
    let myline = Line{start: p, end: p2};

    println!("length = {}", myline.len());
}

fn say_hello()
{
    println!("hello");
}

fn closures()
{
    let sh = say_hello; // storing function in variable
    sh();

    let plus_one = |x: i32| -> i32 {x + 1}; // inline definition of function (closure)
    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    let two = 2;
    let add = |x, y|  // don't have to explicity state arg or return type
    {
        let mut z = x;
        z += y;
        z
    };
    println!("{} + {} = {}", 3, 2, add(3, 2));

    let plus_three = |x: &mut i32| 
    {
        *x += 3;
    };

    let mut f = 12;
    plus_three(&mut f);
    println!("f = {}", f);
}

fn is_even(x: u32) -> bool
{
    x % 2 == 0
}

fn greater_than(limit: u32) -> impl Fn(u32) -> bool
{
    move |y| y > limit
}

fn higher_order_functions()
{
    // sum of all even squares < 500
    let limit = 500;
    let mut sum = 0;

    let above_limit = greater_than(limit);

    for i in 0.. 
    {
        let isq = i*i;

        if above_limit(isq)
        {
            break;
        }
        else if is_even(isq)
        {
            sum += isq;
        }
    }

    println!("sum = {}", sum);

    let sum2 = (0..)
        .map(|x| x * x)
        .take_while(|&x| x < limit)
        .filter(|x| is_even(*x))
        .fold(0, |sum, x| sum + x); // sum is the accumulator (first arg) with initialize value 0
    
    println!("hof sum = {}", sum2);
}

trait Animal
{
    // Factory function
    // fn create(name: &'static str) -> Self; // static fn since it doesnt take in &self

    fn name(&self) -> &'static str;

    fn talk(&self)
    {
        println!("{} cannot talk", self.name());
    }
}

struct Human
{
    name: &'static str
}

impl Animal for Human
{
    // fn create(name: &'static str) -> Human
    // {
    //     Human{name: name}
    // }

    fn name(&self) -> &'static str
    {
        self.name
    }

    fn talk(&self)
    {
        println!("{} says hello!", self.name);
    }
}

struct Cat
{
    name: &'static str
}

impl Animal for Cat
{
    // fn create(name: &'static str) -> Cat
    // {
    //     Cat{name: name}
    // }

    fn name(&self) -> &'static str
    {
        self.name
    }

    fn talk(&self)
    {
        println!("{} says meow!", self.name);
    }
}

trait Summable<T>
{
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32>
{
    fn sum(&self) -> i32
    {
        let mut result: i32 = 0;
        for x in self
        {
            result += x;
        }
        result
    }
}

fn traits()
{
    // let h = Human{name: "John"};
    // let h = Human::create("John");
    // let h: Human = Animal::create("John");
    // h.talk();

    // let c = Cat{name: "Misty"};
    // c.talk();

    // let a  = vec![1, 2, 3];
    // println!("sum = {}", a.sum());

}

#[derive(Debug)]
struct Circle 
{
    radius: f64,
}
  
#[derive(Debug)]
struct Square 
{
    side: f64,
}
  
trait Shape 
{
    fn area(&self) -> f64;
}
  
impl Shape for Square 
{
    fn area(&self) -> f64 
    {
        self.side * self.side
    }
}
  
impl Shape for Circle 
{
    fn area(&self) -> f64 
    {
        self.radius * self.radius * std::f64::consts::PI
    }
}

// fn print_info(shape: impl Shape + Debug)     // input arg has to impl both Shape and Debug traits
// fn print_info<T: Shape + Debug>(shape: T)    // can be useful for cases where multiple arguments need to have those traits
fn print_info<T>(shape: T) 
    where T: Shape + Debug
{
    println!("{:?}", shape);
    println!("The area is {}", shape.area());
}

fn trait_parameters()
{
    let c = Circle{radius: 2.0};
    print_info(c);
}

struct Person
{
    name: String
}

impl Person
{
    // fn new(name: &str) -> Person
    // {
    //     Person{name: name.to_string()}
    // }

    // fn new<S: Into<String>>(name: S) -> Person   // takes types that can be converted to String
    fn new<S>(name: S) -> Person 
        where S: Into<String>
    {
        Person{name: name.into()}  // Convert to appropriate type
    } 

    fn get_ref_name(&self) -> &String   // lifetime ellision here. same lifetime is applied: fn get_ref_name<'a>(&'a self) -> &'a String
    {
        &self.name
    }

    fn greet(&self)
    {
        println!("Hi, my name is {}", self.name);
    }
}

fn into()
{
    let john = Person::new("John");

    let name: String = "Jane".to_string();
    let jane = Person::new(name);
}

struct Creature
{
    name: String
}

impl Creature
{
    fn new(name: &str) -> Creature
    {
        println!("{} enters the game", name);
        Creature{name: name.into()}
    }
}

impl Drop for Creature
{
    fn drop(&mut self)
    {
        println!("{} is dead", self.name);
    }
}

fn drop()
{
    let mut moved_creature;
    {
        let goblin = Creature::new("Jeff");
        println!("game proceeds");
        moved_creature = goblin;
        println!("end of scope");
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
struct Complex<T>
{
    re: T,
    im: T
}

impl<T> Complex<T>
{
    fn new(re: T, im: T) -> Complex<T>
    {
        Complex::<T>{re, im}
    }
}

impl<T> Add for Complex<T>
    where T: Add<Output = T>
{
    type Output = Complex<T>;

    // a + b
    fn add(self, rhs: Self) -> Self::Output
    {
        Complex::<T>{re: self.re + rhs.re, im: self.im + rhs.im}
    }
}

impl<T> AddAssign for Complex<T>
    where T: AddAssign<T>
{
    fn add_assign(&mut self, rhs: Self)
    {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T> Neg for Complex<T>
    where T: Neg<Output = T>
{
    type Output = Complex<T>;

    fn neg(self) -> Self::Output
    {
        Complex{re: -self.re, im: -self.im}
    }
}

// impl<T> PartialEq for Complex<T>
//     where T: PartialEq
// {
//     fn eq(&self, rhs: &Self) -> bool
//     {
//         self.re == rhs.re && self.im == rhs.im
//     }
// }

fn operator_overloading()
{
    let mut a = Complex::new(1, 2);
    let mut b = Complex::new(3, 4);

    // Add
    // println!("a + b = {:?}", a + b);

    // AddAssign
    // a += b;
    // println!("a += b. a = {:?}", a);

    // Neg
    // println!("-a = {:?}", -a);

    // Partial Eq
    println!("a == b: {}", a == b);
    println!("a != b: {}", a != b);
}

trait Printable
{
    fn format(&self) -> String;
}

impl Printable for i32
{
    fn format(&self) -> String
    {
        format!("i32: {}", *self)
    }
}

impl Printable for String
{
    fn format(&self) -> String
    {
        format!("string: {}", *self)
    }
}

// Static Dispatch
fn print_it<T>(z: T)    // monomorphisation: impl of each type that this is called on will be generated for that specific type (happens at compile time)
    where T: Printable
{
    println!("{}", z.format());
}

fn static_dispatch()
{
    let a = 123;
    let b = "hello".to_string();

    // println!("{}", a.format());
    // println!("{}", b.format());

    print_it(a);
    print_it(b);
}

// Dynamic Dispatch: determination for which function (String's vs i32's format()) to call happens at runtime
fn print_it_too(z: &dyn Printable)  // Type erasure happens here since it will just be seen as a Printable instead of String or i32
{
    println!("{}", z.format());
}

fn dynamic_dispatch()
{
    let a = 123;
    let b = "hello".to_string();

    print_it_too(&a);
    print_it_too(&b);

    let shapes: [&dyn Shape; 4] = [
        &Circle{radius: 1.0},
        &Square{side: 3.0},
        &Circle{radius: 2.0},
        &Square{side: 4.0}
    ];

    for (i, shape) in shapes.iter().enumerate()
    {
        println!("Shape #{} has area {}", i, shape.area())
    }
}

enum Creatures
{
    Human(Human),
    Cat(Cat)
}

fn vectors_of_different_objects()
{
    let mut creatures = Vec::new();
    
    creatures.push(Creatures::Human(Human{name: "John"}));
    creatures.push(Creatures::Cat(Cat{name: "Fluffy"}));

    for c in creatures
    {
        match c
        {
            Creatures::Human(h) => h.talk(),
            Creatures::Cat(c) => c.talk()
        }
    }




    let mut animals: Vec<Box<dyn Animal>> = Vec::new();
    animals.push(Box::new(Human{name: "John"}));
    animals.push(Box::new(Cat{name: "Fluffy"}));

    for a in animals.iter()
    {
        a.talk();
    }
}

fn borrowing()
{
    let print_vector = |x: &Vec<i32>|
    {
        println!("{:?}", x);
    };

    let v = vec![1, 2, 3];
    print_vector(&v);
    println!("{:?}", v);

    let mut a = vec![4, 5, 6];
    let b = &mut a;
    b.push(42);
    // println!("a = {:?}", a);
    println!("b = {:?}", b);
}

struct Company<'z>  // specifying lifetime of Company (lifetime can be anything)
{
    name: String,
    ceo: &'z Person // specifying lifetime of ref to Person is the same as lifetime of Company (prevent invalid ref)
}

struct Person2<'a>
{
    name: &'a str
}

impl<'a> Person2<'a>
{
    fn talk(&self)
    {
        println!("Hi, my name is {}", self.name);
    }
}

fn lifetime()
{
    let boss = Person{name: String::from("Elon Musk")};
    let tesla = Company{name: String::from("Tesla"), ceo: &boss};

    let person = Person2{name: "Philip"};
    person.talk();
}

struct Person3
{
    name: Rc<String>
}

impl Person3
{
    fn new(name: Rc<String>) -> Person
    {
        Person{name: name.to_string()}
    }

    fn greet(&self)
    {
        println!("Hi, my name is {}", self.name);
    }
}

fn reference_counted_variables()
{
    let name = Rc::new("John".to_string());
    println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
    
    {
        let person = Person3::new(name.clone());
        println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
        person.greet();
    }

    println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
    println!("Name = {}", name);
}

struct Person4
{
    name: Arc<String>,
    state: Arc<Mutex<String>>
}

impl Person4
{
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Person4
    {
        Person4{name: name, state: state}
    }

    fn greet(&self)
    {
        let mut state = self.state.lock().unwrap();
        state.clear();
        state.push_str("excited");

        println!("Hi, my name is {} and I am {}", self.name, state.as_str());
    }
}

fn atomic_rc_variables()
{
    let name = Arc::new("John".to_string());
    let state = Arc::new(Mutex::new("bored".to_string()));
    let person = Person4::new(name.clone(), state.clone());

    let t = thread::spawn(move || {
        person.greet();
    });

    println!("Name = {}, state = {}", name, state.lock().unwrap().as_str());

    t.join().unwrap();
}

// Circular references with Rc and RefCell
// struct Student
// {
//     name: String,
//     courses: Vec<Rc<RefCell<Course>>>
// }

// impl Student
// {
//     fn new(name: &str) -> Student
//     {
//         Student{name: name.into(), courses: Vec::new()}
//     }
// }

// struct Course
// {
//     name: String,
//     students: Vec<Rc<RefCell<Student>>>
// }

// impl Course
// {
//     fn new(name: &str) -> Course
//     {
//         Course{name: name.into(), students: Vec::new()}
//     }

//     fn add_student(
//         course: Rc<RefCell<Course>>,
//         student: Rc<RefCell<Student>>
//     )
//     {
//         student.borrow_mut().courses.push(course.clone());
//         course.borrow_mut().students.push(student);
//     }
// }

// fn circular_references()
// {
//     let john = Rc::new(RefCell::new(Student::new("John")));
//     let jane = Rc::new(RefCell::new(Student::new("Jane")));
//     let course = Course::new("Rust Course");
//     let magic_course = Rc::new(RefCell::new(course));

//     Course::add_student(magic_course.clone(), john);
//     Course::add_student(magic_course, jane);
// }

// New struct (Enrollment) to contain (and remove circular) dependency
struct Student
{
    name: String,
}

impl Student
{
    fn courses(&self, platform: Platform) -> Vec<String>
    {
        platform.enrollments.iter()
            .filter(|&e| e.student.name == self.name)
            .map(|e| e.course.name.clone())
            .collect()
    }
}

struct Course
{
    name: String,
}

struct Enrollment<'a>
{
    student: &'a Student,
    course: &'a Course
}

impl<'a> Enrollment<'a>
{
    fn new(student: &'a Student, course: &'a Course) -> Enrollment<'a>
    {
        Enrollment{student, course}
    }
}

struct Platform<'a> 
{
    enrollments: Vec<Enrollment<'a>>
}

impl<'a> Platform<'a>
{
    fn new() -> Platform<'a>
    {
        Platform{enrollments: Vec::new()}
    }

    fn enroll(&mut self, student: &'a Student, course: &'a Course)
    {
        self.enrollments.push(Enrollment::new(student, course))
    }
}

fn circular_references()
{
    let john = Student{name: "John".into()};
    let course = Course{name: "Rust Course".into()};
    let mut p = Platform::new();
    p.enroll(&john, &course);

    for c in john.courses(p)
    {
        println!("{} is taking {}", john.name, c);
    }
}

fn threads()
{
    let handle = thread::spawn(|| {
        for _ in 1..10 
        {
            print!("+");
            thread::sleep(time::Duration::from_millis(500));
        }
    });

    for _ in 1..10 
    {
        print!("-");
        thread::sleep(time::Duration::from_millis(300));
    }

    handle.join();
}

fn building_mod_and_crates()
{
    println!("English: {}, {}", 
        phrases::greetings::english::hello(),
        phrases::greetings::english::goodbye()
    );

    println!("French: {}, {}", 
        phrases::greetings::french::hello(),
        phrases::greetings::french::goodbye()
    );
}

fn main()
{
    building_mod_and_crates();
}